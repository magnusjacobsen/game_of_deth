use ggez;
use ggez::timer;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::collections::{HashMap, HashSet};
use rand::prelude::*;

struct MainState {
    pub alive: HashSet<(i64,i64)>,
    start_pop: usize,
}

impl MainState {
    fn new(cells: HashSet<(i64,i64)>) -> GameResult<MainState> {
        let start_pop = cells.len();
        let s = MainState {alive: cells, start_pop: start_pop};
        Ok(s)
    }

    fn tick(&mut self) {
        let mut possibles: HashMap<(i64,i64),u64> = HashMap::new();
        for (x,y) in &self.alive {
            *possibles.entry((x - 1, y - 1)).or_insert(0)   += 1;
            *possibles.entry((x - 1, *y)).or_insert(0)      += 1;
            *possibles.entry((x - 1, y + 1)).or_insert(0)   += 1;
            *possibles.entry((*x, y - 1)).or_insert(0)      += 1;
            *possibles.entry((*x, y + 1)).or_insert(0)      += 1;
            *possibles.entry((x + 1, y - 1)).or_insert(0)   += 1;
            *possibles.entry((x + 1, *y)).or_insert(0)      += 1;
            *possibles.entry((x + 1, y + 1)).or_insert(0)   += 1;
        }
        let mut next_gen: HashSet<(i64,i64)> = HashSet::new();
        for ((x,y),value) in &possibles {
            if *value == 3 || (*value == 2 && self.alive.contains(&(*x,*y))) { //|| (*value == 4 && self.alive.contains(&(*x,*y))) {
                if *x < -20 || *x > 220 || *y < -20 || *y > 170 { continue; }
                next_gen.insert((*x,*y));
            }
        }

        let mut rng = rand::thread_rng();
        for _ in 0..1 {
            let x = rng.gen_range(0, 200);
            let y = rng.gen_range(0, 150);
            next_gen.insert((x, y));
        }
        let current = next_gen.len();
        println!("current_pop: {}, start_pop: {}", current, self.start_pop);
        self.alive = next_gen;
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 15;

        while timer::check_update_time(ctx, DESIRED_FPS) { 
            self.tick();
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let cell = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            graphics::Rect::new(0.0, 0.0 , 4.0, 4.0),
            graphics::Color::from_rgb(26, 234, 66))?;

        for (x,y) in &self.alive {
            graphics::draw(ctx, &cell, (na::Point2::new(*x as f32 * 4.0, *y as f32 * 4.0),))?;

        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "yaya");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "Game of Lyfe");

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1000, 5000);
    let (width, height) = graphics::drawable_size(&ctx);
    let mut start = HashSet::new();

    for _ in 0..num {
        let x = rng.gen_range(0, width as i64 / 4 - 100);
        let y = rng.gen_range(0, height as i64 / 4 - 50);
        start.insert((x + 50, y + 25));
    }

    let state = &mut MainState::new(start)?;
    event::run(ctx, event_loop, state)
}