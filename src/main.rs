pub mod gui;
pub mod state;
pub mod assets;
pub mod util;

use state::playstate::{PlayState, levels};

use ggez;
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::conf::WindowMode;
use ggez::audio::{self, SoundSource};
use ggez::input::mouse;
use ggez::GameError;

use std::path;
use std::env;

const CELL_WIDTH: f32 = 6.0;
const CELL_MARGIN: f32 = 2.0;
const CELL_TOTAL: f32 = CELL_WIDTH + CELL_MARGIN;
const WINDOW_MARGIN: (i64, i64) = (600, 400);
const CAM_CONSTANT: f32 = 8.0;

struct MainState {
    current_state: Box<dyn EventHandler<GameError>>,
    music: audio::Source,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let music = audio::Source::new(ctx, "/CocooN - Soul Splitter.mp3").unwrap();
        let s = PlayState::new(ctx, levels::get_level_1())?;
        Ok(Self {
            current_state: Box::new(s),
            music: music,    
        })
    }

    fn play_music(&mut self, ctx: &mut Context) {
        // "detached" sounds keep playing even after they are dropped
        self.music.set_volume(0.8);
        self.music.set_repeat(true);
        let _ = self.music.play(ctx);
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.current_state.update(ctx)
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.current_state.draw(ctx)
    }
}

pub fn main() -> GameResult {
    let (width, height) = (1280.0, 720.0);
    let mut cb = ggez::ContextBuilder::new("super_simple", "yaya")
    .window_mode(WindowMode {
        width: width,
        height: height,
        resizable: false,
        ..WindowMode::default()
    });
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }

    let (mut ctx, event_loop) = cb.build()?;
    graphics::set_window_title(&mut ctx, "game of deth");
    mouse::set_cursor_hidden(&mut ctx, true);
    let mut state = MainState::new(&mut ctx)?;
    state.play_music(&mut ctx);
    event::run(ctx, event_loop, state)
}