mode=$1
problem_number=$2

case $mode in
    run)
        cargo +nightly fmt
        cargo run --bin prob_$problem_number

        if [ $? -ne 0 ]; then
            echo "No binary target named prob_$problem_number, so will try to add."

            file=$(find . -name "prob_$problem_number.rs")
            if [ -n "$file" ]; then
                echo "Found rust file named prob_$problem_number.rs at $file"
                echo "[[bin]]" >> Cargo.toml
                echo "name = \"prob_$problem_number\"" >> Cargo.toml
                echo "path = \"$file\"" >> Cargo.toml
                python clean.py
                echo "Finished adding binary target prob_$problem_number to Cargo.toml, run the prompt again".
            else 
                echo "No such rust file named prob_$problem_number.rs exists."
            fi
        fi;;
    submit)
        cargo run --bin rust_ps submit $(find . -name "prob_$problem_number.rs")
        ;;
    test)
        rm *.in
        rm *.out
        cargo run --bin rust_ps parse https://www.acmicpc.net/problem/$problem_number
        ;;
esac