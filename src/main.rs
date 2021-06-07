use std::fmt;
mod tetris;

struct Field {
    width: u8,
    height: u8,
    cells: Vec<u8>,
    filled: u8,
    current: u8,
    operations: u64
}

impl Field {
    fn new(width: u8, height: u8) -> Self {
        Field { width, height, cells: vec![0; (width * height) as usize], current: 1, filled: 0, operations: 0 }
    }
    
    fn add(&mut self, tetromino_variant: &tetris::TetrominoVariant, x: u8, y: u8) {
        self.apply_tetromino_with_value(tetromino_variant, x, y, self.current);
        self.current += 1;
        self.filled += 4;
        self.operations += 1;
    }

    fn remove(&mut self, tetromino_variant: &tetris::TetrominoVariant, x: u8, y: u8) {
        self.apply_tetromino_with_value(tetromino_variant, x, y, 0);
        self.current -= 1;
        self.filled -= 4;
    }
    
    fn apply_tetromino_with_value(&mut self, tetromino_variant: &tetris::TetrominoVariant, x: u8, y: u8, value: u8) {
        for block in &tetromino_variant.blocks {
            let p = (x + block.x) + self.width * (y + block.y);
            self.cells[p as usize] = value;
        }
    }
    
    fn is_full(&self) -> bool {
        self.filled == self.width * self.height 
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

fn can_be_placed(field: &Field, tetromino_variant: &tetris::TetrominoVariant, x: u8, y: u8) -> bool {
    for block in &tetromino_variant.blocks {
        let p = (x + block.x) + field.width * (y + block.y);
        if field.cells[p as usize] > 0 {
            return false;
        }
    }
    true
}

fn solve_impl(field: &mut Field, mut tetrominoes: Vec<&tetris::Tetromino>) {
    while let Some(tetromino) = tetrominoes.pop() {
        for variant in &tetromino.variants {
            for y in 0..(field.height - variant.height + 1) {
                for x in 0..(field.width - variant.width + 1) {
                    if can_be_placed(&field, variant, x, y) {
                        field.add(variant, x, y);
                        solve_impl(field, tetrominoes.clone());
                        if field.is_full() {
                            return;
                        }
                        field.remove(variant, x, y);
                    }
                }
            }
        }
    }
}

fn solve(field_width: u8, field_height: u8, tetrominoes_string: &str) {
    let mut field = Field::new(field_width, field_height);
    let tetrominoes = tetris::Tetrominoes::new();
    solve_impl(&mut field, tetrominoes.collection_from_string(tetrominoes_string));
    println!("{}", field);
    println!("Solved in {} operations", field.operations);
}

fn main() {
    //solve(4, 4, "L1J1O1I1");
    //solve(4, 6, "I1O1T2L1Z1");
    //solve(6, 6, "O1S4L4");
    //solve(6, 8, "O2T4J1L1S2Z2");
    solve(5, 8, "Z2S1T2I2L2O1");
}