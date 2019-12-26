use crate::components::*;
use quicksilver::prelude::*;

pub(crate) fn update_velocity(input: &Input, velocity: &mut Velocity) {
    velocity.x = 0f32;
    velocity.y = 0f32;
    if input.left {
        velocity.x = -1f32;
    }
    if input.right {
        velocity.x = 1f32;
    }
    if input.up {
        velocity.y = -1f32;
    }
    if input.down {
        velocity.y = 1f32;
    }
}

pub(crate) fn update_position(velocity: &Velocity, position: &mut Position) {
    position.x += velocity.x;
    position.y += velocity.y;
}

pub(crate) fn update_character_view(position: &Position, view: &mut CharacterView) {
    view.position = *position;
    view.direction = 0f32;
    view.radius = 10f32;
}

pub(crate) fn update_window(view: &CharacterView, window: &mut Window) {
    window.draw(
        &Circle::new((view.position.x, view.position.y), view.radius),
        Col(Color::GREEN),
    );
}
