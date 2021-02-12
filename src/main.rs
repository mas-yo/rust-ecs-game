#[macro_use]
extern crate log;

#[macro_use]
extern crate static_ecs;
use static_ecs::*;

use quicksilver::prelude::*;
use static_ecs::component::*;
use std::f32::consts::*;

mod components;
mod systems;

use components::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum CharacterAnimID {
    Wait,
    Attack,
    Damaged,
}

impl Default for CharacterAnimID {
    fn default() -> Self {
        CharacterAnimID::Wait
    }
}

type CharacterAnimator = Animator<CharacterAnimID, CharacterAnimFrame>;

world! {
    World {
        Input,
        Team,
        Health,
        SwordCollider,
        BodyWeaponCollider,
        BodyDefenseCollider,
        MoveTarget,
        Position,
        Direction,
        Velocity,
        CharacterAnimator,
        StatusBarView<ForHealth>,
        CharacterView,
    }
}

// #[derive(Default)]
struct Game {
    world: World,
}

impl Game {
    fn wait_animation() -> Animation<CharacterAnimFrame> {
        let mut frames = Vec::new();

        for d in 0..20 {
            let s = ((d as f32 / 20f32 * PI).sin() * 0.2f32 - 0.1f32) + 1.0f32;
            frames.push(CharacterAnimFrame {
                radius_scale: s,
                weapon_direction: 0f32,
                ..Default::default()
            });
        }

        Animation::new(true, frames)
    }

    fn attack_animation() -> Animation<CharacterAnimFrame> {
        let mut frames = Vec::new();

        for f in 0..12 {
            let dir = -FRAC_PI_4 - FRAC_PI_8 + f as f32 * FRAC_PI_8 / 2f32;
            frames.push(CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: dir,
                ..Default::default()
            });
        }
        Animation::new(false, frames)
    }

    fn damaged_animation() -> Animation<CharacterAnimFrame> {
        let mut frames = Vec::new();

        for _ in 0..12 {
            frames.push(CharacterAnimFrame {
                radius_scale: 1f32,
                move_forward: -8f32,
                ..Default::default()
            });
        }

        Animation::new(false, frames)
    }

    fn create_hero(world: &mut World) {
        // let entity_id = self.next_entity_id;

        let mut animator = CharacterAnimator::default();
        animator.register(CharacterAnimID::Wait, Self::wait_animation());
        animator.register(CharacterAnimID::Attack, Self::attack_animation());
        animator.register(CharacterAnimID::Damaged, Self::damaged_animation());
        animator.play(CharacterAnimID::Wait);

        add_entity!(
            world;
            Input::default(),
            Team::new(0),
            Health::new(100),
            Position(Vector{x:150f32, y:150f32}),
            BodyDefenseCollider::default(),
            SwordCollider::default(),
            Direction::default(),
            Velocity::default(),
            animator,
            StatusBarView::<ForHealth>::new(24, Color::GREEN),
            CharacterView {
                color: Color::GREEN,
                radius: 10f32,
                radius_scale: 1f32,
                ..Default::default()
            },
        );
    }

    fn create_enemy(world: &mut World, x: f32, y: f32) {
        let mut animator = CharacterAnimator::default();
        animator.register(CharacterAnimID::Wait, Self::wait_animation());
        animator.register(CharacterAnimID::Attack, Self::attack_animation());
        animator.register(CharacterAnimID::Damaged, Self::damaged_animation());
        animator.play(CharacterAnimID::Wait);

        add_entity!(
            world;
            MoveTarget::default(),
            Team::new(1),
            Health::new(100),
            Position(Vector{x:x, y:y}),
            BodyDefenseCollider::default(),
            BodyWeaponCollider::default(),
            Direction::default(),
            Velocity::default(),
            animator,
            StatusBarView::<ForHealth>::new(24, Color::GREEN),
            CharacterView {
                color: Color::RED,
                radius: 15f32,
                radius_scale: 1f32,
                ..Default::default()
            },
        );
    }
}

impl State for Game {
    fn new() -> Result<Game> {
        info!("----- starte game -----");
        let mut world = World::default();
        Game::create_hero(&mut world);
        Game::create_enemy(&mut world, 20f32, 20f32);
        Game::create_enemy(&mut world, 100f32, 20f32);
        Ok(Game { world })
    }

