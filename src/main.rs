use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .simple_window(view)
        .run();
}

struct Model;

fn model(_app: &App) -> Model {
    let cur_monitor = _app.main_window().current_monitor();
    // Go fullscreen
    // _app.main_window()
    //     .set_fullscreen(Some(cur_monitor));

    Model
}

fn view(app: &App, _model: &Model, frame: &Frame) {
    let draw = app.draw();

    let t = app.time * 0.1;
    let win = app.window_rect();

    // Circle density
    let step = 3;

    let mid_to_corner = (win.xy().distance(win.top_right()) as usize) + step;

    // Fill center
    draw.ellipse()
        .radius(5.0)
        .color(hsl(t / 1000.0, 1.0, 0.4));

    for i in (1..mid_to_corner).step_by(step) {
        let i = i as f32;

        draw.ellipse()
            .radius(i)
            .stroke(hsl(t - i / 1000.0, 1.0, 0.4))
            .no_fill()
            .stroke_weight(step as f32);
    }

    draw.to_frame(app, &frame).unwrap();
}
