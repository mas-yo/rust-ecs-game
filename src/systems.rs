use crate::components::*;
use quicksilver::prelude::*;
use std::marker::PhantomData;

pub(crate) trait SystemInterface {
    type Ref;
    type RefMut;
}
pub(crate) trait SystemProcess: SystemInterface {
    fn process(_ref: &Self::Ref, _refmut: &mut Self::RefMut);
}

#[derive(Default)]
pub(crate) struct System<R, M> {
    phantom: PhantomData<(R, M)>,
}

impl<R, M> SystemInterface for System<R, M> {
    type Ref = R;
    type RefMut = M;
}

impl SystemProcess for System<CContainer<Input>, CContainer<Velocity>> {
    fn process(inputs: &Self::Ref, velocities: &mut Self::RefMut) {
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

impl SystemProcess for System<(&CContainer<Team>, &CContainer<Position>), CContainer<MoveTarget>> {
    fn process(team_pos: &Self::Ref, move_targets: &mut Self::RefMut) {
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
    for System<(&CContainer<Position>, &CContainer<MoveTarget>), CContainer<Velocity>>
{
    fn process(pos_tgt: &Self::Ref, velocities: &mut Self::RefMut) {
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

impl SystemProcess for System<CContainer<Velocity>, CContainer<Position>> {
    fn process(velocities: &Self::Ref, positions: &mut Self::RefMut) {
        positions
            .iter_mut()
            .zip_entity(velocities)
            .for_each(|(pos, vel)| {
                pos.x += vel.x;
                pos.y += vel.y;
            });
    }
}

impl SystemProcess for System<CContainer<Position>, CContainer<CharacterView>> {
    fn process(positions: &Self::Ref, views: &mut Self::RefMut) {
        views
            .iter_mut()
            .zip_entity(positions)
            .for_each(|(view, pos)| {
                view.position.x = pos.x;
                view.position.y = pos.y;
                view.direction = 0f32;
            });
    }
}

impl SystemProcess for System<CContainer<CharacterView>, Window> {
    fn process(views: &Self::Ref, window: &mut Self::RefMut) {
        views.iter().for_each(|(_, view)| {
            window.draw(
                &Circle::new((view.position.x, view.position.y), view.radius),
                Col(view.color),
            );
        })
    }
}
