use enigo::{
    Button,
    Direction::{Click},
    Enigo, Mouse,
    {Coordinate::Rel},
};

pub fn move_mouse_wrapper(enigo: &mut Enigo, x: i32, y: i32) {

    enigo.move_mouse(x, y, Rel).unwrap();

}

pub fn left_click_mouse(enigo: &mut Enigo) {
    enigo.button(Button::Left, Click).unwrap();
}

pub fn right_click_mouse(enigo: &mut Enigo) {
    enigo.button(Button::Right, Click).unwrap();
}