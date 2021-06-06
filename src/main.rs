use std::fmt;
mod tetris;

struct Field {
    width: u8,
    height: u8,
    cells: Vec<bool>
}

impl Field {
    fn new(width: u8, height: u8) -> Self {
        Field { width, height, cells: vec![false; (width * height) as usize] }
    }
    
    fn add(&mut self, tetromino_variant: &tetris::TetrominoVariant, x: u8, y: u8) {
        self.apply_tetromino_with_value(tetromino_variant, x, y, true);
    }

    fn remove(&mut self, tetromino_variant: &tetris::TetrominoVariant, x: u8, y: u8) {
        self.apply_tetromino_with_value(tetromino_variant, x, y, false);
    }
    
    fn apply_tetromino_with_value(&mut self, tetromino_variant: &tetris::TetrominoVariant, x: u8, y: u8, value: bool) {
        for block in &tetromino_variant.blocks {
            let p = (x + block.x) + self.width * (y + block.y);
            self.cells[p as usize] = value;
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut p = 0;
        for _ in 0..self.height {
            write!(f, "|")?;
            for _ in 0..self.width {
                write!(f, "{}", if self.cells[p] { '#' } else { ' ' })?; 
                p += 1;
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}

fn main() {
    let tetrominoes = tetris::Tetrominoes::new();
    let mut field = Field::new(4, 4);
    field.add(&tetrominoes.get("L").unwrap().variants[0], 0, 0);
    field.add(&tetrominoes.get("O").unwrap().variants[0], 2, 0);
    println!("{}", field);
    field.remove(&tetrominoes.get("O").unwrap().variants[0], 2, 0);
    println!("{}", field);
}