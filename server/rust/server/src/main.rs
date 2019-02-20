mod mouse; 
extern crate ctrlc;

use std::net::UdpSocket;
use std::str;
use std::thread;
use std::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let socket = UdpSocket::bind("10.6.85.68:7000").unwrap();
        socket.set_nonblocking(true).unwrap();
        let mut buf = [0; 1024];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((amt, _src)) => {
                    let msg = str::from_utf8(&buf).unwrap_or("");
                    tx.send(msg[..amt].to_string()).unwrap();
                    },
                Err(e) =>  println!("couldn't recieve a datagram: {}", e)
            }
        }
    });

    thread::spawn(move || {
        let mut mouse = mouse::new_mouse_controller();
        loop {
            mouse.act(rx.recv().unwrap());
        }
    });

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}
    println!("Got it! Exiting...");
}