use bevy::prelude::*;

use crate::Volocity;

pub fn keyboard_input(input: Res<Input<KeyCode>>, mut query: Query<&mut Volocity>) {
    for mut volocity in query.iter_mut() {
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

        if volocity.last_x + volocity.x == 0 && volocity.last_y + volocity.y == 0 || (volocity.last_x == 0 && volocity.last_y == 0) {
            *volocity = origin_volocity;
        }
    }
}
