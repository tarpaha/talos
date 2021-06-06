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
    let tetrominoes = tetris::tetrominoes();
    let field = Field::new(4, 4);
    println!("{}", field);
}