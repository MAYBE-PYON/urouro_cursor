use enigo::*;
use rand::distributions::{Distribution, Uniform};
use std::thread;
use std::time::Duration;
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let mut enigo = Enigo::new();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_visible(false).build(&event_loop).unwrap();

    if let Some(mon) = window.primary_monitor() {
        loop {
            let mut rng = rand::thread_rng();
            let wdie = Uniform::from(0..mon.size().width);
            let x_pos = wdie.sample(&mut rng);
            let hdie = Uniform::from(0..mon.size().height);
            let y_pos = hdie.sample(&mut rng);
            enigo.mouse_move_to(x_pos as i32, y_pos as i32);

            // 待機
            thread::sleep(Duration::from_secs(10));
        }
    }
}
