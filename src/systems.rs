use crate::components::*;
use crate::*;
use quicksilver::prelude::*;
use std::f32::consts::*;
use std::hash::Hash;
use std::marker::PhantomData;

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

pub(crate) struct ForObserverSet();
pub(crate) struct ForObserverCheck();

impl<V, C> SystemProcess
    for System<CContainer<ValueObserver<V, C>>, (&CContainer<C>, ForObserverSet)>
where
    V: PartialEq + Copy,
{
    fn process(observers: &mut Self::Update, (components, _): &Self::Refer) {
        observers
            .iter_mut()
            .zip_entity(components)
            .for_each(|(observer, component)| {
                observer.set(component);
            });
    }
}

impl<V, C> SystemProcess
    for System<CContainer<ValueObserver<V, C>>, (&CContainer<C>, ForObserverCheck)>
where
    V: PartialEq + Copy,
{
    fn process(observers: &mut Self::Update, components: &Self::Refer) {
        observers
            .iter_mut()
            .zip_entity(components.0)
            .for_each(|(observer, component)| {
                observer.check(component);
            });
    }
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
    fn process(move_targets: &mut Self::Update, (teams, positions): &Self::Refer) {
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

impl SystemProcess
    for System<CContainer<Direction>, (&CContainer<Position>, &CContainer<MoveTarget>)>
{
    fn process(directions: &mut Self::Update, (positions, targets): &Self::Refer) {
        directions
            .iter_mut()
            .zip_entity2(positions, targets)
            .for_each(|(dir, position, target)| {
                if position != target {
                    *dir = (target.y - position.y).atan2(target.x - position.x);
                }
            });
    }
}

impl SystemProcess
    for System<CContainer<Velocity>, (&CContainer<CharacterView>, &CContainer<CharacterAnimator>)>
{
    fn process(velocities: &mut Self::Update, (views, animators): &Self::Refer) {
        velocities
            .iter_mut()
            .zip_entity2(views, animators)
            .for_each(|(velocity, view, animator)| {
                if let Some(val) = animator.value() {
                    if val.move_forward != 0f32 {
                        velocity.x = view.direction.cos() * val.move_forward;
                        velocity.y = view.direction.sin() * val.move_forward;
                    }
                }
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

impl SystemProcess for System<CContainer<Direction>, CContainer<Input>> {
    fn process(directions: &mut Self::Update, inputs: &Self::Refer) {
        directions
            .iter_mut()
            .zip_entity(inputs)
            .for_each(|(direction, input)| {
                if input.left {
                    *direction = PI;
                    if input.up {
                        *direction = FRAC_PI_4 * 5f32;
                    }
                    if input.down {
                        *direction = FRAC_PI_4 * 3f32;
                    }
                } else if input.right {
                    *direction = 0f32;
                    if input.up {
                        *direction = FRAC_PI_4 * 7f32;
                    }
                    if input.down {
                        *direction = FRAC_PI_4;
                    }
                } else {
                    if input.up {
                        *direction = FRAC_PI_2 * 3f32;
                    }
                    if input.down {
                        *direction = FRAC_PI_2;
                    }
                }
            });
    }
}

// pub(crate) struct ForBodyCollider();

// impl<I> SystemProcess
//     for System<
//         CContainer<Collider<I, quicksilver::geom::Circle>>,
//         (&CContainer<Position>, ForBodyCollider),
//     >
// {
//     fn process(colliders: &mut Self::Update, (positions, _): &Self::Refer) {
//         colliders
//             .iter_mut()
//             .zip_entity(positions)
//             .for_each(|(collider, position)| {
//                 collider.shape.pos = *position;
//             });
//     }
// }

// pub(crate) trait IsCollided {
//     fn is_collided(&self, other: &Self) -> bool;
// }

// impl<S> SystemProcess for System<CContainer<Collision<I>>, CContainer<Collider<S>>>
// where
//     S: IsCollided,
// {
//     fn process(collisions: &mut Self::Update, colliders: &Self::Refer) {
//         collisions.iter_mut().for_each(|(entity_id, collision)| {
//             if let Some(collider) = colliders.get(entity_id) {
//                 collision.collided_ids.clear();
//                 colliders
//                     .iter()
//                     .for_each(|(other_entity_id, other_collider)| {
//                         if entity_id == other_entity_id {
//                             return;
//                         }
//                         if collider.shape.is_collided(&other_collider.shape) {
//                             collision.collided_ids.push(other_collider.collider_id);
//                         }
//                     });
//             }
//         });
//     }
// }

impl SystemProcess
    for System<
        CContainer<WeaponHit>,
        (
            &CContainer<SwordCollider>,
            &CContainer<BodyCollider>,
            &CContainer<Team>,
        ),
    >
{
    fn process(
        weapon_hits: &mut Self::Update,
        (sword_colliders, body_colliders, teams): &Self::Refer,
    ) {
        weapon_hits
            .iter_mut()
            .zip_entity2(body_colliders, teams)
            .for_each(|(hit, self_body, self_team)| {
                sword_colliders
                    .iter()
                    .zip_entity(teams)
                    .for_each(|(other_sword, other_team)| {
                        if self_team.team_id() != other_team.team_id() {
                            if other_sword.is_collided(self_body) {
                                hit.hit = true;
                            }
                        }
                    });
            });
    }
}

impl SystemProcess
    for System<
        CContainer<WeaponHit>,
        (
            &CContainer<BodyWeaponCollider>,
            &CContainer<BodyCollider>,
            &CContainer<Team>,
        ),
    >
{
    fn process(
        weapon_hits: &mut Self::Update,
        (body_weapons, body_colliders, teams): &Self::Refer,
    ) {
        weapon_hits
            .iter_mut()
            .zip_entity2(body_colliders, teams)
            .for_each(|(hit, self_body, self_team)| {
                body_weapons
                    .iter()
                    .zip_entity(teams)
                    .for_each(|(other_weapon, other_team)| {
                        if self_team.team_id() != other_team.team_id() {
                            if other_weapon.is_collided(&self_body) {
                                hit.hit = true;
                            }
                        }
                    });
            });
    }
}

impl SystemProcess for System<CContainer<CharacterAnimator>, CContainer<Input>> {
    fn process(animators: &mut Self::Update, inputs: &Self::Refer) {
        animators.iter_mut().zip_entity(inputs).for_each(|(a, i)| {
            if let Some(id) = a.playing_id() {
                if id == CharacterAnimID::Attack && a.is_end() {
                    a.play(CharacterAnimID::Wait);
                }
                if id == CharacterAnimID::Damaged && a.is_end() {
                    a.play(CharacterAnimID::Wait);
                }
                if i.attack && id != CharacterAnimID::Damaged {
                    a.play(CharacterAnimID::Damaged);
                }
            }
            a.update()
        });
    }
}

impl SystemProcess for System<CContainer<CharacterView>, CContainer<CharacterAnimator>> {
    fn process(views: &mut Self::Update, animators: &Self::Refer) {
        views
            .iter_mut()
            .zip_entity(animators)
            .for_each(|(view, animator)| {
                if let Some(val) = animator.value() {
                    view.radius_scale = val.radius_scale;
                    view.weapon_direction = val.weapon_direction;
                }
            });
    }
}

impl SystemProcess
    for System<CContainer<CharacterView>, (&CContainer<Position>, &CContainer<Direction>)>
{
    fn process(views: &mut Self::Update, (positions, directions): &Self::Refer) {
        views
            .iter_mut()
            .zip_entity2(positions, directions)
            .for_each(|(view, pos, dir)| {
                view.position.x = pos.x;
                view.position.y = pos.y;
                view.direction = *dir;
            });
    }
}

impl SystemProcess for System<Window, CContainer<CharacterView>> {
    fn process(window: &mut Self::Update, views: &Self::Refer) {
        views.iter().for_each(|(_, view)| {
            // log::info!("r {}", view.radius_scale);

            window.draw(
                &Circle::new(
                    (view.position.x, view.position.y),
                    view.radius * view.radius_scale,
                ),
                Col(view.color),
            );
            let dir = view.direction + view.weapon_direction;
            let line_end = (
                view.position.x + dir.cos() * view.radius * 1.8f32,
                view.position.y + dir.sin() * view.radius * 1.8f32,
            );
            window.draw(
                &Line::new((view.position.x, view.position.y), line_end),
                Col(view.color),
            );
        });
    }
}
