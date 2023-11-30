use bevy::prelude::*;
use crate::*;

pub fn eat_food(
    query: Query<&Block, (With<SnakeHead>, Changed<Block>)>,
    mut food_query: Query<(&mut Food, &Block), Without<SnakeHead>>,
    mut body: ResMut<SnakeBody>,
    mut commands: Commands,
) {
    for block in query.iter() {
        for (mut food, food_block) in food_query.iter_mut() {
            if food_block.x == block.x && food_block.y == block.y {
                food.need_flush = true;
                if let Some(last) = body.1.take() {
                    body.0.push_back(last);
                }
            } else {
                if let Some(last) = body.1.take() {
                    commands.entity(last).despawn();
                }
            }
        }
    }
}
