extern crate zmq;

use std::thread;

fn main() {
    let mut context = zmq::Context::new();
    let socket = context.socket(zmq::REP);

    socket.bind("tcp://*:25555");

    loop {
        println!("{}", socket.recv());

        // do some work
        thread::sleep_ms(1000);

        socket.send("a resp from server");
    }
}

