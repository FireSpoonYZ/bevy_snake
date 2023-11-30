use crate::*;
use bevy::prelude::*;

pub fn move_head(
    time: Res<Time>,
    mut timer: ResMut<MainTimer>,
    mut query: Query<(&mut Block, &mut Volocity), With<SnakeHead>>,
    mut body: ResMut<SnakeBody>,
    mut commands: Commands,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mut block, volocity) in query.iter_mut() {
            body.0.push_front(
                spawn_block(&mut commands, Color::WHITE, block.x, block.y)
                    .insert(Wall)
                    .id(),
            );
            body.1 = body.0.pop_back();

            block.x += volocity.x;
            block.y += volocity.y;
        }
    }
}
