mod file;

mod utils;
use file::*;
fn main() {
    let file_name = std::env::args().nth(1).expect("File name not provided");
    let file = File::new(&file_name);

    println!("{}", file);
}
