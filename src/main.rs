use quicksilver::prelude::*;
use std::f32::consts::*;
mod components;
mod systems;

use components::*;
use systems::*;

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

#[derive(Default)]
struct Game {
    next_entity_id: EntityID,
    inputs: CContainer<Input>,
    teams: CContainer<Team>,
    sword_colliders: CContainer<SwordCollider>,
    body_weapon_colliders: CContainer<BodyWeaponCollider>,
    body_colliders: CContainer<BodyCollider>,
    weapon_hits: CContainer<WeaponHit>,
    move_targets: CContainer<MoveTarget>,
    positions: CContainer<Position>,
    directions: CContainer<Direction>,
    velocities: CContainer<Velocity>,
    character_animators: CContainer<CharacterAnimator>,
    character_views: CContainer<CharacterView>,
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

    fn create_hero(&mut self) {
        let entity_id = self.next_entity_id;

        self.inputs.push(entity_id, Input::default());
        self.teams.push(entity_id, Team::new(0));
        self.positions.push(
            entity_id,
            Position {
                x: 150f32,
                y: 150f32,
            },
        );
        self.weapon_hits.push(entity_id, WeaponHit::default());
        self.body_colliders.push(entity_id, BodyCollider::default());
        self.sword_colliders
            .push(entity_id, SwordCollider::default());

        self.directions.push(entity_id, Direction::default());
        self.velocities.push(entity_id, Velocity::default());

        let mut animator = CharacterAnimator::default();
        animator.register(CharacterAnimID::Wait, Self::wait_animation());
        animator.register(CharacterAnimID::Attack, Self::attack_animation());
        animator.register(CharacterAnimID::Damaged, Self::damaged_animation());
        animator.play(CharacterAnimID::Wait);
        self.character_animators.push(entity_id, animator);

        self.character_views.push(
            entity_id,
            CharacterView {
                color: Color::GREEN,
                radius: 10f32,
                radius_scale: 1f32,
                ..Default::default()
            },
        );

        self.next_entity_id = self.next_entity_id + 1;
    }

    fn create_enemy(&mut self) {
        let entity_id = self.next_entity_id;

        self.move_targets.push(entity_id, MoveTarget::default());
        self.teams.push(entity_id, Team::new(1));

        self.weapon_hits.push(entity_id, WeaponHit::default());
        self.body_colliders.push(entity_id, BodyCollider::default());
        self.body_weapon_colliders
            .push(entity_id, BodyWeaponCollider::default());

        self.positions
            .push(entity_id, Position { x: 10f32, y: 10f32 });
        self.directions.push(entity_id, Direction::default());
        self.velocities.push(entity_id, Velocity::default());

        let mut animator = CharacterAnimator::default();
        animator.register(CharacterAnimID::Wait, Self::wait_animation());
        animator.register(CharacterAnimID::Attack, Self::attack_animation());
        animator.register(CharacterAnimID::Damaged, Self::damaged_animation());
        animator.play(CharacterAnimID::Wait);
        self.character_animators.push(entity_id, animator);

        self.character_views.push(
            entity_id,
            CharacterView {
                color: Color::RED,
                radius: 15f32,
                radius_scale: 1f32,
                ..Default::default()
            },
        );

        self.next_entity_id = self.next_entity_id + 1;
    }
}

impl State for Game {
    fn new() -> Result<Game> {
        let mut game = Self::default();
        game.create_hero();
        game.create_enemy();
        Ok(game)
    }

    /// Will happen at a fixed rate of 60 ticks per second under ideal conditions. Under non-ideal conditions,
    /// the game loop will do its best to still call the update at about 60 TPS.
    ///
    /// By default it does nothing
    fn update(&mut self, _window: &mut Window) -> Result<()> {
        System::process(&mut self.body_colliders, &self.character_views);
        System::process(&mut self.sword_colliders, &(&self.character_views, &self.character_animators));
        System::process(&mut self.body_weapon_colliders, &self.character_views);

        System::process(
            &mut self.weapon_hits,&()
        );
        System::process(
            &mut self.weapon_hits,
            &(&self.sword_colliders, &self.body_colliders, &self.teams),
        );
        System::process(
            &mut self.weapon_hits,
            &(
                &self.body_weapon_colliders,
                &self.body_colliders,
                &self.teams,
            ),
        );

        System::process(&mut self.move_targets, &(&self.teams, &self.positions));
        System::process(&mut self.velocities, &self.inputs);
        System::process(&mut self.velocities, &(&self.positions, &self.move_targets));
        System::process(
            &mut self.velocities,
            &(&self.character_views, &self.character_animators),
        );

        System::process(&mut self.positions, &self.velocities);
        System::process(&mut self.directions, &self.inputs);
        System::process(&mut self.directions, &(&self.positions, &self.move_targets));
        System::process(&mut self.character_animators, &self.inputs);
        System::process(&mut self.character_animators, &self.weapon_hits);
        System::process(&mut self.character_animators, &());
        System::process(&mut self.character_views, &self.character_animators);
        System::process(
            &mut self.character_views,
            &(&self.positions, &self.directions),
        );

        Ok(())
    }
    /// Process an incoming event
    ///
    /// By default it does nothing
    fn event(&mut self, event: &Event, _: &mut Window) -> Result<()> {
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
                        self.inputs.iter_mut().for_each(|(_, i)| {
                            i.left = pressed;
                        });
                    }
                    Key::D => {
                        self.inputs.iter_mut().for_each(|(_, i)| {
                            i.right = pressed;
                        });
                    }
                    Key::W => {
                        self.inputs.iter_mut().for_each(|(_, i)| {
                            i.up = pressed;
                        });
                    }
                    Key::S => {
                        self.inputs.iter_mut().for_each(|(_, i)| {
                            i.down = pressed;
                        });
                    }
                    Key::Space => {
                        // log::info!("space");
                        self.inputs.iter_mut().for_each(|(_, i)| {
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
        System::process(window, &self.character_views);
        Ok(())
    }
}

fn main() {
    web_logger::init();
    run::<Game>("Game", Vector::new(800, 600), Settings::default());
}
