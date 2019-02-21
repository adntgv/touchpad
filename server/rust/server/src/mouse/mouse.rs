extern crate enigo;

use std::fmt;
use self::enigo::{Enigo, MouseControllable, MouseButton};

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
pub struct MouseEvent {
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
    mul: i32,
    pressed: bool,
}

pub fn new() -> MouseEvent {
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
            mul: 1,
            pressed: false,
        }
    }

impl MouseEvent {
    pub fn act(&mut self, msg: String){
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
            self._do();
        }
    }
    
    fn _do(&mut self) {
        let mut enigo = Enigo::new();
        self.dx = self.x - self.px;
        self.dy = self.y - self.py; 
        self.mul = (self.dx.powf(2.0) + self.dy.powf(2.0)).sqrt().ln() as i32;
        match self.paction {
            Action::DOWN => {
                match self.action {
                    Action::UP => {
                        if self.pressed {
                            enigo.mouse_up(MouseButton::Left);
                            return
                        }
                        if self.event_time - self.down_time < LONG_TAP_TIMEOUT{
                            enigo.mouse_click(MouseButton::Left)
                        } else {
                            enigo.mouse_click(MouseButton::Right)
                        }
                        return
                    },
                    Action::MOVE => {
                        if self.event_time - self.down_time > DRAG_TIMEOUT {
                            if !self.pressed {
                                enigo.mouse_down(MouseButton::Left);
                                return
                            }
                        }
                        enigo.mouse_move_relative(self.mul * self.dx as i32, self.mul * self.dy as i32);
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
                        enigo.mouse_move_relative(self.mul * self.dx as i32, self.mul * self.dy as i32);
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