#[derive(Debug)]
struct CubeSat {
    id : u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

impl Copy for CubeSat { }

impl Copy for StatusMessage { }

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        CubeSat { id : self.id }
    }
}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}

fn main() {

}