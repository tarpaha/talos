use std::fmt;
use std::str;
use std::error::Error;

pub struct Block {
    pub x: u8,
    pub y: u8
}

pub struct TetrominoVariant {
    pub width: u8,
    pub height: u8,
    pub blocks: Vec<Block>
}

impl TetrominoVariant {
    fn from_string_lines(lines: Vec<&str>) -> Self {
        let width = lines[0].len() as u8;
        let height = lines.len() as u8;
        let mut blocks = vec![];
        let mut y = 0;
        for line in lines {
            let mut x = 0;
            for ch in line.chars() {
                if ch == '#' {
                    blocks.push(Block { x, y });
                }
                x += 1;
            }
            y += 1;
        }
        TetrominoVariant { width, height, blocks }
    }
}

impl fmt::Display for TetrominoVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "({}, {})", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let ch = if self.blocks.iter().any(|block| block.x == x && block.y == y) { '#' } else { ' ' };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

pub struct Tetromino {
    name: String,
    pub variants: Vec<TetrominoVariant>
}

impl Tetromino {
    fn from_variants(name: &str, variants: Vec<TetrominoVariant>) -> Self {
        Tetromino { name: name.to_string(), variants }
    }
}

impl fmt::Display for Tetromino {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\"{}\"", self.name)?;
        for variant in &self.variants {
            write!(f, "{}", variant)?;
        }
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[allow(non_snake_case)]
fn I() -> Tetromino {
    Tetromino::from_variants("I", vec! [
        TetrominoVariant::from_string_lines(vec![
            "#",
            "#",
            "#",
            "#"
        ]),
        TetrominoVariant::from_string_lines(vec![
            "####"
        ])
    ])
}

#[allow(non_snake_case)]
fn J() -> Tetromino {
    Tetromino::from_variants("J", vec! [
        TetrominoVariant::from_string_lines(vec![
            " #",
            " #",
            "##"
        ]),
        TetrominoVariant::from_string_lines(vec![
            "#  ",
            "###"
        ]),
        TetrominoVariant::from_string_lines(vec![
            "##",
            "# ",
            "# "
        ]),
        TetrominoVariant::from_string_lines(vec![
            "###",
            "  #"
        ]),
    ])
}

#[allow(non_snake_case)]
fn L() -> Tetromino {
    Tetromino::from_variants("L", vec! [
        TetrominoVariant::from_string_lines(vec![
            "# ",
            "# ",
            "##"
        ]),
        TetrominoVariant::from_string_lines(vec![
            "###",
            "#  "
        ]),
        TetrominoVariant::from_string_lines(vec![
            "##",
            " #",
            " #"
        ]),
        TetrominoVariant::from_string_lines(vec![
            "  #",
            "###"
        ]),
    ])
}

#[allow(non_snake_case)]
fn O() -> Tetromino {
    Tetromino::from_variants("O", vec! [
        TetrominoVariant::from_string_lines(vec![
            "##",
            "##"
        ])
    ])
}

#[allow(non_snake_case)]
fn S() -> Tetromino {
    Tetromino::from_variants("S", vec! [
        TetrominoVariant::from_string_lines(vec![
            " ##",
            "## "
        ]),
        TetrominoVariant::from_string_lines(vec![
            "# ",
            "##",
            " #"
        ])
    ])
}

#[allow(non_snake_case)]
fn T() -> Tetromino {
    Tetromino::from_variants("T", vec! [
        TetrominoVariant::from_string_lines(vec![
            " # ",
            "###"
        ]),
        TetrominoVariant::from_string_lines(vec![
            "# ",
            "##",
            "# "
        ]),
        TetrominoVariant::from_string_lines(vec![
            "###",
            " # "
        ]),
        TetrominoVariant::from_string_lines(vec![
            " #",
            "##",
            " #"
        ])
    ])
}

#[allow(non_snake_case)]
fn Z() -> Tetromino {
    Tetromino::from_variants("Z", vec! [
        TetrominoVariant::from_string_lines(vec![
            "## ",
            " ##"
        ]),
        TetrominoVariant::from_string_lines(vec![
            " #",
            "##",
            "# "
        ])
    ])
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum TetrominoError {
    InvalidTetromino(char),
    InvalidCount(char),
}

impl fmt::Display for TetrominoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TetrominoError::InvalidTetromino(c) => write!(f, "Invalid tetromino character: {}", c),
            TetrominoError::InvalidCount(c) => write!(f, "Invalid count character: {}", c),
        }
    }
}

impl Error for TetrominoError {}

pub struct Tetrominoes {
    tetrominoes: Vec<Tetromino>
}

impl Tetrominoes {
    pub fn new() -> Self {
        Tetrominoes { tetrominoes: vec![I(), J(), L(), O(), S(), T(), Z()] }
    }
    
    pub fn get(&self, name: &str) -> Option<&Tetromino> {
        self.tetrominoes.iter().find(|t| t.name == name)
    }
    
    pub fn collection_from_string(&self, s: &str) -> Result<Vec<&Tetromino>, Box<dyn Error>> {
        let mut tetrominoes = vec![];
        let chars: Vec<char> = s.chars().collect();
        for i in (0..chars.len()).step_by(2) {
            let tetromino = self.get(&chars[i].to_string())
                .ok_or(TetrominoError::InvalidTetromino(chars[i]))?;
            let count = char::to_digit(chars[i + 1], 10)
                .ok_or(TetrominoError::InvalidCount(chars[i + 1]))?;
            for _ in 0..count {
                tetrominoes.push(tetromino);
            }
        }
        Ok(tetrominoes)
    }
}