use std::fs::File;

mod cat;
fn main() {
    let cat_file = File::open("src\\test.txt").unwrap();
    let file = cat::file::get_bytes(cat_file);
    print!("{}", file)

}
