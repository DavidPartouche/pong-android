extern crate amethyst;

mod bundle;
mod pong;
mod systems;

use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::core::transform::TransformBundle;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage};
#[cfg(not(target_os = "android"))]
use amethyst::utils::application_root_dir;

use bundle::PongBundle;
use std::time::Duration;

const ARENA_HEIGHT: f32 = 100.0;
const ARENA_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_WIDTH: f32 = 4.0;
const PADDLE_VELOCITY: f32 = 75.0;

const BALL_RADIUS: f32 = 2.0;

const SPRITESHEET_SIZE: (f32, f32) = (8.0, 16.0);

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use pong::Pong;

    #[cfg(not(target_os = "android"))]
    let assets_dir = format!("{}/assets/", application_root_dir());
    #[cfg(target_os = "android")]
    let assets_dir = "";

    let display_config_path = format!("{}resources/display.ron", assets_dir);
    let config = DisplayConfig::load(&display_config_path);

    let key_bindings_path = {
        if cfg!(feature = "sdl_controller") {
            format!("{}resources/input_controller.ron", assets_dir)
        } else {
            format!("{}resources/input.ron", assets_dir)
        }
    };

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?,
        )?.with_bundle(PongBundle)?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new().with_dep(&["paddle_system"]))?;
    let mut game = Application::build(assets_dir, Pong)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        ).build(game_data)?;
    game.run();

    Ok(())
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub velocity: f32,
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        Paddle {
            velocity: 1.0,
            side: side,
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
