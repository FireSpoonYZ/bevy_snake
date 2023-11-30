use bevy::prelude::*;
use rand::Rng;

use crate::{Food, Block, Wall};

pub fn flush_food(
    mut query: Query<(&mut Food, &mut Block), Changed<Food>>,
    wall_query: Query<&Block, (With<Wall>, Without<Food>)>,
) {
    for (mut food, mut block) in query.iter_mut() {
        if food.need_flush {
            let mut rng = rand::thread_rng();
            'out: loop {
                block.x = rng.gen_range(-12..13);
                block.y = rng.gen_range(-12..13);

                for wall_block in wall_query.iter() {
                    if wall_block.x == block.x && wall_block.y == block.y {
                        continue 'out;
                    }
                }
                break;
            }
            food.need_flush = false;
        }
    }
}