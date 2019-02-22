extern crate enigo;

use super::touchpad::{Action, Touchpad};
use std::fmt;
use self::enigo::{Enigo, MouseButton};

static LONG_TAP_TIMEOUT: i64 = 500;
static DRAG_TIMEOUT: i64 = 500;

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
    pub fn act(&mut self, msg: Touchpad, enigo: &mut Enigo){

        self.paction = self.action; 
        self.action = msg.action;
        self.px = self.x; 
        self.x = msg.x[0];
        self.py = self.y; 
        self.y = msg.y[0];
        self.event_time = msg.eventTime;
        self.down_time = msg.downTime;

        self._do(enigo);
    }
    
    fn _do(&mut self, enigo: &mut Enigo) {
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