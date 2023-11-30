use bevy::prelude::*;

use crate::Block;

pub fn render_blocks(mut query: Query<(&Block, &mut Visibility, &mut Sprite, &mut Transform), Changed<Block>>) {
    for (block, mut visibility, mut sprite, mut transform) in query.iter_mut() {
        sprite.color = block.color;
        *visibility = Visibility::Visible;
        transform.translation.x = block.x as f32 * 10.0;
        transform.translation.y = block.y as f32 * 10.0;
    }
}
