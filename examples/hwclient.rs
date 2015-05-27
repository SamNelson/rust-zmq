extern crate zmq;

fn main() {
    let mut context = zmq::Context::new();
    let socket = context.socket(zmq::REQ);

    socket.connect("tcp://localhost:25555");

    loop {
        socket.send("a req from client");
        println!("{}", socket.recv());
    }
}
