use egui::{CentralPanel, Context, Layout, TextEdit, Button, Align};
use egui_extras::syntax_highlighting::{highlight, CodeTheme};

/// Struct representing the Lua code editor state.
pub struct LuaEditor {
    pub code: String,
}

impl Default for LuaEditor {
    fn default() -> Self {
        Self {
            code: "-- Write Lua code here\nprint('Hello, Lua!')".to_owned(),
        }
    }
}

impl LuaEditor {
    /// Updates the code in the editor.
    pub fn update_code(&mut self, code: &str) {
        self.code = code.to_owned();
    }
}

/// Shows the Lua code editor GUI, result display, and execute button.
///
/// - Displays a syntax-highlighted, non-editable box with available Lua functions.
/// - Provides a scrollable, fixed-height code editor with syntax highlighting.
/// - Shows result or error messages at the bottom.
/// - Calls `on_execute` when the Execute button is pressed.
pub fn show_lua_editor(
    ctx: &Context,
    editor: &mut LuaEditor,
    on_execute: impl Fn(&str) -> Result<(), mlua::Error>,
    result_message: &mut String,
) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Lua Code Editor");
        ui.add_space(8.0);

        ui.label("Write your Lua code below and click 'Execute' to run it.");
        ui.add_space(8.0);
        ui.label("Global functions available:");
        
        let mut functions = String::from("mouse_move(x, y) -- Move mouse to (x, y)\nleft_click() -- Perform a left click\nright_click() -- Perform a right click\nsleep(ms) -- Sleep for ms milliseconds");
        let theme = CodeTheme::from_memory(ui.ctx(), ui.style());
        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
            let job = highlight(ui.ctx(), ui.style(), &theme, string, "lua");
            ui.fonts(|f| f.layout_job(job))
        };
        ui.add(
            egui::TextEdit::multiline(&mut functions)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .desired_rows(4)
                .layouter(&mut layouter)
                .desired_width(f32::INFINITY)
                .interactive(false)
        );
        
        egui::ScrollArea::vertical()
            .max_height(400.0)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(&mut editor.code)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_rows(29)
                        .layouter(&mut layouter)
                        .desired_width(f32::INFINITY)
                        
                );
            });

         // Show result message at the bottom
        if !result_message.is_empty() {
            ui.label(result_message.as_str());
            ui.add_space(8.0);
        }
        
        ui.with_layout(Layout::right_to_left(Align::BOTTOM), |ui| {
            if ui.add(Button::new("Execute")).clicked() {
                let result = on_execute(&editor.code);
                *result_message = match result {
                    Ok(_) => "Code executed successfully!".to_string(),
                    Err(err) => format!("Error: {}", err),
                };
            }
        });
    });
}