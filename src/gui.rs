use ggez::nalgebra as na;
use ggez::Context;
use ggez::graphics::{self, Text, Font, Color, Rect, TextFragment};
use ggez::input::mouse;
use ggez::event::MouseButton;

type Point = na::Point2<f32>;
struct BBox {a: Point, b: Point, rect: Rect}

struct MouseState {
    down: bool,
    up: bool,
    pressed: bool,
    hovered: bool,
}

struct Button {
    b_box: BBox,
    mouse_state: MouseState,
    label_offset: Point,
    label: (graphics::Text, graphics::Text),
    border: (graphics::Mesh, graphics::Mesh),
}

pub trait Draw {
    fn update(&mut self, position: Point, is_pressed: bool);
    fn draw(&self, ctx: &mut Context);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl MouseState {
    fn new() -> Self {
        Self { down: false, up: false, pressed: false, hovered: false }
    }

    fn update(&mut self, is_inside: bool, is_pressed: bool) {
        if is_inside && is_pressed && !self.down && !self.pressed {
            self.down = true;
        } else if self.pressed && !is_pressed {
            self.up = true;
        } else {
            self.down = false;
            self.up = false;
        }
        self.pressed = is_pressed;
        self.hovered = is_inside;
    }
}

impl BBox {
    fn from_rect(rect: Rect) -> Self {
        let a = Point::new(rect.x, rect.y);
        let b = Point::new(rect.x + rect.w, rect.y + rect.h);
        Self {a: a, b: b, rect: rect}
    }
    
    fn is_inside(&self, p: &Point) -> bool {
        p.x >= self.a.x && p.y >= self.a.y && p.x <= self.b.x && p.y <= self.b.y
    }
}

impl Screen {
    pub fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let m_pos = mouse::position(ctx);
        let position = na::Point2::new(m_pos.x, m_pos.y);
        let is_pressed = mouse::button_pressed(ctx, MouseButton::Left);

        for i in 0..self.components.len() {
            self.components[i].update(position, is_pressed);
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        for c in self.components.iter() {
            c.draw(ctx);
        }
    }

    pub fn add_button(&mut self, ctx: &mut Context, rect: Rect, label_text: String, label_size: f32, font: Font, color: Color, color_hover: Color) {
        let button = Button::new(ctx, rect, label_text, label_size, font, color, color_hover);
        self.components.push(Box::new(button));
    }
}

impl Draw for Button {
    fn update(&mut self, position: Point, is_pressed: bool) {
        let is_inside = self.b_box.is_inside(&position);
        self.mouse_state.update(is_inside, is_pressed);
    }

    fn draw(&self, ctx: &mut Context) {
        if self.mouse_state.hovered {
            graphics::draw(ctx, &self.border.1, (self.b_box.a,)).unwrap();
            graphics::draw(ctx, &self.label.1, (self.label_offset,)).unwrap();
        } else {
            graphics::draw(ctx, &self.border.0, (self.b_box.a,)).unwrap();
            graphics::draw(ctx, &self.label.0, (self.label_offset,)).unwrap();
        }
    }
}

impl Button {
    pub fn new(ctx: &mut Context, rect: Rect, label_text: String, label_size: f32, font: Font, color: Color, color_hover: Color) -> Self {
        let mouse_state = MouseState::new();
        let tf = TextFragment::from((label_text.clone(), font, label_size)).color(color);
        let label = Text::new(tf);
        let tf_hover = TextFragment::from((label_text, font, label_size)).color(color_hover);
        let label_hover = Text::new(tf_hover);
        let label_offset = centering_offset(&label, ctx, &rect);

        let border = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            Rect::new(0.0,0.0, rect.w, rect.h),
            color).unwrap();
        let border_hover = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            Rect::new(0.0,0.0, rect.w, rect.h),
            color_hover).unwrap();
        let b_box = BBox::from_rect(rect);
        Self {
            b_box: b_box, 
            mouse_state: mouse_state, 
            label_offset: label_offset, 
            label: (label, label_hover), 
            border: (border, border_hover)}
    }
}

fn centering_offset(label: &Text, ctx: &mut Context, rect: &Rect) -> Point {
    Point::new(
        vertical_offset(label, ctx, rect), 
        horizontal_offset(label, ctx, rect))
}

fn vertical_offset(label: &Text, ctx: &mut Context, rect: &Rect) -> f32 {
    rect.x + (rect.w / 2.0 - label.width(ctx) as f32 / 2.0)
}

fn horizontal_offset(label: &Text, ctx: &mut Context, rect: &Rect) -> f32 {
    rect.y + (rect.h / 2.0 - label.height(ctx) as f32 / 2.0)
}