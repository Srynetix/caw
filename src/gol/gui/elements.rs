use ggez::event;
use ggez::graphics;
use ggez::timer;
use ggez::Context;

use imgui::*;
use imgui_gfx_renderer::*;

use super::super::logic::GameState;
use super::state::UiState;

fn show_help_marker(ui: &Ui, desc: &str) {
    ui.text_disabled(im_str!("(?)"));
    if ui.is_item_hovered() {
        ui.tooltip(|| {
            ui.text(desc);
        });
    }
}

fn draw_main_menu(ui: &Ui, ctx: &mut Context, _game_state: &mut GameState, ui_state: &mut UiState) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("File"), true, || {
            if MenuItem::new(im_str!("Quit"))
                .shortcut(im_str!("ALT+F4"))
                .build(&ui)
            {
                event::quit(ctx);
            }
        });
        ui.menu(im_str!("Tools"), true, || {
            if MenuItem::new(im_str!("Settings")).build(&ui) {
                ui_state.show_window = true;
            }
        });
        ui.menu(im_str!("Help"), true, || {
            if MenuItem::new(im_str!("Show help")).build(&ui) {
                ui_state.show_help = true;
            }

            ui.separator();

            if MenuItem::new(im_str!("About")).build(&ui) {
                ui_state.show_about = true;
            }
        });
    });
}

fn draw_settings_window(
    ui: &Ui,
    ctx: &mut Context,
    game_state: &mut GameState,
    ui_state: &mut UiState,
) {
    // Configuration window
    if ui_state.show_window {
        Window::new(im_str!("caw settings"))
            .position([50.0, 50.0], Condition::Always)
            .position_pivot([0.0, 0.0])
            .movable(false)
            .resizable(false)
            .opened(&mut ui_state.show_window)
            .collapsible(true)
            .build(&ui, || {
                ui.text(im_str!("Rendering"));
                ui.separator();
                ui.text(im_str!(" FPS: {:2.0}", timer::fps(ctx)));
                ui.separator();
                ui.text(im_str!("Grid settings"));
                ui.separator();
                ui.text(im_str!(" Current ticks: {}", game_state.current_tick));
                ui.text(im_str!(" Moving cells: {}", game_state.stats.moving));
                ui.text(im_str!(" Stopped cells: {}", game_state.stats.stopped));
                ui.separator();
                ui.text(im_str!("Actions"));
                ui.separator();
                // Is running
                ui.checkbox(im_str!("Running"), &mut game_state.running);
                ui.same_line(0.0);
                show_help_marker(&ui, "Pause or resume simulation state");

                // Randomize state
                if ui.button(im_str!("Randomize"), [100.0, 20.0]) {
                    game_state.randomize();
                }
                ui.same_line(0.0);
                // Clear state
                if ui.button(im_str!("Clear"), [100.0, 20.0]) {
                    game_state.clear();
                }
            });
    }
}

fn draw_help_window(
    ui: &Ui,
    ctx: &mut Context,
    _game_state: &mut GameState,
    ui_state: &mut UiState,
) {
    let (win_w, _win_h) = graphics::size(ctx);

    if ui_state.show_help {
        Window::new(im_str!("Help"))
            .position([win_w - 50.0, 50.0], Condition::Always)
            .position_pivot([1.0, 0.0])
            .opened(&mut ui_state.show_help)
            .movable(false)
            .resizable(false)
            .collapsible(true)
            .build(&ui, || {
                ui.text(im_str!("Mouse left-click to draw cells"));
                ui.text(im_str!("Mouse right-click to erase cells"));
                ui.text(im_str!("Mouse wheel to change draw size"));
            });
    }
}

fn draw_about_window(
    ui: &Ui,
    ctx: &mut Context,
    _game_state: &mut GameState,
    ui_state: &mut UiState,
) {
    let (win_w, win_h) = graphics::size(ctx);

    if ui_state.show_about {
        Window::new(im_str!("About"))
            .position([win_w / 2.0, win_h / 2.0], Condition::Always)
            .position_pivot([0.5, 0.5])
            .opened(&mut ui_state.show_about)
            .movable(false)
            .resizable(false)
            .collapsible(false)
            .build(&ui, || {
                ui.text(im_str!("caw - cellular automata workspace"));
                ui.text(im_str!("version {}", env!("CARGO_PKG_VERSION")));
                ui.separator();
                ui.text(im_str!("written in Rust using 'ggez' and 'imgui-rs'"));
                ui.separator();
                ui.text(im_str!("by Srynetix"));
            })
    }
}

pub fn render_ui(ui: &Ui, ctx: &mut Context, game_state: &mut GameState, ui_state: &mut UiState) {
    draw_main_menu(ui, ctx, game_state, ui_state);
    draw_settings_window(ui, ctx, game_state, ui_state);
    draw_help_window(ui, ctx, game_state, ui_state);
    draw_about_window(ui, ctx, game_state, ui_state);
}
