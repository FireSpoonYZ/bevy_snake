use bevy::prelude::*;
use crate::*;

pub fn game_over(
    mut timer: ResMut<MainTimer>,
    mut query: Query<&Block, (With<SnakeHead>, Changed<Block>)>,
    wall_query: Query<&Block, (With<Wall>, Without<SnakeHead>)>,
) {
    if timer.0.just_finished() {
        for block in query.iter_mut() {
            for wall_block in wall_query.iter() {
                if block.x == wall_block.x && block.y == wall_block.y {
                    println!("Game Over!");
                    timer.0.pause();
                    return;
                }
            }
        }
    }
}