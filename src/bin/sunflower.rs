use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    angle: f32,
    n: f32,
    constant: f32,
    points: Vec<Point>,
}

struct Point {
    x: f32,
    y: f32,
    phi: f32,
    radius: f32,
}

fn model(app: &App) -> Model {
    let w = 512;
    let h = w;
    app.new_window()
        .size(w, h)
        .title("sunflower")
        .view(view)
        .build()
        .unwrap();

    Model {
        angle: 137.5,
        n: 0.0,
        constant: 7.0,
        points: Vec::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.n += 1.0;

    let phi = model.n * model.angle.to_radians();
    let radius = model.constant * (phi).sqrt();

    let x = radius * phi.cos();
    let y = radius * phi.sin();

    model.points.push(Point { x, y, phi, radius });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for (i, p) in model.points.iter().enumerate() {
        draw.ellipse().x_y(p.x, p.y).radius(4.0).color(hsv(
            map_range(i % 256, 0, 255, 0.0, 1.0),
            1.0,
            1.0,
        ));
    }
    draw.to_frame(app, &frame).unwrap();

    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}
