# Compile Rust

    rustc main.rs

# Track execution


    time python main-pandas.py
    time python main-standar.py
    time ./main

# Benchmarking
    hyperfine ./rust "python main-pandas.py" "python main-standard.py" --runs 5 --warmup 5