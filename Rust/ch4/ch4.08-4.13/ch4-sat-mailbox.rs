#[derive(Debug)]
struct CubeSat {
    id : u64,
    Mailbox : Mailbox,
}
#[derive(Debug)]
struct Mailbox {
    messages : Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message){
        to.Mailbox.messages.push(msg);
    }
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.Mailbox.messages.pop()
    }
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id : 0,
        Mailbox : Mailbox{messages : vec![]},
    };

    println!("t0 : {:?}", sat_a);

    base.send(&mut sat_a, Message::from("hello there!"));

    println!("t1 : {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2 : {:?}", sat_a);

    println!("msg : {:?}", msg);
}