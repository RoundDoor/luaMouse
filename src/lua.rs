use mlua::Lua;
use std::rc::Rc;
use std::cell::RefCell;
use enigo::{Enigo};
use crate::mouse_move;

/// Executes the given Lua code string with mouse and sleep functions bound.
///
/// This function sets up the Lua environment with the following global functions:
/// - `mouse_move(x, y)`: Moves the mouse cursor by the given relative x and y offsets (pixels).
/// - `left_click()`: Performs a left mouse button click.
/// - `right_click()`: Performs a right mouse button click.
/// - `sleep(ms)`: Pauses execution for the given number of milliseconds.
///
/// # Arguments
/// * `lua` - Reference to a Lua interpreter instance
/// * `code` - Lua code to execute
/// * `enigo` - Shared Enigo instance for mouse control
///
/// # Returns
/// * `Ok(())` if the code executed successfully
/// * `Err(mlua::Error)` if there was an error during execution
pub fn execute_lua_code(lua: &Lua, code: &str, enigo: Rc<RefCell<Enigo>>) -> Result<(), mlua::Error> {
    
    // Create a Lua function to move the mouse

    let mouse_move = {
        let enigo = enigo.clone();
        lua.create_function(move |_, (x, y): (i32, i32)| {
            mouse_move::move_mouse_wrapper(&mut enigo.borrow_mut(), x, y);
            Ok(())
        })?
    };

    // Create a Lua function to sleep for a given number of milliseconds

    let left_click = {
        let enigo = enigo.clone();
        lua.create_function(move |_, ()| {
            mouse_move::left_click_mouse(&mut enigo.borrow_mut());
            Ok(())
        })?
    };

    // Function to perform a right click

    let right_click = {
        let enigo = enigo.clone();
        lua.create_function(move |_, ()| {
            mouse_move::right_click_mouse(&mut enigo.borrow_mut());
            Ok(())
        })?
    };


    // Function to sleep for a given number of milliseconds

    let sleep = lua.create_function(|_, (ms,): (u64,)| {
        std::thread::sleep(std::time::Duration::from_millis(ms));
        Ok(())
    })?;


    // Register the functions in Lua
    lua.globals().set("sleep", sleep)?;
    lua.globals().set("right_click", right_click)?;
    lua.globals().set("left_click", left_click)?;
    lua.globals().set("mouse_move", mouse_move)?;
    lua.load(code).exec()
}