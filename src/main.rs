mod window_state;

use window_state::WindowState;
use ggez::{GameResult, event, conf::FullscreenType};

const WINDOW_WIDTH : f32 = 400.0;
const ASPECT_RATIO: f32 = 16.0/9.0;
const WINDOW_HEIGHT: f32 = WINDOW_WIDTH/ASPECT_RATIO; 


fn run_ggez_renderer() -> GameResult {
    let win_mode = ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT);
    let cb = ggez::ContextBuilder::new("Rust Raytrace", "FleeXo").window_mode(win_mode);

    let (ctx, event_loop) = cb.build()?;
    let state = WindowState::new(ASPECT_RATIO)?;
    event::run(ctx, event_loop, state)
}
pub fn main() -> GameResult {
    run_ggez_renderer()
}