use std::{env, fs};


pub fn read_file(filepath: &str) -> String{
    println!("{}", env::current_dir().expect("failed").to_str().unwrap());
    println!("{}", filepath.clone());
    let content = fs::read_to_string(filepath).expect("something went to shit");

    return content;
}
