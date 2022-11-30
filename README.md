Template largely copied from [RikvanToor/aoc-rust-template](https://github.com/RikvanToor/aoc-rust-template), upgraded for `clap` v4.

This template can be forked/cloned from [beeb/aoc-rust](https://github.com/beeb/aoc-rust).

## Usage

First, find your session cookie. To do so, go to [https://adventofcode.com](adventofcode.com), log in, and press F12. In the "Application" tab, under "Storage > Cookies > https://adventofcode.com", copy the value of the "session" cookie. Paste it into a file named `.session` at the root of the project. You can now download input files. To do so, run the subcommand `get-input`: `cargo run -- get-input 1`. The `1` stands for day 1, meaning it will download the input of day 1 of Advent of Code. Alternatively, from december 1st to 25th, you can skip the day parameter, and the program will download today's input. You can also use `--all` instead of a day parameter to download all input files.

To run an implementation, use `cargo run -- run 1` to run day 1. Just like `get-input`, you can skip the day parameter to run today's program, or use `--all` to run all days.
