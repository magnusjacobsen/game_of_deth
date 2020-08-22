
use ggez::Context;
use ggez::input::{keyboard, mouse};
use ggez::event::MouseButton;
use ggez::graphics::{self, Color};

use std::collections::HashMap;

use crate::state::playstate::PlayState;
use crate::{CELL_WIDTH, CELL_TOTAL};

pub fn update_key_activity(ctx: &mut Context, state: &mut PlayState) {
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

pub fn update_mouse_activity(ctx: &mut Context, state: &mut PlayState) {
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
    colors.push(Color::from_rgb(0, 84, 163));
    colors.push(Color::from_rgb(192, 49, 33));
    colors.push(Color::from_rgb(194, 101, 57));
    colors.push(Color::from_rgb(241, 118, 15));
    colors.push(Color::from_rgb(234, 182, 21));
    colors.push(Color::from_rgb(238, 214, 44));
    colors.push(Color::from_rgb(211, 215, 30));
    colors.push(Color::from_rgb(166, 177, 55));
    colors.push(Color::from_rgb(124, 185, 56));
    colors.push(Color::from_rgb(64, 161, 66));
    colors.push(Color::from_rgb(82, 180, 79));
    colors.push(Color::from_rgb(20, 159, 40));
    colors.push(Color::from_rgb(0, 144, 94));
    colors.push(Color::from_rgb(27, 146, 150));
    colors.push(Color::from_rgb(0, 149, 182));
    colors.push(Color::from_rgb(4, 153, 186));
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

pub fn calculate_offset(cells: &HashMap<(i64,i64),usize>, ctx: &mut Context) -> (f32, f32) {
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
    
    let mut diff_x = (minx + (maxx - minx) / 2.0).floor();
    let mut diff_y = (miny + (maxy - miny) / 2.0).floor();
    diff_x = if diff_x.is_finite() { diff_x } else { 0.0 };
    diff_y = if diff_y.is_finite() { diff_y } else { 0.0 };
    let dim_x = ((dim.0 / CELL_TOTAL) / 2.0).floor();
    let dim_y = ((dim.1 / CELL_TOTAL) / 2.0).floor();
    let offset_x = (dim_x - diff_x) * CELL_TOTAL;
    let offset_y = (dim_y - diff_y) * CELL_TOTAL;
    (offset_x, offset_y)
}