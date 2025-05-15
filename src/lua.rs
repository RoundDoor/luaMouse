use mlua::Lua;
use std::rc::Rc;
use std::cell::RefCell;
use enigo::{Enigo};
use crate::mouse_move;


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