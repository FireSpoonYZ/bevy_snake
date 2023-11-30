use crate::*;
use bevy::prelude::*;

pub fn keyboard_input(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Block, &mut Volocity)>,
    body_query: Query<(Entity, &Block), (With<SnakeBody>, Without<Volocity>)>,
    body: ResMut<SnakeBodys>,
) {
    for (block, mut volocity) in query.iter_mut() {
        let origin_volocity = *volocity;

        if input.just_pressed(KeyCode::Left) {
            volocity.x = -1;
            volocity.y = 0;
        }
        if input.just_pressed(KeyCode::Right) {
            volocity.x = 1;
            volocity.y = 0;
        }
        if input.just_pressed(KeyCode::Up) {
            volocity.x = 0;
            volocity.y = 1;
        }
        if input.just_pressed(KeyCode::Down) {
            volocity.x = 0;
            volocity.y = -1;
        }

        if let Some(first_id) = body.0.front() {
            for (entity, body) in body_query.iter() {
                if entity == *first_id {
                    if body.x == block.x + volocity.x && body.y == block.y + volocity.y {
                        *volocity = origin_volocity;
                    }
                }
            }
        }
    }
}
