use {
    rg3d::{
        engine::Engine,
        engine::framework::prelude::*,
        event_loop::ControlFlow,
        core::color::{Color, Hsv},
    },
};

#[derive(Default)]
struct Game {
    hue: f32,
}

impl GameState for Game {
    fn init(_engine: &mut Engine) -> Self
    where Self: Sized
    {
        Self::default()
    }
    fn on_tick(&mut self, engine: &mut Engine, dt: f32, _: &mut ControlFlow) {
        self.hue += 24.0 * dt;
        engine
            .renderer
            .set_backbuffer_clear_color(Color::from(Hsv::new(self.hue % 360.0, 100.0, 100.0)));
    }
}

fn main() {
    println!("Hello, world!");
    Framework::<Game>::new()
        .unwrap()
        .title("Simple")
        .run();
}
