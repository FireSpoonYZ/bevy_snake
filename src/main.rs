pub mod component;
pub mod entity;
pub mod resource;
pub mod system;

pub use component::*;
pub use entity::*;
pub use resource::*;
pub use system::*;

use bevy::{prelude::*, window::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".to_string(),
                resolution: WindowResolution::new(270.0, 270.0).with_scale_factor_override(1.0),
                resizable: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(MainTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
        .init_resource::<SnakeBodys>()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_systems(Startup, init)
        .add_systems(
            Update,
            (
                render_blocks,
                flush_food,
                move_head,
                keyboard_input,
                eat_food,
                game_over,
            ),
        )
        .run();
}

fn init(mut commands: Commands) {
    // setup camera
    commands.spawn(Camera2dBundle::new_with_far(5.0));

    // spawn snake head
    spawn_block(&mut commands, Color::RED, 0, 0)
        .insert(SnakeHead)
        .insert(Volocity { x: 0, y: 1 });

    // spawn wall
    for i in -13..14 {
        for j in -13..14 {
            if i == -13 || i == 13 || j == -13 || j == 13 {
                spawn_block(&mut commands, Color::WHITE, i, j).insert(Wall);
            }
        }
    }

    // spawn food
    spawn_block(&mut commands, Color::BLUE, 0, 0).insert(Food::new());
}
