# Rust-HK-Maze-Generator
Maze generator using the Prim's algorithm in Rust

The size of the maze is calculated using the empty blocks in the rows and column.
So a size of 3 will draw a maze like this:
```
   1 2 3
  XXXXXXX
1 S X   X
  X X XXX
2 X     X
  X X X X
3 X X X F
  XXXXXXX
```

The default size is 7 in height and width but they can be modified independently with the "--height *int*" and "--width *int*" options.
The minimum is 3 for both.

This version is under optimized so don't expect good performance.