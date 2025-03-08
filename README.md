# Rust Directory for PS
There are several problems to use Rust in PS. One is that there must be one executive file for every solved problem, which 
is devastating if making a Cargo dir for every problem. Hence, we hereby notate a structure that is modularized.

## lib.rs
The `lib.rs` contains the modules inside `src`. Whenever a new module is made, it should be added in this file.

## clean.py
This cleans the `Cargo.toml` file, where the bin elements inside the `toml` file is sorted and removed if duplication occurred.

## run.sh
This shell file needs two arguments `mode`, `problem_number`.
```shell
match $mode {
    run => {Runs the target named prob_`problem_number`. If Cargo fails to find the target, it is automatically searched and added to the `Cargo.toml`, and then cleaned.}
    submit => {Makes the file prob_`problem_number`.rs into a submittable format and writes it in `temp.rs`.}
    test => {Parses the BOJ Website and receives the sample test datas. Then runs an internal test for the received test datas.}
}
```

## To Add Models
To write models that are desired to be used throughout the codebase, follow these steps.

1. Make `foo.rs` inside `/src/models/`.
2. Add `pub mod foo;` inside `/src/models/mod.rs`.
3. Add `pub mod foo;` inside `pub mod models{}` in the `/src/lib.rs` file.