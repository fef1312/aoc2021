# Advent of Code 2021

These are my personal solutions for the Advent of Code 2021 challenges.
Please note that these are definitely *not* the best ways of solving the problems from a software
architecture point of view, but that is not my intention.
This means you should not take this as sound design advice for your projects, especially since i'm
very new to Rust and want to play with the language to build some intuition for what works well and
what you should rather not do.
I'm trying to solve everything using interesting algorithms, bit bangers, and similar stuff that is
highly optimized to tackle just the very specific problem at hand, rather than to come up with a
general solution.

## Running

My input files are in `input` and should be fed into the program's stdin.
For example, to run the challenge for day 2, type

```
cat input/day02.txt | cargo run day02
```

Starting from day 3, the individual challenges are further subdivided into part 1 and 2.
The input stays the same, but the binary name is `dayXXpY`:

```
cat input/day03.txt | cargo run day03p1
```

## License

Everything is released under the 2-Clause BSD License, which can be found in the `LICENSE` file.
