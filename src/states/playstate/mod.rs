pub mod levels;

use levels::Level;
use crate::assets::Assets;
use crate::{util, CELL_MARGIN, WINDOW_MARGIN, CELL_TOTAL, CAM_CONSTANT};

use std::collections::HashMap;
use std::io;
use std::io::Write;

use ggez::timer;
use ggez::event::{EventHandler, KeyCode};
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, Rect};
use ggez::input::{keyboard, mouse};
use ggez::nalgebra as na;

pub struct PlayState {
    pub alive: Level,
    pub start: Level,
    pub is_running: bool,
    pub one_turn: bool,
    pub assets: Assets,
    pub camera: (f32, f32),
    pub pressed_keys: Vec<(KeyCode,bool)>,
    pub keys_up: HashMap<KeyCode, bool>,
    pub keys_down: HashMap<KeyCode, bool>,
    pub mouse_pressed: bool,
    pub mouse_down: bool,
    pub mouse_position: (i64,i64),
    added_counter: usize,
    time_steps: usize,
}

impl PlayState {
    pub fn new(ctx: &mut Context, level: Level) -> GameResult<Self> {
        let mut assets = Assets::new(ctx);
        assets.text_start = graphics::Text::new((format!("Start: {}", level.len()), assets.font, 20.0));
        assets.text_alive = graphics::Text::new((format!("Alive: {}", level.len()), assets.font, 20.0));

        let mut pressed_keys = vec![];
        let mut keys_up = HashMap::new();
        let mut keys_down = HashMap::new();
        let keys = vec![KeyCode::Space, KeyCode::Right, KeyCode::P];
        for key in keys {
            pressed_keys.push((key, false));
            keys_up.insert(key, false);
            keys_down.insert(key, false);
        }
        let offset = util::calculate_offset(&level, ctx);

        let state = Self {
            alive: level.clone(),
            start: level,
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

    fn new_level(&mut self, level: HashMap<(i64,i64),usize>, ctx: &mut Context) {
        self.assets.text_start = graphics::Text::new((format!("Start: {}", level.len()), self.assets.font, 20.0));
        self.assets.text_alive = graphics::Text::new((format!("Alive: {}", level.len()), self.assets.font, 20.0));
        self.assets.text_steps = graphics::Text::new((format!("Time steps: {}", self.time_steps), self.assets.font, 20.0));
        self.assets.text_added = graphics::Text::new((format!("Added: {}", self.added_counter), self.assets.font, 20.0));

        self.added_counter = 0;
        self.time_steps = 0;
        self.camera = util::calculate_offset(&level, ctx);

        self.alive = level.clone();
        self.start = level;
    }

    fn tick(&mut self) {
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

impl EventHandler for PlayState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 15;

        while timer::check_update_time(ctx, DESIRED_FPS) { 
            if self.is_running && self.alive.len() > 0 {
                self.tick();
            }
        }

        util::update_key_activity(ctx, self);
        util::update_mouse_activity(ctx, self);

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
        if self.keys_down[&KeyCode::Right] {
            if !self.is_running {
                self.tick();
            }
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

        // Level selection
        if keyboard::is_key_pressed(ctx, KeyCode::Key1) {
            let level = levels::get_level_1();
            self.new_level(level, ctx);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Key2) {
            let level = levels::get_level_2();
            self.new_level(level, ctx);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Key3) {
            let level = levels::get_level_3();
            self.new_level(level, ctx);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Key4) {
            let level = levels::get_level_4();
            self.new_level(level, ctx);
        }       
        if keyboard::is_key_pressed(ctx, KeyCode::Key5) {
            let level = levels::get_level_5();
            self.new_level(level, ctx);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Key6) {
            let level = levels::get_level_6();
            self.new_level(level, ctx);
        }        
        if keyboard::is_key_pressed(ctx, KeyCode::Key7) {
            let level = levels::get_level_7();
            self.new_level(level, ctx);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Key8) {
            let level = levels::get_level_8();
            self.new_level(level, ctx);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Key9) {
            let level = levels::get_level_9();
            self.new_level(level, ctx);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Key0) {
            let (width, height) = graphics::drawable_size(&ctx);
            let level = levels::get_level_random((width as i64 - WINDOW_MARGIN.0) / CELL_TOTAL as i64, (height as i64 - WINDOW_MARGIN.1) / CELL_TOTAL as i64);
            self.new_level(level, ctx);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::E) {
            self.new_level(HashMap::new(), ctx);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::R) {
            self.new_level(self.start.clone(), ctx);
        }

        if self.keys_down[&KeyCode::P] {
            for ((x,y),_) in &self.alive {
                print!("({},{}),", x, y);
            }
            print!("\n\n");
            io::stdout().flush().unwrap();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult{
        graphics::clear(ctx, [4.0/255.0, 17.0/255.0, 24.0/255.0, 1.0].into());

        let m = mouse::position(ctx);
        let mx = (m.x / CELL_TOTAL).floor() * CELL_TOTAL;
        let my = (m.y / CELL_TOTAL).floor() * CELL_TOTAL;

        let hilight = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            Rect::new(0.0, 0.0 , CELL_TOTAL, CELL_TOTAL),
            Color::from_rgb(221, 227, 230))?;

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