extern crate enigo;
use enigo::{Enigo, MouseControllable, MouseButton};
use std::net::UdpSocket;
use std::str;
use std::fmt;

static LONG_TAP_TIMEOUT: i64 = 500;
static DRAG_TIMEOUT: i64 = 500;

#[derive(Debug, Copy, Clone)]
enum Action {
    NONE,
    DOWN,
    UP,
    MOVE,
}

fn to_action(s: &str) -> Action {
    match s {
        "ACTION_DOWN" => Action::DOWN,
        "ACTION_UP" => Action::UP,
        "ACTION_MOVE" => Action::MOVE,
        _ => Action::NONE,
    }
}

#[derive(Debug, Copy, Clone)]
struct MouseEvent {
    paction: Action,
    action: Action,
    dx: f64,
    px: f64,
    x: f64,
    dy: f64,
    py: f64,
    y: f64,
    event_time: i64,
    down_time: i64,
    mul: i64,
    pressed: bool,
}

fn new() -> MouseEvent {
        MouseEvent {
            paction: Action::NONE,
            action: Action::NONE,
            dx: 0.0,
            px: 0.0,
            x: 0.0,
            dy: 0.0,
            py: 0.0,
            y: 0.0,
            event_time: 0,
            down_time: 0,
            mul: 0,
            pressed: false,
        }
    }

impl MouseEvent {
    fn act(&mut self, msg: String){
        if msg.contains("MotionEvent") {
            let s =  msg.replace("MotionEvent","")
                .replace(" ","")
                .replace("{","")
                .replace("}","");

            let fields: Vec<&str> = s
                .trim().split(',').collect();

            for field in &fields {
                let parts: Vec<&str> = field.split("=").collect();
                match parts[0] {
                    "action" => {
                        self.paction = self.action; 
                        self.action = to_action(parts[1]);
                        },
                    "x[0]" => {
                        self.px = self.x; 
                        self.x = parts[1].parse::<f64>().unwrap();
                        },
                    "y[0]" => {
                        self.py = self.y; 
                        self.y = parts[1].parse::<f64>().unwrap();
                        },
                    "eventTime" => {
                        self.event_time = parts[1].parse::<i64>().unwrap();
                        },
                    "downTime" => {
                        self.down_time = parts[1].parse::<i64>().unwrap();
                        },
                    _ => continue,
                };
            }
            println!("{:?}", self);
            self._do();
        }
    }
    fn _do(&mut self) {
        
        println!("doing");
        let mut enigo = Enigo::new();
        self.dx = self.x - self.px;
        self.dy = self.y - self.py; 
        
        match self.paction {
            Action::DOWN => {
                match self.action {
                    Action::UP => {
                        if self.pressed {
                            println!("Mouse up");
                            enigo.mouse_up(MouseButton::Left);
                            return
                        }
                        if self.event_time - self.down_time < LONG_TAP_TIMEOUT{
                            println!("Click left");
                            enigo.mouse_click(MouseButton::Left)
                        } else {
                            println!("Click right");
                            enigo.mouse_click(MouseButton::Right)
                        }
                        return
                    },
                    Action::MOVE => {
                        if self.event_time - self.down_time > DRAG_TIMEOUT {
                            if !self.pressed {
                                println!("Mouse down");
                                enigo.mouse_down(MouseButton::Left);
                                return
                            }
                        }
                        println!("Mouse move");
                        enigo.mouse_move_relative(self.dx as i32, self.dy as i32);
                        return;
                        }

                    _ => return,
                }
            },
            Action::MOVE => {
                match self.action {
                    Action::UP => {
                        self.dx = 0.0;
                        self.dy = 0.0;
                    },
                    Action::MOVE => {
                        println!("Mouse move");
                        enigo.mouse_move_relative(self.dx as i32, self.dy as i32);
                    },
                    _ => return,
                }
            },
            _ => return,

        }
        return
    }
}

impl fmt::Display for MouseEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "paction: {},\naction: {},\ndx: {},\npx: {},x: {},\ndy: {},\npy: {},y: {},event_time: {},down_time: {},mul: {}" ,
            self.paction, self.action, self.dx, self.px, self.x, self.dy, self.py, self.y, self.event_time, self.down_time, self.mul)
    }
 }

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}


fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("10.6.85.68:7000")?;

    let mut buf = [0; 1024];
    let mut mouse = new();
    loop {
        match socket.recv_from(&mut buf) {
            Ok((_amt, _src)) => {
                let msg = str::from_utf8(&buf).unwrap_or("");
                mouse.act(msg.to_string());
                },
            Err(e) => {
                println!("couldn't recieve a datagram: {}", e);
                break
            }
        }
    }
    Ok(())
}