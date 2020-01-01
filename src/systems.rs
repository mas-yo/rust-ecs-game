use crate::components::*;
use quicksilver::prelude::*;
use std::marker::PhantomData;
use std::ops::Deref;

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

impl<RC, MC> System<CContainer<RC>, CContainer<MC>> {
    fn for_each_component<F>(_ref: &CContainer<RC>, _refmut: &mut CContainer<MC>, pred: F)
    where
        F: Fn(&RC, &mut MC) -> (),
    {
        _refmut.iter_mut().for_each(|rm| {
            if let Some(r) = CContainer::<RC>::get(_ref, rm.entity_id()) {
                pred(r, rm);
            }
        })
    }
}

impl<RC1, RC2, MC> System<(&CContainer<RC1>, &CContainer<RC2>), CContainer<MC>> {
    fn for_each_component<F>(
        _ref: (&CContainer<RC1>, &CContainer<RC2>),
        _refmut: &mut CContainer<MC>,
        pred: F,
    ) where
        F: Fn(&RC1, &RC2, &mut MC) -> (),
    {
        _refmut.iter_mut().for_each(|rm| {
            if let Some(r1) = CContainer::<RC1>::get(_ref.0, rm.entity_id()) {
                if let Some(r2) = CContainer::<RC2>::get(_ref.1, rm.entity_id()) {
                    pred(r1, r2, rm);
                }
            }
        })
    }
}

impl SystemProcess for System<CContainer<Input>, CContainer<Velocity>> {
    fn process(inputs: &Self::Ref, velocities: &mut Self::RefMut) {
        Self::for_each_component(inputs, velocities, |input, velocity|
        {
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

impl SystemProcess
    for System<(&CContainer<Team>, &CContainer<Position>), CContainer<MoveTarget>>
{
    fn process(team_pos: &Self::Ref, move_targets: &mut Self::RefMut) {
        let (teams, positions) = team_pos;
        move_targets.iter_mut().for_each(|target| {

            if let Some((self_team, self_pos)) =
                get_component2(teams, positions, target.entity_id())
            {
                teams
                    .iter()
                    .filter(|t| t.team_id() != self_team.team_id())
                    .for_each(|t| {
                        if let Some(pos) = CContainer::<Position>::get(positions, t.entity_id()) {
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
            }
        });
    }
}

impl SystemProcess
    for System<(&CContainer<Position>, &CContainer<MoveTarget>), CContainer<Velocity>>
{
    fn process(pos_tgt: &Self::Ref, velocities: &mut Self::RefMut) {
        Self::for_each_component(*pos_tgt, velocities, |pos, target, vel| {
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
        Self::for_each_component(velocities, positions, |vel, pos| {
            pos.x += vel.x;
            pos.y += vel.y;
        });
    }
}

impl SystemProcess for System<CContainer<Position>, CContainer<CharacterView>> {
    fn process(positions: &Self::Ref, views: &mut Self::RefMut) {
        Self::for_each_component(positions, views, |pos, view| {
            view.position.x = pos.x;
            view.position.y = pos.y;
            view.direction = 0f32;
        });
    }
}

impl SystemProcess for System<CContainer<CharacterView>, Window> {
    fn process(views: &Self::Ref, window: &mut Self::RefMut) {
        views.iter().for_each(|view| {
            window.draw(
                &Circle::new((view.position.x, view.position.y), view.radius),
                Col(view.color),
            );
        })
    }
}

