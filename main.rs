use std::fs;

fn main() {
    println!("Rust");
    let file_name = "large.csv";
    let _contents = fs::read_to_string(&file_name).unwrap();
    // println!("{}", _contents);
}
