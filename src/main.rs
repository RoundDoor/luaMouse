use std::rc::Rc;
use std::cell::RefCell;
use enigo::Enigo;
use mlua::Lua;
mod gui;
mod lua; 
pub mod mouse_move;

struct App {
    editor: gui::LuaEditor,
    lua: Lua,
    enigo: Rc<RefCell<Enigo>>,
    result_message: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            editor: gui::LuaEditor::default(),
            lua: Lua::new(),
            enigo: Rc::new(RefCell::new(Enigo::new(&enigo::Settings::default()).unwrap())),
            result_message: String::new(),
        }
    }
}

fn main() {
    let mut app = App::default();

    app.editor.update_code("-- Move the mouse in a square pattern
local side = 200      -- Length of each side of the square in pixels
local steps = 50      -- Number of steps per side (smoothness)
local delay = 10      -- Delay in ms between steps

for i = 1, 2 do  -- Draw the square twice
    -- Right
    for s = 1, steps do
        mouse_move(side / steps, 0)
        sleep(delay)
    end
    -- Down
    for s = 1, steps do
        mouse_move(0, side / steps)
        sleep(delay)
    end
    -- Left
    for s = 1, steps do
        mouse_move(-side / steps, 0)
        sleep(delay)
    end
    -- Up
    for s = 1, steps do
        mouse_move(0, -side / steps)
        sleep(delay)
    end
end
");


    let mut options = eframe::NativeOptions::default();
    options.viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([800.0, 600.0])
        .with_resizable(false);
    eframe::run_native("Lua Mouse App", options, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
}



impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let lua = &self.lua;
        let enigo = self.enigo.clone();
        gui::show_lua_editor(ctx, &mut self.editor, |code| {
            lua::execute_lua_code(lua, code, enigo.clone())
        }, &mut self.result_message);
    }
}