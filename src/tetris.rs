use std::fmt;

struct TetrominoVariant {
    width: u8,
    height: u8,
    cells: Vec<(u8, u8)>
}

impl TetrominoVariant {
    fn from_string_lines(lines: Vec<&str>) -> Self {
        let width = lines[0].len() as u8;
        let height = lines.len() as u8;
        let mut cells = vec![];
        let mut y = 0;
        for line in lines {
            let mut x = 0;
            for ch in line.chars() {
                if ch == '#' {
                    cells.push((x, y));
                }
                x += 1;
            }
            y += 1;
        }
        TetrominoVariant { width, height, cells }
    }
}

impl fmt::Display for TetrominoVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "({}, {})", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let ch = if self.cells.contains(&(x, y)) { '#' } else { ' ' };
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
    variants: Vec<TetrominoVariant>
}

impl Tetromino {
    fn from_variants(variants: Vec<TetrominoVariant>) -> Self {
        Tetromino { variants }
    }
}

impl fmt::Display for Tetromino {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    Tetromino::from_variants(vec! [
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
    Tetromino::from_variants(vec! [
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
    Tetromino::from_variants(vec! [
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
    Tetromino::from_variants(vec! [
        TetrominoVariant::from_string_lines(vec![
            "##",
            "##"
        ])
    ])
}

#[allow(non_snake_case)]
fn S() -> Tetromino {
    Tetromino::from_variants(vec! [
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
    Tetromino::from_variants(vec! [
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
    Tetromino::from_variants(vec! [
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

pub fn tetrominoes() -> Vec<Tetromino> {
    vec![I(), J(), L(), O(), S(), T(), Z()]
}