use crate::components::*;
use quicksilver::prelude::*;
use std::f32::consts::*;
use std::marker::PhantomData;
use web_logger;

pub(crate) trait SystemInterface {
    type Update;
    type Refer;
}
pub(crate) trait SystemProcess: SystemInterface {
    fn process(update: &mut Self::Update, _ref: &Self::Refer);
}

pub(crate) struct System<U, R> {
    phantom: PhantomData<(U, R)>,
}

impl<U, R> SystemInterface for System<U, R> {
    type Update = U;
    type Refer = R;
}

impl SystemProcess for System<CContainer<Velocity>, CContainer<Input>> {
    fn process(velocities: &mut Self::Update, inputs: &Self::Refer) {
        velocities
            .iter_mut()
            .zip_entity(inputs)
            .for_each(|(velocity, input)| {
                velocity.x = 0f32;
                velocity.y = 0f32;
                if input.left {
                    velocity.x = -2f32;
                }
                if input.right {
                    velocity.x = 2f32;
                }
                if input.up {
                    velocity.y = -2f32;
                }
                if input.down {
                    velocity.y = 2f32;
                }
            });
    }
}

impl SystemProcess for System<CContainer<MoveTarget>, (&CContainer<Team>, &CContainer<Position>)> {
    fn process(move_targets: &mut Self::Update, team_pos: &Self::Refer) {
        let (teams, positions) = team_pos;
        move_targets
            .iter_mut()
            .zip_entity2(teams, positions)
            .for_each(|(target, self_team, self_pos)| {
                teams
                    .iter()
                    .filter(|(_, team)| team.team_id() != self_team.team_id())
                    .for_each(|(entity_id, _)| {
                        if let Some(pos) = CContainer::<Position>::get(positions, entity_id) {
                            let distance = pos.distance((self_pos.x, self_pos.y));
                            if distance < 100f32 {
                                target.x = pos.x;
                                target.y = pos.y;
                            } else {
                                target.x = self_pos.x;
                                target.y = self_pos.y;
                            }
                        }
                    });
            });
    }
}

impl SystemProcess
    for System<CContainer<Velocity>, (&CContainer<Position>, &CContainer<MoveTarget>)>
{
    fn process(velocities: &mut Self::Update, pos_tgt: &Self::Refer) {
        velocities
            .iter_mut()
            .zip_entity2(pos_tgt.0, pos_tgt.1)
            .for_each(|(vel, pos, target)| {
                let mut tmp = Vector::default();
                tmp.x = target.x - pos.x;
                tmp.y = target.y - pos.y;
                vel.x = tmp.x / 50f32;
                vel.y = tmp.y / 50f32;
            });
    }
}

impl SystemProcess for System<CContainer<Position>, CContainer<Velocity>> {
    fn process(positions: &mut Self::Update, velocities: &Self::Refer) {
        positions
            .iter_mut()
            .zip_entity(velocities)
            .for_each(|(pos, vel)| {
                pos.x += vel.x;
                pos.y += vel.y;
            });
    }
}

impl SystemProcess
    for System<CContainer<CharacterView>, (&CContainer<Position>, &CContainer<Velocity>)>
{
    fn process(views: &mut Self::Update, pos_vel: &Self::Refer) {
        views
            .iter_mut()
            .zip_entity2(pos_vel.0, pos_vel.1)
            .for_each(|(view, pos, vel)| {
                view.position.x = pos.x;
                view.position.y = pos.y;
                if vel.x != 0f32 || vel.y != 0f32 {
                    view.direction = vel.y.atan2(vel.x);
                }
            });
    }
}

impl SystemProcess for System<Window, CContainer<CharacterView>> {
    fn process(window: &mut Self::Update, views: &Self::Refer) {
        views.iter().for_each(|(_, view)| {
            window.draw(
                &Circle::new((view.position.x, view.position.y), view.radius),
                Col(view.color),
            );
            let line_end = (
                view.position.x + view.direction.cos() * view.radius * 1.8f32,
                view.position.y + view.direction.sin() * view.radius * 1.8f32,
            );
            window.draw(
                &Line::new((view.position.x, view.position.y), line_end),
                Col(view.color),
            );
        });
    }
}
