//! Game of Life app

use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, DrawMode, DrawParam, Mesh};
use ggez::input::{
    self,
    keyboard::{KeyCode, KeyMods},
    mouse::MouseButton,
};
use ggez::{Context, ContextBuilder, GameResult};

use super::gui::{render_ui, ImGuiWrapper, UiState};
use super::logic::GameState;
use super::renderer::{ImageRenderer, Renderer};

pub struct App {
    game_state: GameState,
    ui_state: UiState,
    image: ImageRenderer,
    imgui_wrapper: ImGuiWrapper,
    scale: f32,
}

impl App {
    pub fn new(ctx: &mut Context, scale: f32, hidpi_factor: f32) -> Self {
        let win_size = graphics::size(ctx);
        let game_size = ((win_size.0 / scale) as usize, (win_size.1 / scale) as usize);
        let mut game_state = GameState::new(game_size);
        let mut image = ImageRenderer::new();
        image.set_size((win_size.0 as usize, win_size.1 as usize));
        game_state.randomize();

        Self {
            game_state,
            ui_state: UiState::new(),
            image,
            imgui_wrapper: ImGuiWrapper::new(ctx, hidpi_factor),
            scale,
        }
    }

    pub fn screen_pos_to_game(&self, mouse_position: (f32, f32)) -> (usize, usize) {
        let scale = self.image.scale(&self.game_state);
        (
            (mouse_position.0 / scale.0 as f32) as usize,
            (mouse_position.1 / scale.1 as f32) as usize,
        )
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.imgui_wrapper.mouse_captured {
            let mouse_left_pressed =
                input::mouse::button_pressed(ctx, input::mouse::MouseButton::Left);
            let mouse_right_pressed =
                input::mouse::button_pressed(ctx, input::mouse::MouseButton::Right);

            if mouse_left_pressed || mouse_right_pressed {
                let mouse_position = input::mouse::position(ctx);
                let game_pos = self.screen_pos_to_game((mouse_position.x, mouse_position.y));
                self.game_state.set_value_at_pos_with_radius(
                    game_pos,
                    self.ui_state.cursor_size.max(1),
                    mouse_left_pressed,
                );
            }
        }

        // Cycle
        self.game_state.cycle();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        if !self.imgui_wrapper.keyboard_captured {
            match keycode {
                KeyCode::Space => self.game_state.running = !self.game_state.running,
                KeyCode::Return => self.game_state.randomize(),
                _ => {}
            }
        }

        self.imgui_wrapper.update_key_down(keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.imgui_wrapper.update_key_up(keycode, keymods);
    }

    fn text_input_event(&mut self, _ctx: &mut Context, val: char) {
        self.imgui_wrapper.update_text(val);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((false, false, false));
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.imgui_wrapper.update_mouse_wheel(x, y);

        if !self.imgui_wrapper.mouse_captured {
            if self.ui_state.cursor_size as f32 + y < 1.0 {
                self.ui_state.cursor_size = 1
            } else if self.ui_state.cursor_size as f32 + y > 100.0 {
                self.ui_state.cursor_size = 100;
            } else {
                self.ui_state.cursor_size += y as usize;
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Render
        self.image.render(&self.game_state);

        // Create image
        let image = graphics::Image::from_rgba8(
            ctx,
            self.image.size.0 as u16,
            self.image.size.1 as u16,
            &self.image.data,
        )?;
        graphics::draw(ctx, &image, DrawParam::default())?;

        // Cursor target
        if !self.imgui_wrapper.mouse_captured {
            let circle = Mesh::new_circle(
                ctx,
                DrawMode::Fill(Default::default()),
                input::mouse::position(ctx),
                (self.ui_state.cursor_size as f32 / 2.0 * self.scale).max(1.0),
                0.5,
                graphics::Color::from_rgba(0, 255, 0, 64),
            )?;
            graphics::draw(ctx, &circle, DrawParam::default())?;
        }

        let mut game_state = self.game_state.clone();
        let mut ui_state = self.ui_state.clone();

        self.imgui_wrapper.render(ctx, |ui, nctx| {
            render_ui(ui, nctx, &mut game_state, &mut ui_state);
        });

        self.game_state = game_state;
        self.ui_state = ui_state;

        graphics::present(ctx)
    }
}

pub fn run() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("caw", "Srynetix")
        .window_setup(conf::WindowSetup {
            title: "caw - cellular automata workspace".to_string(),
            ..Default::default()
        })
        .window_mode(conf::WindowMode {
            width: 1280.0,
            height: 800.0,
            resizable: true,
            ..Default::default()
        })
        .build()
        .unwrap();

    let hidpi_factor = event_loop.get_primary_monitor().get_hidpi_factor() as f32;
    let mut game = App::new(&mut ctx, 2.0, hidpi_factor);

    if let Err(e) = event::run(&mut ctx, &mut event_loop, &mut game) {
        eprintln!("Error occured: {}", e);
    }
}
