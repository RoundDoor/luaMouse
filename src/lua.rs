use crate::mouse_move;
use enigo::Enigo;
use mlua::Lua;
use std::cell::RefCell;
use std::rc::Rc;

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
pub fn execute_lua_code(
    lua: &Lua,
    code: &str,
    enigo: Rc<RefCell<Enigo>>,
) -> Result<(), mlua::Error> {
    // check if functions are already registered
    if !lua
        .globals()
        .get::<Option<mlua::Function>>("mouse_move")?
        .is_some()
    {
        // Register the mouse_move function
        let mouse_move = {
            let enigo = enigo.clone();
            lua.create_function(move |_, (x, y): (i32, i32)| {
                mouse_move::move_mouse(&mut enigo.borrow_mut(), x, y);
                Ok(())
            })?
        };
        lua.globals().set("mouse_move", mouse_move)?;
    }

    if !lua
        .globals()
        .get::<Option<mlua::Function>>("left_click")?
        .is_some()
    {
        // Register the left_click function
        let left_click = {
            let enigo = enigo.clone();
            lua.create_function(move |_, ()| {
                mouse_move::left_click_mouse(&mut enigo.borrow_mut());
                Ok(())
            })?
        };

        lua.globals().set("left_click", left_click)?;
    }

    if !lua
        .globals()
        .get::<Option<mlua::Function>>("right_click")?
        .is_some()
    {
        // Register the right_click function
        let right_click = {
            lua.create_function(move |_, ()| {
                mouse_move::right_click_mouse(&mut enigo.borrow_mut());
                Ok(())
            })?
        };

        lua.globals().set("right_click", right_click)?;
    }

    if !lua
        .globals()
        .get::<Option<mlua::Function>>("sleep")?
        .is_some()
    {
        // Register the sleep function
        let sleep = {
            lua.create_function(move |_, (ms,): (u64,)| {
                std::thread::sleep(std::time::Duration::from_millis(ms));
                Ok(())
            })?
        };
        lua.globals().set("sleep", sleep)?;
    }

    lua.load(code).exec()
}
