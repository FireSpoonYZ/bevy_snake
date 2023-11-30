use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Food {
    pub need_flush: bool,
}

impl Food {
    pub fn new() -> Self {
        Self { need_flush: true }
    }
}