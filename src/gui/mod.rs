use ggez::Context;
use ggez::graphics::{self, Text, Font, Color, Rect, TextFragment};
use ggez::input::mouse;
use ggez::event::MouseButton;
use std::collections::{HashMap, HashSet};

pub const UI_ID: usize = 0;

type Point = glam::Vec2;
struct BBox {a: Point, b: Point, _rect: Rect}

pub trait Widget {
    fn draw(&self, ctx: &mut Context);
}

pub struct Ui {
    rect: Rect,
    widget_edges: Vec<Vec<usize>>,
    id_counter: usize,
    reusable_ids: HashSet<usize>,
    widgets: Vec<Option<Box<dyn Widget>>>,
}

impl BBox {
    fn from_rect(rect: Rect) -> Self {
        let a = Point::new(rect.x, rect.y);
        let b = Point::new(rect.x + rect.w, rect.y + rect.h);
        Self {a: a, b: b, _rect: rect}
    }
    
    fn is_inside(&self, p: &Point) -> bool {
        p.x >= self.a.x && p.y >= self.a.y && p.x <= self.b.x && p.y <= self.b.y
    }
}

impl Ui {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            rect: Rect::new(0.0, 0.0, width, height),
            id_counter: 0,
            reusable_ids: HashSet::new(),
            widgets: vec![None],
            widget_edges: vec![vec![]],
        }
    }
}