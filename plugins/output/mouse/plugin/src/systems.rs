use bevy::prelude::EventReader;
use enigo::{Enigo, MouseControllable};
use rdev::EventType;

use mouse_output_api::{Direction, DragAndDrop, DragAndDropAction, MouseClick, MoveMouseRelatively, MoveMouseToPosition, Scroll};
use mouse_output_api::mouse::Button;

pub fn on_move_mouse_relatively_event_system(mut events: EventReader<MoveMouseRelatively>) {
    events.read()
        .for_each(move_mouse_relatively);
}

pub fn on_move_mouse_to_position_event_system(mut events: EventReader<MoveMouseToPosition>) {
    events.read()
        .for_each(move_mouse_to_position);
}

pub fn on_scroll_event_system(mut events: EventReader<Scroll>) {
    events.read()
        .for_each(scroll);
}

pub fn on_click_event_system(mut events: EventReader<MouseClick>) {
    events.read()
        .for_each(|event| click(&event.button))
}

pub fn on_drag_and_drop_event_system(mut events: EventReader<DragAndDrop>) {
    events.read()
        .for_each(drag_and_drop);
}

fn move_mouse_to_position(move_mouse: &MoveMouseToPosition) {
    println!("mouse move absolutely");
    let (x, y) = (move_mouse.x, move_mouse.y);
    rdev::simulate(&EventType::MouseMove { x, y }).unwrap();
}

// todo use one crate for global and relative mouse movements
fn move_mouse_relatively(move_mouse: &MoveMouseRelatively) {
    let distance = move_mouse.distance;
    match move_mouse.direction {
        Direction::Up => Enigo.mouse_move_relative(0, -distance),
        Direction::Down => Enigo.mouse_move_relative(0, distance),
        Direction::Left => Enigo.mouse_move_relative(-distance, 0),
        Direction::Right => Enigo.mouse_move_relative(distance, 0),
    }
}

fn scroll(scroll: &Scroll) {
    let distance = scroll.distance;
    match scroll.direction {
        Direction::Up => rdev::simulate(&EventType::Wheel { delta_x: 0, delta_y: distance }).unwrap(),
        Direction::Down => rdev::simulate(&EventType::Wheel { delta_x: 0, delta_y: -distance }).unwrap(),
        Direction::Left => rdev::simulate(&EventType::Wheel { delta_x: -distance, delta_y: 0 }).unwrap(),
        Direction::Right => rdev::simulate(&EventType::Wheel { delta_x: distance, delta_y: 0 }).unwrap()
    }
}

fn drag_and_drop(drag_and_drop: &DragAndDrop) {
    let button = &drag_and_drop.button;
    match drag_and_drop.action {
        DragAndDropAction::Start => press_button(button),
        DragAndDropAction::End => release_button(button)
    }
}

fn click(button: &Button) {
    press_button(button);
    release_button(button);
}

fn press_button(button: &Button) {
    rdev::simulate(&EventType::ButtonPress(button.into())).unwrap();
}

fn release_button(button: &Button) {
    rdev::simulate(&EventType::ButtonRelease(button.into())).unwrap();
}