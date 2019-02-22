pub mod mouse;
pub mod touchpad;

pub fn new_mouse_controller() -> mouse::MouseEvent {
    mouse::new()
}