mod tetris;

fn main() {
    let tetrominoes = tetris::tetrominoes();
    for tetromino in tetrominoes {
        println!("{}", tetromino);
    }
}