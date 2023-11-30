use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub struct Volocity {
    pub x: i32,
    pub y: i32,
    pub last_x: i32,
    pub last_y: i32,
}