    /// Will happen at a fixed rate of 60 ticks per second under ideal conditions. Under non-ideal conditions,
    /// the game loop will do its best to still call the update at about 60 TPS.
    ///
    /// By default it does nothing
    fn update(&mut self, _window: &mut Window) -> Result<()> {
        system!(
            self.world,
            |_entity_id,
             collider: &SwordCollider,
             view: &CharacterView,
             animator: &CharacterAnimator| {
                let mut col = collider.clone();
                let dir = view.direction + view.weapon_direction;
                col.line.a = view.position;
                col.line.b.x = view.position.x + dir.cos() * view.radius * 1.8f32;
                col.line.b.y = view.position.y + dir.sin() * view.radius * 1.8f32;

                col.active = false;
                if let Some(id) = animator.playing_id() {
                    if id == CharacterAnimID::Attack {
                        col.active = true;
                    }
                }
                col
            }
        );

        system!(
            self.world,
            |_entity_id, collider: &BodyWeaponCollider, view: &CharacterView| {
                let mut col = collider.clone();
                col.circle.pos = view.position;
                col.circle.radius = view.radius;
                col
            }
        );

        {
            let sword_colliders = component!(self.world, SwordCollider);
            let body_weapon_colliders = component!(self.world, BodyWeaponCollider);
            let teams = component!(self.world, Team);

            system!(
                self.world,
                |defense_entity_id,
                 body_defense: &BodyDefenseCollider,
                 view: &CharacterView,
                 defense_team: &Team| {
                    let mut new_body_defense = body_defense.clone();

                    new_body_defense.hit = false;
                    new_body_defense.circle.pos = view.position;
                    new_body_defense.circle.radius = view.radius;

                    sword_colliders.iter().zip_entity(teams).for_each(
                        |(sword_entity_id, sword_collider, sword_team)| {
                            if defense_entity_id == sword_entity_id {
                                return;
                            }
                            if defense_team.team_id() == sword_team.team_id() {
                                return;
                            }
                            if sword_collider.is_collided(body_defense) {
                                new_body_defense.hit = true;
                            }
                        },
                    );

                    body_weapon_colliders.iter().zip_entity(teams).for_each(
                        |(weapon_entity_id, weapon_collider, weapon_team)| {
                            if defense_entity_id == weapon_entity_id {
                                return;
                            }
                            if defense_team.team_id() == weapon_team.team_id() {
                                return;
                            }
                            if weapon_collider.is_collided(body_defense) {
                                new_body_defense.hit = true;
                            }
                        },
                    );
                    new_body_defense
                }
            );
        }

        system!(
            self.world,
            |_entity_id, health: &Health, collider: &BodyDefenseCollider| {
                let mut new_health = health.clone();
                if collider.hit {
                    new_health.current_health -= 10;
                }
                new_health
            }
        );

        {
            let teams = component!(self.world, Team);
            let positions = component!(self.world, Position);
            system!(
                self.world,
                |_entity_id, move_target: &MoveTarget, self_team: &Team, self_pos: &Position| {
                    let mut new_target = move_target.clone();
                    teams
                        .iter()
                        .filter(|(_, team)| team.team_id() != self_team.team_id())
                        .for_each(|(entity_id, _)| {
                            if let Some(pos) = positions.get(entity_id) {
                                let distance = pos.0.distance((self_pos.0.x, self_pos.0.y));
                                if distance < 100f32 {
                                    new_target.0.x = pos.0.x;
                                    new_target.0.y = pos.0.y;
                                } else {
                                    new_target.0.x = self_pos.0.x;
                                    new_target.0.y = self_pos.0.y;
                                }
                            }
                        });

                    new_target
                }
            );
        }

        system!(self.world, |_entity_id,
                             velocity: &Velocity,
                             input: &Input| {
            let mut new_velocity = velocity.clone();
            new_velocity.0.x = 0f32;
            new_velocity.0.y = 0f32;
            if input.left {
                new_velocity.0.x = -2f32;
            }
            if input.right {
                new_velocity.0.x = 2f32;
            }
            if input.up {
                new_velocity.0.y = -2f32;
            }
            if input.down {
                new_velocity.0.y = 2f32;
            }
            new_velocity
        });

        system!(
            self.world,
            |_entity_id, velocity: &Velocity, pos: &Position, target: &MoveTarget| {
                let mut new_velocity = velocity.clone();
                let mut tmp = Vector::default();
                tmp.x = target.0.x - pos.0.x;
                tmp.y = target.0.y - pos.0.y;
                new_velocity.0.x = tmp.x / 50f32;
                new_velocity.0.y = tmp.y / 50f32;
                new_velocity
            }
        );

        system!(
            self.world,
            |_entity_id, vel: &Velocity, view: &CharacterView, animator: &CharacterAnimator| {
                let mut velocity = vel.clone();
                if let Some(val) = animator.value() {
                    if val.move_forward != 0f32 {
                        velocity.0.x = view.direction.cos() * val.move_forward;
                        velocity.0.y = view.direction.sin() * val.move_forward;
                    }
                }
                velocity
            }
        );

        system!(self.world, |_entity_id, pos: &Position, vel: &Velocity| {
            let mut new_pos = pos.clone();
            new_pos.0.x += vel.0.x;
            new_pos.0.y += vel.0.y;
            new_pos
        });

        system!(self.world, |_entity_id, dir: &Direction, input: &Input| {
            let mut new_dir = dir.clone();
            if input.left {
                new_dir = PI;
                if input.up {
                    new_dir = FRAC_PI_4 * 5f32;
                }
                if input.down {
                    new_dir = FRAC_PI_4 * 3f32;
                }
            } else if input.right {
                new_dir = 0f32;
                if input.up {
                    new_dir = FRAC_PI_4 * 7f32;
                }
                if input.down {
                    new_dir = FRAC_PI_4;
                }
            } else {
                if input.up {
                    new_dir = FRAC_PI_2 * 3f32;
                }
                if input.down {
                    new_dir = FRAC_PI_2;
                }
            }
            new_dir
        });

        system!(
            self.world,
            |_entity_id, dir: &Direction, pos: &Position, target: &MoveTarget| {
                let mut new_dir = dir.clone();
                if pos.0 != target.0 {
                    new_dir = (target.0.y - pos.0.y).atan2(target.0.x - pos.0.x);
                }
                new_dir
            }
        );

        system!(self.world, |_entity_id,
                             animator: &CharacterAnimator,
                             input: &Input| {
            let mut new_animator = animator.clone();
            if let Some(id) = new_animator.playing_id() {
                if id == CharacterAnimID::Attack && new_animator.is_end() {
                    new_animator.play(CharacterAnimID::Wait);
                }
                if input.attack && id != CharacterAnimID::Attack {
                    new_animator.play(CharacterAnimID::Attack);
                }
            }
            new_animator
        });

        system!(
            self.world,
            |_entity_id, animator: &CharacterAnimator, collider: &BodyDefenseCollider| {
                let mut new_animator = animator.clone();
                if let Some(id) = new_animator.playing_id() {
                    if id == CharacterAnimID::Damaged && new_animator.is_end() {
                        new_animator.play(CharacterAnimID::Wait);
                    }
                    if collider.hit && id != CharacterAnimID::Damaged {
                        new_animator.play(CharacterAnimID::Damaged);
                    }
                }
                new_animator
            }
        );

        system!(self.world, |_entity_id, animator: &CharacterAnimator| {
            let mut new_animator = animator.clone();
            new_animator.update();
            new_animator
        });

        system!(
            self.world,
            |_entity_id, view: &CharacterView, animator: &CharacterAnimator| {
                let mut new_view = view.clone();
                if let Some(val) = animator.value() {
                    new_view.radius_scale = val.radius_scale;
                    new_view.weapon_direction = val.weapon_direction;
                }
                new_view
            }
        );

        system!(self.world, |_entity_id,
                             view: &CharacterView,
                             pos: &Position,
                             dir: &Direction| {
            let mut new_view = view.clone();
            new_view.position.x = pos.0.x;
            new_view.position.y = pos.0.y;
            new_view.direction = *dir;
            new_view
        });

        system!(self.world, |_entity_id,
                             bar: &StatusBarView<ForHealth>,
                             health: &Health| {
            let mut new_bar = bar.clone();
            new_bar.current_length = (new_bar.frame_length as f32 * health.ratio()) as i32;
            new_bar
        });

        system!(
            self.world,
            |_entity_id, bar: &StatusBarView<ForHealth>, view: &CharacterView| {
                let mut new_bar = bar.clone();
                new_bar.position = view.position + Vector::new(10f32, -10f32);
                if new_bar.animated_length != new_bar.current_length {
                    let diff = new_bar.current_length - new_bar.animated_length;
                    let mov = diff / diff.abs();
                    new_bar.animated_length += mov;
                }
                new_bar
            }
        );

        Ok(())
    }
    /// Process an incoming event
    ///
    /// By default it does nothing
    fn event(&mut self, event: &Event, _: &mut Window) -> Result<()> {
        let inputs = component_mut!(self.world, Input);
        match event {
            Event::Key(key, state) => {
                let mut pressed = false;
                if *state == ButtonState::Pressed {
                    pressed = true;
                } else if *state == ButtonState::Released {
                    pressed = false;
                }
                match key {
                    Key::A => {
                        inputs.iter_mut().for_each(|(_, i)| {
                            i.left = pressed;
                        });
                    }
                    Key::D => {
                        inputs.iter_mut().for_each(|(_, i)| {
                            i.right = pressed;
                        });
                    }
                    Key::W => {
                        inputs.iter_mut().for_each(|(_, i)| {
                            i.up = pressed;
                        });
                    }
                    Key::S => {
                        inputs.iter_mut().for_each(|(_, i)| {
                            i.down = pressed;
                        });
                    }
                    Key::Space => {
                        // log::info!("space");
                        inputs.iter_mut().for_each(|(_, i)| {
                            i.attack = pressed;
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        component!(self.world, CharacterView)
            .iter()
            .for_each(|(_, view)| {
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
        component!(self.world, StatusBarView<ForHealth>)
            .iter()
            .for_each(|(_, view)| {
                window.draw(
                    &Rectangle::new(
                        (view.position.x - 1f32, view.position.y - 1f32),
                        (view.frame_length + 1i32, 7i32),
                    ),
                    Col(Color::BLACK),
                );
                window.draw(
                    &Rectangle::new(
                        (view.position.x, view.position.y),
                        (view.animated_length, 6f32),
                    ),
                    Col(Color::RED),
                );
                window.draw(
                    &Rectangle::new(
                        (view.position.x, view.position.y),
                        (view.current_length, 6f32),
                    ),
                    Col(view.color),
                );
            });
        Ok(())
    }
}

fn main() {
    web_logger::init();
    run::<Game>("Game", Vector::new(800, 600), Settings::default());
}
