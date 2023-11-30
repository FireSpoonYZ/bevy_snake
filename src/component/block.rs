use bevy::{prelude::*, ecs::system::EntityCommands};

#[derive(Debug, Component)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

pub fn spawn_block<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    color: Color,
    x: i32,
    y: i32,
) -> EntityCommands<'w, 's, 'a> {
    let mut ret = commands.spawn(SpriteBundle {
        sprite: Sprite {
            // color,
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..Default::default()
        },
        visibility: Visibility::Hidden,
        ..Default::default()
    });
    ret.insert(Block { x, y, color });
    ret
}
