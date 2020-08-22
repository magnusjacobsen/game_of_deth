use ggez::Context;

trait Widget {
    fn new(ui: Ui) {
        
    }
    fn draw(&self, &mut Context);
}

struct Position {

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