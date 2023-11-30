use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Resource, Default)]
pub struct SnakeBody(pub VecDeque<Entity>, pub Option<Entity>);
