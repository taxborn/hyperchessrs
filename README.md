# A [4DChess](https://esolangs.org/wiki/4DChess) interpreter in [Rust](https://www.rust-lang.org/).

As described from it's esolangs wiki page:

> 4DChess is an esoteric programming language designed by User: Zemeckis on 18th November, 2019. It is directly inspired from Brainf\*\*\*. Instead of brain\*\*\*'s usual one-dimensional memory cell array, 4DChess uses a four-dimensional hypercube-like memory cell array of 8 cells per dimension. 

## What I wanted to do
Here is what I wanted to achieve:

- [ ] Learn interpreters and how they work
- [ ] Use Rust to create an interpreter
- [ ] Create an interpreter for an esoteric language
- [ ] Create sample programs in esoteric languages

## How to compile and run:

**Note:** You need to have [Rust installed](https://rustup.rs) on your system to build. I will include released binaries later.

```
$ git clone https://github.com/taxborn/hyperchessrs && cd hyperchessrs

$ cargo build # You can use --release flag here to generate optimized binary
```

Now that the binary has been compiled, you can access it by running:

```$ ./target/debug/hyperchessrs <name of 4D Chess file>.4dc # If --release was used, the path is ./target/release/hyperchessrs```

## What I want to add in the future:
- [ ] Optimize it
- [ ] Add more sample programs
- [ ] Clean up the runner file
- [ ] Add to the [esolangs](https://esolangs.org/wiki/4DChess) page

## References used:
- [Interpreter Wiki](https://en.wikipedia.org/wiki/Interpreter_(computing))
- [4DChess esolangs page](https://esolangs.org/wiki/4DChess)
- 