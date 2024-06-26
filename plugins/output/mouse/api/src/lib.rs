use bevy::prelude::Event;

use input_model::Button;

#[derive(Event, Debug, Clone)]
pub struct MoveMouseRelatively {
    pub direction: Direction,
    pub distance: i32,
}

impl MoveMouseRelatively {
    pub fn new(direction: Direction, distance: i32) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

#[derive(Event, Debug)]
pub struct MoveMouseToPosition {
    pub x: f64,
    pub y: f64,
}

impl MoveMouseToPosition {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Event, Debug)]
pub struct Scroll {
    pub direction: Direction,
    pub distance: i64,
}

impl Scroll {
    pub fn new(direction: Direction, distance: i64) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Event, Debug)]
pub struct MouseClick {
    pub button: Button,
}

impl MouseClick {
    pub fn new(button: Button) -> Self {
        Self {
            button
        }
    }
}

#[derive(Event, Debug)]
pub struct DragAndDrop {
    pub action: DragAndDropAction,
    pub button: Button,
}

impl DragAndDrop {
    pub fn new(action: DragAndDropAction, button: Button) -> Self {
        Self {
            action,
            button,
        }
    }
}

#[derive(Debug)]
pub enum DragAndDropAction {
    Start,
    End,
}
