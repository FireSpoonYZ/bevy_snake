use crate::*;
use bevy::prelude::*;

pub fn game_over(
    mut timer: ResMut<MainTimer>,
    query: Query<&Block, (With<SnakeHead>, Changed<Block>)>,
    wall_query: Query<&Block, (With<Wall>, Without<SnakeHead>)>,
    body_query: Query<&Block, (With<SnakeBody>, Without<SnakeHead>, Without<Wall>)>
) {
    if timer.0.just_finished() {
        for block in query.iter() {
            for wall_block in wall_query.iter().chain(body_query.iter()) {
                if block.x == wall_block.x && block.y == wall_block.y {
                    println!("Game Over!");
                    timer.0.pause();
                    return;
                }
            }
        }
    }
}
