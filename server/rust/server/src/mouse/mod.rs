pub mod mouse;

pub fn new_mouse_controller() -> mouse::MouseEvent {
    mouse::new()
}