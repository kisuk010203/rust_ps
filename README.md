# Rust Directory for PS
There are several problems to use Rust in PS. One is that there must be one executive file for every solved problem, which 
is devastating if making a Cargo dir for every problem. Hence, we hereby notate a structure that is modularized.

## lib.rs
The `lib.rs` contains the modules inside `src`. Whenever a new module is made, it should be added in this file.

## clean.py
This cleans the `Cargo.toml` file, where the bin elements inside the `toml` file is sorted and removed if duplication occurred.

## run.sh
This shell file gets input the `problem_number` and runs the target named prob_`problem_number`. If Cargo failed to find the target, it is automatically searched and added to the `Cargo.toml`, and then cleaned. 
```Shell
./run.sh 231 # This is same as cargo run --bin prob_231.
```