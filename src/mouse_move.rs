// Wrappers for Enigo mouse movement and click functions.
// All functions use relative movement.

use enigo::{
    Button,
    Direction::{Click},
    Enigo, Mouse,
    {Coordinate::Rel},
};

/// Moves the mouse cursor by the given relative x and y offsets.
///
/// # Arguments
/// * `enigo` - Mutable reference to an Enigo instance
/// * `x` - Relative x offset (pixels)
/// * `y` - Relative y offset (pixels)
///
/// # Example
/// ```
/// move_mouse_wrapper(&mut enigo, 10, 0); // Move mouse 10 pixels to the right
/// ```
pub fn move_mouse_wrapper(enigo: &mut Enigo, x: i32, y: i32) {
    enigo.move_mouse(x, y, Rel).unwrap();
}

/// Performs a left mouse button click.
///
/// # Arguments
/// * `enigo` - Mutable reference to an Enigo instance
pub fn left_click_mouse(enigo: &mut Enigo) {
    enigo.button(Button::Left, Click).unwrap();
}

/// Performs a right mouse button click.
///
/// # Arguments
/// * `enigo` - Mutable reference to an Enigo instance
pub fn right_click_mouse(enigo: &mut Enigo) {
    enigo.button(Button::Right, Click).unwrap();
}