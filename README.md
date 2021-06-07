# talos
Brute force solver for "The Talos Principle" game puzzles.
## Usage
Need to provide field size and list of [tetrominoes](https://tetris.fandom.com/wiki/Tetromino).

In example for this game puzzle:

![screenshot](img/game.jpg)

Use this command line arguments: `6 6 O2T2L2J1I1S1`

In result each tetromino is shown with its own digit:

![console](img/console.png)

Because of brute force algorithm type it can work very slow for big fields (like 8x6).
But this can be avoided by filling part of the field. Like on picture below where 8x6 field was reduced to 6x6 and solved. 
![console](img/big.jpg)