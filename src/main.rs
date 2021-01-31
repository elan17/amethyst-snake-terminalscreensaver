use amethyst::{
    prelude::*,
    core::frame_limiter::FrameRateLimitStrategy::{
        Yield,
        Sleep,
        Unlimited
    },
    utils::application_root_dir
};

mod systems;
mod components;
mod entities;
mod resources;


use components::position::*;
use systems::{
    renderer::*,
    move_snakes::*,
    clear_dead_snakes::*,
    generate_snakes::*
};
use resources::grid::*;
use entities::head::*;

pub struct Snakes;
impl SimpleState for Snakes{}

fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir()?;

    let mut renderer = CursesRenderer::new();
    let dim = {
        renderer.get_window_dimensions()
    };

    let game_data = GameDataBuilder::default()
        .with_thread_local(CursesRenderer::new())
        .with(SnakeGenerator::new(), "snake_generator", &[])
        .with(MoveSnakes::new(), "move_snakes", &["snake_generator"])
        .with(ClearDeadSnakesSystem, "dead_snakes", &["move_snakes"])
        ;

    let assets_dir = app_root.join("assets");
    let mut game = Application::build(assets_dir, Snakes)?
        .with_resource(Grid::new(dim))
        .register::<Position>()
        .register::<Head>()
        .with_frame_limit(Sleep, 10)
        .build(game_data)?;
    game.run();

    Ok(())
}
