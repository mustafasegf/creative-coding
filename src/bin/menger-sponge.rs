use nannou::prelude::*;
use nannou::winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    boxes: Vec<Cuboid>,

    x: f32,
    y: f32,
    z: f32,
    auto_rotate: bool,
}

fn model(app: &App) -> Model {
    let w = 512;
    let h = w;
    app.new_window()
        .size(w, h)
        .title("rotating cube")
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let boxes = vec![Cuboid::from_x_y_z_w_h_d(0.0, 0.0, 0.0, 200.0, 200.0, 200.0)];

    Model {
        boxes,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        auto_rotate: true,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    if model.auto_rotate {
        // model.x = 1.00 * update.since_start.as_secs_f32();
        // model.y = 1.15 * update.since_start.as_secs_f32();
        // model.z = 1.30 * update.since_start.as_secs_f32();
        model.x += 0.0050;
        model.y += 0.0030;
        model.z += 0.0090;
    }
}

fn raw_window_event(app: &App, model: &mut Model, event: &WindowEvent) {
    match event {
        WindowEvent::CursorMoved { position, .. } if !model.auto_rotate => {
            let win = app.window_rect();
            model.y = map_range(position.x as f32, 0.0, win.w(), -PI, PI);
            model.x = map_range(position.y as f32, 0.0, win.h(), -PI, PI);
        }
        WindowEvent::MouseWheel { delta, .. } if !model.auto_rotate => {
            if let MouseScrollDelta::LineDelta(_, y) = delta {
                model.z += y * 0.1;
            }
        }
        WindowEvent::MouseInput { state, button, .. } => {
            if *state == ElementState::Pressed {
                match button {
                    MouseButton::Left => {
                        model.boxes = model.boxes.iter().flat_map(create_boxes).collect();
                    }
                    MouseButton::Right => {
                        model.auto_rotate = !model.auto_rotate;
                    }
                    MouseButton::Middle => {
                        model.boxes =
                            vec![Cuboid::from_x_y_z_w_h_d(0.0, 0.0, 0.0, 100.0, 100.0, 100.0)];
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn create_boxes(cuboid: &Cuboid) -> Vec<Cuboid> {
    let (x, y, z, w, h, d) = cuboid.x_y_z_w_h_d();

    let mut boxes = Vec::new();

    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                if abs(i) + abs(j) + abs(k) > 1 {
                    let (w, h, d) = (w / 3.0, h / 3.0, d / 3.0);
                    let (x, y, z) = (x + i as f32 * w, y + j as f32 * h, z + k as f32 * d);
                    boxes.push(Cuboid::from_x_y_z_w_h_d(x, y, z, w, h, d));
                }
            }
        }
    }

    boxes
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    let rot = vec3(model.x, model.y, model.z);

    model.boxes.iter().enumerate().for_each(|(i, b)| {
        let wpoints = b.triangles_iter().flat_map(geom::Tri::vertices).map(|p| {
            let col = hsva(
                map_range(i, 0, model.boxes.len() - 1, 0.0, 1.0),
                1.0,
                1.0,
                0.7,
            );
            // let col = rgba(1.0, 1.0, 1.0, 0.3);
            (p, col)
        });

        draw.radians(rot).mesh().points_colored(wpoints);
    });

    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);

    draw.to_frame(app, &frame).unwrap();
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
