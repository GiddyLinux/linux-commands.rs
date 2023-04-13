use std::fs::File;
use std::io::Read;
use std::fmt;


#[derive(Default)]
pub struct file {
    bytes: Vec<u8>,
}


impl file 
{
    pub fn get_bytes(mut orginal_file: File) -> file
    {
        let mut buffer: Vec<u8> = Vec::new(); 
        orginal_file.read_to_end(&mut buffer);
        return file { bytes: buffer};
    }
    fn print_bytes(file: file)
    {
        let cat_file = File::open("src\\test.txt").unwrap();
        let file = file::get_bytes(cat_file);
        
        print!("{}",file);
    }
}
impl fmt::Display for file
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",String::from_utf8(self.bytes.to_vec()).unwrap())
    }
}
