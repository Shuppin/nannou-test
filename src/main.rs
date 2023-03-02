use std::time::Instant;

use nannou::prelude::*;

// 1 meter = `METER_TO_PIXEL_RATIO` pixels
const METER_TO_PIXEL_RATIO: f32 = 300.0;
const RADIUS: f32 = 20.0;
const GRAVITY: f32 = -9.81;
const RESTITUTION_COEFFICIENT: f32 = 0.85;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

struct Circle {
    vel: Vec2,
    pos: Vec2,
    radius: f32,
}

// A model describing the internal state.
struct Model {
    circles: Vec<Circle>,
    last_update: Instant,
    window_dimensions: Vec2,
}


fn model(_app: &App) -> Model {
    let (width, height) = _app.window_rect().w_h();
    Model {
        circles: Vec::new(),
        last_update: Instant::now(),
        window_dimensions: Vec2::new(width, height)
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let now = Instant::now();
    let delta_time = now.duration_since(model.last_update).as_secs_f32();

    // Update each circle's position based on elapsed time and acceleration
    for circle in &mut model.circles {
        circle.pos.y += circle.vel.y * delta_time + 0.5 * GRAVITY*METER_TO_PIXEL_RATIO * delta_time.powi(2);
        circle.vel.y += GRAVITY*METER_TO_PIXEL_RATIO * delta_time;

        // Check if the circle has hit the bottom of the screen
        if circle.pos.y - circle.radius < -model.window_dimensions.y / 2.0 {
            circle.pos.y = -model.window_dimensions.y / 2.0 + circle.radius;
            circle.vel.y *= -RESTITUTION_COEFFICIENT;
        }

    }

    model.last_update = now;

}

// A controller describing how to update the model on certain events.
fn event(app: &App, model: &mut Model, event: Event) {

    match event {
        Event::WindowEvent { simple: None, .. } => (),
        Event::WindowEvent { simple: Some(event), .. } => match event {
            MousePressed(button) => {
                mouse_pressed_event(&app, model, button);
            }
            KeyPressed(key) => {
                key_pressed_event(model, key);
            }
            _ => (),
        },
        _ => (),
    }

}

fn mouse_pressed_event(app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left {
        // The left mouse button is pressed
        println!("Left mouse button pressed!");
        let mouse_position = app.mouse.position();
        let circle = Circle {
            vel: Vec2::new(0.0, 0.0),
            pos: Vec2::new(
                mouse_position.x,
                mouse_position.y
            ),
            radius: RADIUS,
        };
        model.circles.push(circle);
    }
}

fn key_pressed_event(model: &mut Model, key: Key) {
    match key {
        Key::X => {
            model.circles = Vec::new()
        },
        _ => ()
    }
}

// A view describing how to present the model.
fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let window = app.main_window();
    let window_rect = window.rect();
    
    // Set background colour
    draw.background().rgb(0.11, 0.12, 0.13);

    // Draw the major and minor grid
    draw_grid(&draw, &window_rect, 100.0, 1.0);
    draw_grid(&draw, &window_rect, 25.0, 0.5);

    // Draw axis things
    draw_axis_lines(&draw, &window_rect);
    draw_axis_labels(&draw, &window_rect);

    draw_circles(&draw, model);
    
    draw_mouse_label(&app, &draw);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_grid(draw: &Draw, window_rect: &Rect, step: f32, weight: f32) {
    let step_by = || (0..).map(|i| i as f32 * step);
    let r_iter = step_by().take_while(|&f| f < window_rect.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > window_rect.left());

    let x_iter = r_iter.chain(l_iter);
    for x in x_iter {
        draw.line()
            .weight(weight)
            .points(pt2(x, window_rect.bottom()), pt2(x, window_rect.top()));
    }
    let t_iter = step_by().take_while(|&f| f < window_rect.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > window_rect.bottom());
    let y_iter = t_iter.chain(b_iter);
    for y in y_iter {
        draw.line()
            .weight(weight)
            .points(pt2(window_rect.left(), y), pt2(window_rect.right(), y));
    }
}

fn draw_axis_lines(draw: &Draw, window_rect: &Rect) {
    let line_colour = gray(0.5);
    let ends = [
        window_rect.mid_top(),
        window_rect.mid_right(),
        window_rect.mid_bottom(),
        window_rect.mid_left(),
    ];
    for &end in &ends {
        draw.arrow()
            .start_cap_round()
            .head_length(16.0)
            .head_width(8.0)
            .color(line_colour)
            .end(end);
    }
}

fn draw_axis_labels(draw: &Draw, window_rect: &Rect) {
    // Crosshair text.
    let line_colour = gray(0.5);
    let top = format!("{:.1}m", window_rect.top()/METER_TO_PIXEL_RATIO);
    let bottom = format!("{:.1}m", window_rect.bottom()/METER_TO_PIXEL_RATIO);
    let left = format!("{:.1}m", window_rect.left()/METER_TO_PIXEL_RATIO);
    let right = format!("{:.1}m", window_rect.right()/METER_TO_PIXEL_RATIO);
    let x_off = 30.0;
    let y_off = 20.0;
    draw.text("0.0")
        .x_y(15.0, 15.0)
        .color(line_colour)
        .font_size(14);
    draw.text(&top)
        .h(window_rect.h())
        .font_size(14)
        .align_text_top()
        .color(line_colour)
        .x(x_off);
    draw.text(&bottom)
        .h(window_rect.h())
        .font_size(14)
        .align_text_bottom()
        .color(line_colour)
        .x(x_off);
    draw.text(&left)
        .w(window_rect.w())
        .font_size(14)
        .left_justify()
        .color(line_colour)
        .y(y_off);
    draw.text(&right)
        .w(window_rect.w())
        .font_size(14)
        .right_justify()
        .color(line_colour)
        .y(y_off);
}

fn draw_circles(draw: &Draw, model: &Model) {
    // Draw all the circles in the model
    for circle in &model.circles {
        draw.ellipse()
            .x_y(circle.pos.x, circle.pos.y)
            .w_h(circle.radius * 2.0, circle.radius * 2.0)
            .color(RED);
    }
}

fn draw_mouse_label(app: &App, draw: &Draw) {
    let mouse_pos = app.mouse.position();
    
    // Ellipse at mouse.
    if app.mouse.buttons.left().is_down() {
        draw.ellipse()
            .wh([5.0; 2].into())
            .xy(mouse_pos)
            .color(BLUEVIOLET);
    } else {
        draw.ellipse()
            .wh([5.0; 2].into())
            .xy(mouse_pos);
    }

    // Mouse position text.
    let pos = format!("[{}, {}]", mouse_pos.x, mouse_pos.y);
    draw.text(&pos)
        .xy(mouse_pos + vec2(0.0, 20.0))
        .font_size(14)
        .color(WHITE);
}