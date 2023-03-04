mod physics;

use std::time::Instant;
use physics::*;
use nannou::{
    prelude::*,
    draw::mesh::vertex::Color
};

// 1 meter = `METER_TO_PIXEL_RATIO` pixels
const METER_TO_PIXEL_RATIO: f32 = 100.0;
const RADIUS: f32 = 20.0;
const GRAVITY: f32 = -9.81;
const _RESTITUTION_COEFFICIENT: f32 = 0.85;
const LAUNCH_STRENGTH: f32 = 15.0;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

// A model describing the internal state.
struct Model {
    physics_world: PhysicsWorld,
    last_update: Instant,
    line_start: Point2,
    line_end: Point2
}


fn model(_app: &App) -> Model {
    _app.window(_app.window_id()).unwrap().set_title("phys 1");

    let window_dimensions = _app.window_rect().w_h().into();
    Model {
        physics_world: PhysicsWorld::new( 
            GRAVITY,
            window_dimensions,
            METER_TO_PIXEL_RATIO
        ),
        last_update: Instant::now(),
        line_start: Point2::ZERO,
        line_end: Point2::ZERO
    }
}


fn update(_app: &App, model: &mut Model, _update: Update) {

    // Check if the window dimensions have changed
    let new_dimensions = _app.window_rect().w_h().into();
    model.physics_world.set_bounds(new_dimensions);

    // Get delta time
    let now = Instant::now();
    let delta_time = now.duration_since(model.last_update).as_secs_f32();
    
    model.physics_world.step(delta_time);

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
            MouseMoved(position) => {
                mouse_moved_event(model, position);
            }
            MouseReleased(button) => {
                mouse_released_event(&app, model, button);
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
        let mouse_position = app.mouse.position();
        model.line_start = mouse_position;
    }
}

fn mouse_moved_event(model: &mut Model, position: Point2) {
    model.line_end = position;
}

fn mouse_released_event(app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left {

        let offset = (random_f32()*RADIUS*0.5)-(RADIUS*0.25);
        let mouse_position = app.mouse.position();
        
        // Unused while physics is being reworked
        let impulse: Vec2 = (model.line_start - model.line_end).into();
        let multiplied_impulse = impulse*LAUNCH_STRENGTH;

        let id1 = model.physics_world.next_id();
        let id2 = model.physics_world.next_id();

        let circle1 = Particle::new(
            model.line_start.clone(),
            0.0,
            RADIUS+offset,
            generate_random_colour(),
            id1
        );
        
        let circle2 = Particle::new(
            model.line_end.clone(),
            10.0,
            RADIUS+offset,
            generate_random_colour(),
            id2
        );
        
        model.physics_world.add_object(circle1);
        model.physics_world.add_object(circle2);
        let stick = Stick {
            id_1: id1,
            id_2: id2,
            distance: 100.0,
        };
        model.physics_world.add_stick(stick)

    }
}

fn key_pressed_event(model: &mut Model, key: Key) {
    match key {
        Key::X => {
            model.physics_world.clear()
        },
        Key::Space => {
            model.physics_world.add_impulses(20000.0*random_f32()-10000.0, 20000.0*random_f32()-10000.0)
        }
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

    draw_sticks(&draw, model);
    draw_circles(&draw, model);

    draw_shoot_overlay(app, &draw, model);
    
    draw_mouse_label(&app, &draw);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_grid(draw: &Draw, window_rect: &Rect, step: f32, weight: f32) {
    let step_by = || (0..).map(|i| i as f32 * step);
    let r_iter = step_by().take_while(|&f| f < window_rect.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > window_rect.left());

    let x_iter = r_iter.chain(l_iter);
    for x in x_iter.clone() {
        draw.line()
            .weight(weight)
            .points(pt2(x, window_rect.bottom()), pt2(x, window_rect.top()));
    }

    let t_iter = step_by().take_while(|&f| f < window_rect.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > window_rect.bottom());
    let y_iter = t_iter.chain(b_iter);
    for (_i, y) in y_iter.enumerate() {
        draw.line()
            .weight(weight)
            .points(pt2(window_rect.left(), y), pt2(window_rect.right(), y));

        //if alternate_shade {
        //    for (j, x) in x_iter.clone().enumerate() {
        //        //if (_i + j) % 2 == 1 {
        //            draw.rect()
        //                .color(rgba(1.0, 1.0, 1.0, 0.1))
        //                .stroke_weight(0.0)
        //                .x_y(x+step/2.0, y+step/2.0)
        //                .w_h(step, step);
        //        //}
        //    }
        //}

    }
}

fn _draw_grid_squares(draw: &Draw, window_rect: &Rect, step: f32) {
    let x_square_iter = (0..).map(|i| i as f32 * step - step / 2.0).take_while(|&f| f < window_rect.right());
    let y_square_iter = (0..).map(|i| i as f32 * step - step / 2.0).take_while(|&f| f < window_rect.top());
    for (i, y) in y_square_iter.enumerate() {
        for (j, x) in x_square_iter.clone().enumerate() {
            if (i + j) % 2 == 1 {
                draw.rect()
                    .color(rgba(1.0, 1.0, 1.0, 0.1))
                    .stroke_weight(0.0)
                    .x_y(x, y)
                    .w_h(step, step);
            }
        }
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
    let top = format!("{:.3}m", window_rect.top()/METER_TO_PIXEL_RATIO);
    let bottom = format!("{:.3}m", window_rect.bottom()/METER_TO_PIXEL_RATIO);
    let left = format!("{:.3}m", window_rect.left()/METER_TO_PIXEL_RATIO);
    let right = format!("{:.3}m", window_rect.right()/METER_TO_PIXEL_RATIO);
    let x_off = 30.0;
    let y_off = 20.0;
    draw.text("0, 0")
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
    for particle in model.physics_world.get_particles() {
        draw.ellipse()
            .x_y(particle.pos.x, particle.pos.y)
            .w_h(particle.radius * 2.0, particle.radius * 2.0)
            .color(particle.colour);
    }
}

fn draw_sticks(draw: &Draw, model: &mut Model) {
    let sticks = model.physics_world.get_sticks();
    for stick in sticks {
        if let Some(particle1) = model.physics_world.get_particle_by_id(stick.id_1) {
            if let Some(particle2) = model.physics_world.get_particle_by_id(stick.id_2) {
                draw.line()
                    .weight(2.0)
                    .points(particle1.pos, particle2.pos);
            } else {};
        } else {}; 
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

fn draw_shoot_overlay(app: &App, draw: &Draw, model: &Model) {
    if app.mouse.buttons.left().is_down() {
        draw.ellipse()
            .wh([5.0; 2].into())
            .xy(model.line_start)
            .color(BLUEVIOLET);
        draw.line()
            .start(model.line_start)
            .end(model.line_end)
            .weight(2.0)
            .color(BLUEVIOLET);
    }
}


fn generate_random_colour() -> Color {
    let red = random();
    let green = random();
    let blue = random();
    return Color::new(red, green, blue, 1.0)
}