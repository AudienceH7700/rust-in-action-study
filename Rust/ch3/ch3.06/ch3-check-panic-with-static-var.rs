#![allow(unused_variables)]

static mut ERROR: i32 = 0;

#[derive(Debug)]
struct File {
    name : String,
    data : Vec<u8>,
}

impl File {
    fn new(name:&str) -> File {
        File {
            name : String::from(name),
            data : Vec::new(),
        }
    }
    
    fn new_with_data(
        name : &str,
        data : &Vec<u8>
        ) -> File {
            let mut f = File::new(name);
            f.data = data.clone();
            f
    }
    
    
}

fn read(f : &File,
    save_to : Vec<u8>,
) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();
    read_length
}

fn open(f : &File) -> bool {
    true
}

fn close(f : &File) -> bool {
    true
}

fn main() {
    let f = File::new("something.txt");

    let buffer: Vec<u8> = vec![];

    read(&f, buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while reading the file ")
        }
    }

    close(&f);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while reading the file ")
        }
    }
}