pub mod levels;

use std::io;
use std::io::Write; // <--- bring flush() into scope

use ggez;
use ggez::timer;
use ggez::event::{self, EventHandler, KeyCode, MouseButton};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::conf::WindowMode;
use ggez::audio::{self, SoundSource};
use ggez::input::{keyboard, mouse};

use std::collections::{HashMap};
use std::path;
use std::env;

const CELL_WIDTH: f32 = 6.0;
const CELL_MARGIN: f32 = 2.0;
const CELL_TOTAL: f32 = CELL_WIDTH + CELL_MARGIN;
const WINDOW_MARGIN: (i64, i64) = (600, 400);
const CAM_CONSTANT: f32 = 8.0;

struct Assets {
    colored_cells: Vec<ggez::graphics::Mesh>,
    music: audio::Source,
    font: graphics::Font,
    text_start: graphics::Text,
    text_alive: graphics::Text,
    text_added: graphics::Text,
    text_steps: graphics::Text,
}

impl Assets {
    fn new(ctx: &mut Context) -> Assets {
        let colored_cells = create_colored_cells(ctx);
        let music = audio::Source::new(ctx, "/CocooN - Soul Splitter.mp3").unwrap();
        let font = graphics::Font::new(ctx, "/DejaVuSerifCondensed.ttf").unwrap();
        let text_start = graphics::Text::new(("Start: 0", font, 20.0));
        let text_alive = graphics::Text::new(("Alive: 0", font, 20.0));
        let text_added = graphics::Text::new(("Added: 0", font, 20.0));
        let text_steps = graphics::Text::new(("Time steps: 0", font, 20.0));

        Assets {
            colored_cells: colored_cells, 
            music: music, 
            font: font, 
            text_added: text_added, 
            text_alive: text_alive,
            text_steps: text_steps,
            text_start: text_start,
        }
    }
}

struct MainState {
    pub alive: HashMap<(i64,i64), usize>,
    pub is_running: bool,
    pub one_turn: bool,
    pub assets: Assets,
    pub camera: (f32, f32),
    pressed_keys: Vec<(KeyCode,bool)>,
    keys_up: HashMap<KeyCode, bool>,
    keys_down: HashMap<KeyCode, bool>,
    mouse_pressed: bool,
    mouse_down: bool,
    mouse_position: (i64,i64),
    added_counter: usize,
    time_steps: usize,
}

impl MainState {
    fn new(ctx: &mut Context, cells: HashMap<(i64,i64),usize>) -> GameResult<MainState> {
        let mut assets = Assets::new(ctx);
        assets.text_start = graphics::Text::new((format!("Start: {}", cells.len()), assets.font, 20.0));
        assets.text_alive = graphics::Text::new((format!("Alive: {}", cells.len()), assets.font, 20.0));

        let mut pressed_keys = vec![];
        let mut keys_up = HashMap::new();
        let mut keys_down = HashMap::new();
        let keys = vec![KeyCode::Space,];
        for key in keys {
            pressed_keys.push((key, false));
            keys_up.insert(key, false);
            keys_down.insert(key, false);
        }
        let offset = calculate_offset(&cells, ctx);

        let state = MainState {
            alive: cells, 
            is_running: true, 
            one_turn: false,
            assets: assets,
            camera: offset,
            pressed_keys: pressed_keys,
            keys_up: keys_up,
            keys_down: keys_down,
            mouse_pressed: false,
            mouse_down: false,
            mouse_position: (0,0),
            added_counter: 0,
            time_steps: 0,
        };
        Ok(state)
    }

    fn play_music(&mut self) {
        // "detached" sounds keep playing even after they are dropped
        self.assets.music.set_volume(0.8);
        self.assets.music.set_repeat(true);
        let _ = self.assets.music.play();
    }

    fn new_level(&mut self, level: HashMap<(i64,i64),usize>, ctx: &mut Context) {
        self.assets.text_start = graphics::Text::new((format!("Start: {}", level.len()), self.assets.font, 20.0));
        self.assets.text_alive = graphics::Text::new((format!("Alive: {}", level.len()), self.assets.font, 20.0));
        self.assets.text_steps = graphics::Text::new((format!("Time steps: {}", self.time_steps), self.assets.font, 20.0));
        self.assets.text_added = graphics::Text::new((format!("Added: {}", self.added_counter), self.assets.font, 20.0));

        self.added_counter = 0;
        self.time_steps = 0;
        self.camera = calculate_offset(&level, ctx);

        self.alive = level;
        println!("cam: {:?}", self.camera);
    }

