use nannou::prelude::*;
use nannou::winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    cuboid: Cuboid,
    wireframe: Vec<Cuboid>,
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

    let cuboid = Cuboid::from_x_y_z_w_h_d(0.0, 0.0, 0.0, 100.0, 100.0, 100.0);

    let wireframe = create_wireframe(&cuboid);
    Model {
        cuboid,
        wireframe,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        auto_rotate: true,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    if model.auto_rotate {
        model.x = 1.00 * update.since_start.as_secs_f32();
        model.y = 1.15 * update.since_start.as_secs_f32();
        model.z = 1.30 * update.since_start.as_secs_f32();
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
            if *button == MouseButton::Left && *state == ElementState::Pressed {
                model.auto_rotate = !model.auto_rotate;
            }
        }
        _ => {}
    }
}

fn create_wireframe(cuboid: &Cuboid) -> Vec<Cuboid> {
    let x = cuboid.x();
    let y = cuboid.y();
    let z = cuboid.z();

    let ww = cuboid.w();
    let hh = cuboid.h();
    let dd = cuboid.d();

    let xx = ww * 0.5;
    let yy = hh * 0.5;
    let zz = dd * 0.5;

    let w = 5.0; // wire width
    vec![
        //top
        Cuboid::from_x_y_z_w_h_d(x + -xx, y + yy, z + 0.0, w, w, dd),
        Cuboid::from_x_y_z_w_h_d(x + 0.0, y + yy, z + zz, ww, w, w),
        Cuboid::from_x_y_z_w_h_d(x + xx, y + yy, z + 0.0, w, w, dd),
        Cuboid::from_x_y_z_w_h_d(x + 0.0, y + yy, z + -zz, ww, w, w),
        // bottom
        Cuboid::from_x_y_z_w_h_d(x + -xx, y + -yy, z + 0.0, w, w, dd),
        Cuboid::from_x_y_z_w_h_d(x + 0.0, y + -yy, z + zz, ww, w, w),
        Cuboid::from_x_y_z_w_h_d(x + xx, y + -yy, z + 0.0, w, w, dd),
        Cuboid::from_x_y_z_w_h_d(x + 0.0, y + -yy, z + -zz, ww, w, w),
        // sides
        Cuboid::from_x_y_z_w_h_d(x + -xx, y + 0.0, z + -zz, w, hh, w),
        Cuboid::from_x_y_z_w_h_d(x + -xx, y + 0.0, z + zz, w, hh, w),
        Cuboid::from_x_y_z_w_h_d(x + xx, y + 0.0, z + zz, w, hh, w),
        Cuboid::from_x_y_z_w_h_d(x + xx, y + 0.0, z + -zz, w, hh, w),
    ]
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    let rot = vec3(model.x, model.y, model.z);

    model.wireframe.iter().enumerate().for_each(|(i, w)| {
        let wpoints = w.triangles_iter().flat_map(geom::Tri::vertices).map(|p| {
            let col = hsv(map_range(i, 0, 12, 0.0, 1.0), 1.0, 1.0);
            (p, col)
        });

        draw.radians(rot).mesh().points_colored(wpoints);
    });

    draw.to_frame(app, &frame).unwrap();
}
