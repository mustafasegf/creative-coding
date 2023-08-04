use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points: Vec<Point2>,
    egui: Egui,
    len: f32,
}

fn model(app: &App) -> Model {
    let w = 512;
    let h = w;
    let window_id = app
        .new_window()
        .size(w, h)
        .title("hilbert curve")
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    let order = 4;
    let n = pow(2, order) as usize;
    let total = n * n;
    let len = w as f32 / n as f32;

    let points = (0..total)
        .map(|i| hilbert(i, order) * len - vec2(len * n as f32 / 2.0, len * n as f32 / 2.0))
        .collect();

    Model {
        egui,
        points,
        len: w as f32 / (2 * n) as f32,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("len:");
        ui.add(egui::Slider::new(&mut model.len, -512.0..=512.0));
    });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn hilbert(i: usize, order: usize) -> Point2 {
    let points = [pt2(0.0, 0.0), pt2(0.0, 1.0), pt2(1.0, 1.0), pt2(1.0, 0.0)];

    let mut index = i & 3;
    let mut v = points[index];

    for j in 1..order {
        index = (i >> (2 * j as usize)) & 3;
        let len = pow(2, j) as f32;
        match index {
            0 => (v.x, v.y) = (v.y, v.x),
            1 => (v.x, v.y) = (v.x, v.y + len),
            2 => (v.x, v.y) = (v.x + len, v.y + len),
            3 => (v.x, v.y) = (2.0 * len - 1.0 - v.y, len - 1.0 - v.x),
            _ => unreachable!(),
        };
    }
    v
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let len = model.points.len();
    let t = app.time * 100.0 % (len as f32) * 2.0;
    let skip = (t as i32 - len as i32).max(0) as usize % len;
    // println!("{skip}");

    let points = model
        .points
        .clone()
        .into_iter()
        .skip(skip)
        .take(t as usize % (2 * len))
        .enumerate()
        .map(|(i, p)| {
            let fract = i as f32 / (len - skip) as f32;
            let r = fract % 1.0;
            let g = (1.0 - fract) % 1.0;
            let b = (0.5 + fract) % 1.0;
            let rgba = srgba(r, g, b, 1.0);
            (p, rgba)
        });

    draw.polyline()
        .weight(10.0)
        .join_round()
        .x_y(model.len, model.len)
        .points_colored(points);

    draw.to_frame(app, &frame).unwrap();
    // model.egui.draw_to_frame(&frame).unwrap();
}