    fn tick(&mut self) {
        if self.is_running {
            let mut possibles: HashMap<(i64,i64),u64> = HashMap::new();
            for ((x,y),_) in &self.alive {
                *possibles.entry((x - 1, y - 1)).or_insert(0)   += 1;
                *possibles.entry((x - 1, *y)).or_insert(0)      += 1;
                *possibles.entry((x - 1, y + 1)).or_insert(0)   += 1;
                *possibles.entry((*x, y - 1)).or_insert(0)      += 1;
                *possibles.entry((*x, y + 1)).or_insert(0)      += 1;
                *possibles.entry((x + 1, y - 1)).or_insert(0)   += 1;
                *possibles.entry((x + 1, *y)).or_insert(0)      += 1;
                *possibles.entry((x + 1, y + 1)).or_insert(0)   += 1;
            }
            let mut next_gen: HashMap<(i64,i64),usize> = HashMap::new();
            for ((x,y),value) in &possibles {
                if self.alive.contains_key(&(*x,*y)) && (*value == 3 || *value == 2) {
                    let old_value = self.alive[&(*x,*y)];
                    let new_value = if old_value == 0 { 0 } else if old_value >= 15 { 15 } else { old_value + 1 };
                    next_gen.insert((*x,*y), new_value);
                } else if *value == 3 {
                    next_gen.insert((*x,*y), 1);
                }
            }
            self.alive = next_gen;
            
            self.time_steps += 1;
            
            self.assets.text_steps = graphics::Text::new((format!("Time steps: {}", self.time_steps), self.assets.font, 20.0));
            self.assets.text_alive = graphics::Text::new((format!("Alive: {}", self.alive.len()), self.assets.font, 20.0));
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 15;

        while timer::check_update_time(ctx, DESIRED_FPS) { 
            self.tick();
        }

        update_key_activity(ctx, self);
        update_mouse_activity(ctx, self);

        if self.mouse_down {
            if !self.alive.contains_key(&self.mouse_position) {
                self.alive.insert(self.mouse_position, 0);
                self.added_counter += 1;
                self.assets.text_added = graphics::Text::new((format!("Added: {}", self.added_counter), self.assets.font, 20.0));
            }
        }

        if self.keys_down[&KeyCode::Space] {
            self.is_running ^= true;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.camera.1 += CAM_CONSTANT;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.camera.0 += CAM_CONSTANT;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.camera.1 -= CAM_CONSTANT;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.camera.0 -= CAM_CONSTANT;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Key1) {
            let level = levels::get_level_1();
            self.new_level(level, ctx);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Key2) {
            let level = levels::get_level_2();
            self.new_level(level, ctx);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::R) {
            let (width, height) = graphics::drawable_size(&ctx);
            let level = levels::get_level_random((width as i64 - WINDOW_MARGIN.0) / CELL_TOTAL as i64, (height as i64 - WINDOW_MARGIN.1) / CELL_TOTAL as i64);
            self.new_level(level, ctx);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::E) {
            self.new_level(HashMap::new(), ctx);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::P) {
            for ((x,y),_) in &self.alive {
                print!("({},{}),", x, y);
            }
            io::stdout().flush().unwrap();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [4.0/255.0, 17.0/255.0, 24.0/255.0, 1.0].into());

        let m = mouse::position(ctx);
        let mx = (m.x / CELL_TOTAL).floor() * CELL_TOTAL;
        let my = (m.y / CELL_TOTAL).floor() * CELL_TOTAL;

        let hilight = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            graphics::Rect::new(0.0, 0.0 , CELL_TOTAL, CELL_TOTAL),
            graphics::Color::from_rgb(221, 227, 230))?;
        graphics::draw(ctx, &hilight, (na::Point2::new(mx, my),))?;

        for ((x,y),gen) in &self.alive {
            let pos_x = *x as f32 * (CELL_TOTAL) + self.camera.0 + CELL_MARGIN / 2.0;
            let pos_y = *y as f32 * (CELL_TOTAL) + self.camera.1 + CELL_MARGIN / 2.0;
            graphics::draw(
                ctx, 
                &self.assets.colored_cells[*gen], 
                (na::Point2::new(pos_x, pos_y),))?;
        }

        graphics::draw(ctx, &self.assets.text_steps, (na::Point2::new(10.0, 10.0),))?;
        graphics::draw(ctx, &self.assets.text_start, (na::Point2::new(10.0, 40.0),))?;
        graphics::draw(ctx, &self.assets.text_added, (na::Point2::new(10.0, 70.0),))?;
        graphics::draw(ctx, &self.assets.text_alive, (na::Point2::new(10.0, 100.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn update_key_activity(ctx: &mut Context, state: &mut MainState) {
    let mut next = vec![];
    for (key, pressed) in &state.pressed_keys {
        let current_val = keyboard::is_key_pressed(ctx, *key);
        if current_val && !*pressed {
            *state.keys_down.get_mut(key).unwrap() = true;
        } else if !current_val && *pressed {
            *state.keys_up.get_mut(key).unwrap() = true;
        } else {
            if state.keys_down[key] {
                *state.keys_down.get_mut(key).unwrap() = false;
            }
            if state.keys_up[key] {
                *state.keys_up.get_mut(key).unwrap() = false;
            }
        }
        next.push((*key, current_val));
    }
    state.pressed_keys = next;
}

fn update_mouse_activity(ctx: &mut Context, state: &mut MainState) {
    let m_pos = mouse::position(ctx);
    let mx = ((m_pos.x - state.camera.0) / CELL_TOTAL).floor() as i64;
    let my = ((m_pos.y - state.camera.1) / CELL_TOTAL).floor() as i64;
    let mouse_pressed = mouse::button_pressed(ctx, MouseButton::Left);
    
    state.mouse_position = (mx, my);
    if !state.mouse_down && !state.mouse_pressed {
        if mouse_pressed {
            state.mouse_down = true;
        }
    } else {
        state.mouse_down = false;
    }
    state.mouse_pressed = mouse_pressed; 
}

pub fn create_colored_cells(ctx: &mut Context) -> Vec<ggez::graphics::Mesh> {
    let mut colors = vec![];
    colors.push(graphics::Color::from_rgb(0, 84, 163));
    colors.push(graphics::Color::from_rgb(192, 49, 33));
    colors.push(graphics::Color::from_rgb(194, 101, 57));
    colors.push(graphics::Color::from_rgb(241, 118, 15));
    colors.push(graphics::Color::from_rgb(234, 182, 21));
    colors.push(graphics::Color::from_rgb(238, 214, 44));
    colors.push(graphics::Color::from_rgb(211, 215, 30));
    colors.push(graphics::Color::from_rgb(166, 177, 55));
    colors.push(graphics::Color::from_rgb(124, 185, 56));
    colors.push(graphics::Color::from_rgb(64, 161, 66));
    colors.push(graphics::Color::from_rgb(82, 180, 79));
    colors.push(graphics::Color::from_rgb(20, 159, 40));
    colors.push(graphics::Color::from_rgb(0, 144, 94));
    colors.push(graphics::Color::from_rgb(27, 146, 150));
    colors.push(graphics::Color::from_rgb(0, 149, 182));
    colors.push(graphics::Color::from_rgb(4, 153, 186));
    // 16 colors
    let mut meshes = vec![];
    for c in colors {
        let mesh = graphics::Mesh::new_rectangle(
                        ctx, 
                        graphics::DrawMode::fill(), 
                        graphics::Rect::new(0.0, 0.0 , CELL_WIDTH, CELL_WIDTH),
                        c).unwrap();
        meshes.push(mesh);
    }
    meshes
}

fn calculate_offset(cells: &HashMap<(i64,i64),usize>, ctx: &mut Context) -> (f32, f32) {
    let dim = graphics::drawable_size(ctx);
    let mut minx = std::f32::MAX;
    let mut maxx = std::f32::MIN;
    let mut miny = std::f32::MAX;
    let mut maxy = std::f32::MIN;

    for ((x,y),_) in cells {
        let xf = *x as f32;
        let yf = *y as f32;
        minx = if xf < minx { xf } else { minx };
        maxx = if xf > maxx { xf } else { maxx };
        miny = if yf < miny { yf } else { miny };
        maxy = if yf > maxy { yf } else { maxy };
    }
    
    let diff_x = minx + (maxx - minx);
    let diff_y = miny + (maxy - miny);
    let unscaled_x = if diff_x > 0.0 { (dim.0 / CELL_TOTAL) - diff_x } else { (-dim.0 / CELL_TOTAL) + diff_x };
    let unscaled_y = if diff_y > 0.0 { (dim.1 / CELL_TOTAL) - diff_y } else { (-dim.1 / CELL_TOTAL) + diff_y }; 



    let offsetx = (unscaled_x / 2.0).floor() * CELL_TOTAL;
    let offsety = (unscaled_y / 2.0).floor() * CELL_TOTAL;

    println!("dif: {}, {}", diff_x, diff_y);
    println!("unscaled: {}, {}", unscaled_x, unscaled_y);
    println!("offset: {}, {}", offsetx, offsety);

    (offsetx, offsety)
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

    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "game of deth");
    mouse::set_cursor_hidden(ctx, true);

    let (width, height) = graphics::drawable_size(&ctx);
    let start = levels::get_level_random((width as i64 - WINDOW_MARGIN.0) / CELL_TOTAL as i64, (height as i64 - WINDOW_MARGIN.1) / CELL_TOTAL as i64);

    let state = &mut MainState::new(ctx, start)?;
    state.play_music();
    event::run(ctx, event_loop, state)
}