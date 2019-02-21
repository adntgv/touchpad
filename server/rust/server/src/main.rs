extern crate ctrlc;
extern crate get_if_addrs;

mod mouse; 
use std::net::UdpSocket;
use std::str;
use std::thread;
use std::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::env;

static PORT: i32 = 7000; 

fn derive_ip() -> String {
    let mut address: String = "localhost".to_string();
    for iface in get_if_addrs::get_if_addrs().unwrap() {
        match iface.addr {
            get_if_addrs::IfAddr::V4(network) => { 
                let ip = network.ip;
                if ip.is_private() {
                    address = format!("{}", ip);
                }
            },
            _ => continue
        }
    }
    return address;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut address: String;

        if args.len() < 2 {
            address = derive_ip();
        } else {
            address = args[1].to_string();
        }
        address = format!("{}:{}", address, PORT);
        println!("Starting server on {}", address);
        let socket = UdpSocket::bind(address).unwrap();
        socket.set_nonblocking(true).unwrap();
        let mut buf = [0; 1024];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((amt, _src)) => {
                    let msg = str::from_utf8(&buf).unwrap_or("");
                    tx.send(msg[..amt].to_string()).unwrap();
                    },
                Err(_e) => continue
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

    println!("Press Ctrl-C to exit");
    while running.load(Ordering::SeqCst) {}
    println!("Exiting...");
}