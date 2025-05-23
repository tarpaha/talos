use std::collections::HashSet;
use std::{fmt, thread};
use std::error::Error;
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use rand::seq::SliceRandom;

mod args;
mod tetris;
mod regions;

use args::parse_args;
use tetris::{TetrominoVariant, Tetromino, Tetrominoes};

#[derive(Clone)]
struct Field {
    width: u8,
    height: u8,
    cells: Vec<u8>,
    free: HashSet<(i32, i32)>,
    filled: u8,
    current: u8,
    operations: u64
}

impl Field {
    fn new(width: u8, height: u8) -> Self {
        let mut free = HashSet::new();
        for y in 0..height {
            for x in 0..width {
                free.insert((x as i32, y as i32));
            }
        }
        Field { width, height, cells: vec![0; (width * height) as usize], free, current: 1, filled: 0, operations: 0 }
    }
    
    fn add(&mut self, tetromino_variant: &TetrominoVariant, x: u8, y: u8) {
        self.apply_tetromino_with_value(tetromino_variant, x, y, self.current);
        self.current += 1;
        self.filled += 4;
        self.operations += 1;
    }

    fn remove(&mut self, tetromino_variant: &TetrominoVariant, x: u8, y: u8) {
        self.apply_tetromino_with_value(tetromino_variant, x, y, 0);
        self.current -= 1;
        self.filled -= 4;
    }
    
    fn apply_tetromino_with_value(&mut self, tetromino_variant: &TetrominoVariant, x: u8, y: u8, value: u8) {
        for block in &tetromino_variant.blocks {
            let p = (x + block.x) + self.width * (y + block.y);
            self.cells[p as usize] = value;
            if value > 0 {
                self.free.remove(&((x + block.x) as i32, (y + block.y) as i32));
            } else {
                self.free.insert(((x + block.x) as i32, (y + block.y) as i32));
            }
        }
    }
    
    fn is_full(&self) -> bool {
        self.filled == self.cells.len() as u8
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut p = 0;
        for _ in 0..self.height {
            write!(f, "|")?;
            for _ in 0..self.width {
                write!(f, "{}", if self.cells[p] > 0 { char::from_digit(self.cells[p] as u32, 16).unwrap() } else { ' ' })?; 
                p += 1;
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}

fn can_be_placed(field: &Field, tetromino_variant: &TetrominoVariant, x: u8, y: u8) -> bool {
    for block in &tetromino_variant.blocks {
        let p = (x + block.x) + field.width * (y + block.y);
        if field.cells[p as usize] > 0 {
            return false;
        }
    }
    true
}

// check if there is enough space to place tetrominoes,
// that means that there are no free regions with size that is not divisible by 4
fn have_enough_space(field: &Field) -> bool {
    let regions = regions::find_connected_region_sizes(
        field.width as i32,
        field.height as i32,
        &field.free);
    regions.iter().all(|&size| size % 4 == 0)
}

fn solve_impl(
    field: &mut Field,
    tetrominoes: &[&Tetromino],
    index: usize,
    done: &AtomicBool,
    tx: &mpsc::Sender<(String, u64)>
) -> bool {
    if done.load(Ordering::Relaxed) {
        return false;
    }
    if index >= tetrominoes.len() {
        return false;
    }
    let tetromino = tetrominoes[index];
    for variant in &tetromino.variants {
        for y in 0..(field.height - variant.height + 1) {
            for x in 0..(field.width - variant.width + 1) {
                if done.load(Ordering::Relaxed) {
                    return false;
                }
                // if tetromino do not intersect with already placed tetrominoes
                if can_be_placed(&field, variant, x, y) {
                    // then place it
                    field.add(variant, x, y);
                    // check is field is fully filled
                    if field.is_full() {
                        // solution found, try to set a completion flag
                        if done
                            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                            .is_ok()
                        {
                            // it was successful, then it means that this is the first solution
                            tx.send((format!("{}", field), field.operations)).unwrap();
                            return true;
                        }
                        else
                        {
                            // a solution was already found, stop here
                            return false;
                        }
                    }
                    // check if there is enough space to place the next tetromino
                    if !have_enough_space(&field) {
                        // no space, remove the last placed tetromino
                        field.remove(variant, x, y);
                        // and continue for the next coordinate
                        continue;
                    }
                    // field is not complete and have place for next tetromino,
                    // try to place it recursively
                    if solve_impl(field, tetrominoes, index + 1, done, tx) {
                        return true;
                    }
                    // have no solution for current (x, y),
                    // continue for the next coordinate
                    field.remove(variant, x, y);
                }
            }
        }
    }
    false
}


fn solve(
    field_width: u8,
    field_height: u8,
    tetrominoes_string: &str,
    threads_count: usize
) -> Result<Option<(String, u64)>, Box<dyn Error>> {
    let field = Field::new(field_width, field_height);
    let tetrominoes = Tetrominoes::new();
    let tetrominoes: Vec<&Tetromino> = tetrominoes.collection_from_string(tetrominoes_string)?;

    let (tx, rx) = mpsc::channel();
    let done = Arc::new(AtomicBool::new(false));

    // scope required to be sure that no one thread will outlive this function
    let result = thread::scope(|scope| {
        for _ in 0..threads_count {
            let mut field = field.clone();
            
            let mut tetrominoes = tetrominoes.clone();
            tetrominoes.shuffle(&mut rand::rng());

            let done = done.clone();
            let tx = tx.clone();

            scope.spawn(move || {
                solve_impl(&mut field, &tetrominoes, 0, &done, &tx)
            });
        }
        // tx was cloned in each thread, drop it here
        drop(tx);

        // rx.recv() can finish with two values:
        // Ok - solution was found by one of the threads, and it was sent to the channel
        // Err - no solution was found, all threads were ended, and so all tx were dropped
        rx.recv()
    });

    Ok(result.ok())
}

fn main() -> Result<(), Box<dyn Error>>{
    let (width, height, tetrominoes) = parse_args();
    let num_cpus = num_cpus::get();
    println!("Solving {}x{} field with {} threads", width, height, num_cpus);
    let now = Instant::now();
    match solve(width, height, &tetrominoes, num_cpus)? {
        Some((solution, operations)) => {
            let elapsed_millis = now.elapsed().as_millis();
            print!("{}", solution);
            print!("Solved in {} ms, {} operations", elapsed_millis, operations);
            if elapsed_millis > 0 {
                println!(", {} op/sec", operations as u128 * 1000 / elapsed_millis);
            } else {
                println!();
            }
        }
        None => println!("No solution found")
    }
    Ok(())
}