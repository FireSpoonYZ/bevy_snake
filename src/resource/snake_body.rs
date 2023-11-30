use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Resource, Default)]
pub struct SnakeBodys(pub VecDeque<Entity>, pub Option<Entity>);

#[derive(Debug, Component, Default)]
pub struct SnakeBody;
