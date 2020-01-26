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
}

impl Default for CharacterAnimID {
    fn default() -> Self {
        CharacterAnimID::Wait
    }
}

type CharacterAnimator = Animator<CharacterAnimID, CharacterAnimFrame>;
type CharacterStateObserver = ValueObserver<CharacterState, CharacterState>;
type CharacterAnimEndObserver = ValueObserver<bool, CharacterAnimator>;

#[derive(Default)]
struct Game {
    next_entity_id: EntityID,
    character_state_observer: CContainer<CharacterStateObserver>,
    character_anim_end_observer: CContainer<CharacterAnimEndObserver>,
    inputs: CContainer<Input>,
    teams: CContainer<Team>,
    character_states: CContainer<CharacterState>,
    move_targets: CContainer<MoveTarget>,
    positions: CContainer<Position>,
    velocities: CContainer<Velocity>,
    character_animators: CContainer<CharacterAnimator>,
    character_views: CContainer<CharacterView>,
}

impl Game {
    fn wait_animation() -> Animation<CharacterAnimFrame> {
        let mut frames = vec![
            CharacterAnimFrame {
                radius_scale: 1.0f32,
                weapon_direction: 0f32,
            };
            58
        ];

        frames.push(CharacterAnimFrame {
            radius_scale: 1.1f32,
            ..Default::default()
        });
        frames.push(CharacterAnimFrame {
            radius_scale: 1.1f32,
            ..Default::default()
        });

        Animation::new(true, frames)
    }

    fn attack_animation() -> Animation<CharacterAnimFrame> {
        let mut frames = vec![
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: -FRAC_PI_4 - FRAC_PI_8,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: -FRAC_PI_4 - FRAC_PI_8,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: -FRAC_PI_4,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: -FRAC_PI_4,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: -FRAC_PI_8,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: -FRAC_PI_8,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: 0f32,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: 0f32,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: FRAC_PI_8,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: FRAC_PI_8,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: FRAC_PI_4,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: FRAC_PI_4,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: FRAC_PI_4 + FRAC_PI_8,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: FRAC_PI_4 + FRAC_PI_8,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: 0f32,
            },
            CharacterAnimFrame {
                radius_scale: 1f32,
                weapon_direction: 0f32,
            },
        ];
        Animation::new(false, frames)
    }

    fn create_hero(&mut self) {
        let entity_id = self.next_entity_id;

        fn exact<T>(value: &T) -> T
        where
            T: Copy,
        {
            *value
        }
        self.character_state_observer.push(
            entity_id,
            CharacterStateObserver::new(CharacterState::default(), exact::<CharacterState>),
        );

        self.character_anim_end_observer.push(
            entity_id,
            CharacterAnimEndObserver::new(false, CharacterAnimator::is_end),
        );

        self.inputs.push(entity_id, Input::default());
        self.teams.push(entity_id, Team::new(0));
        self.positions.push(
            entity_id,
            Position {
                x: 150f32,
                y: 150f32,
            },
        );
        self.velocities.push(entity_id, Velocity::default());

        self.character_states
            .push(entity_id, CharacterState::default());

        let mut animator = CharacterAnimator::default();
        animator.register(CharacterAnimID::Wait, Self::wait_animation());
        animator.register(CharacterAnimID::Attack, Self::attack_animation());
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
        self.positions
            .push(entity_id, Position { x: 10f32, y: 10f32 });
        self.velocities.push(entity_id, Velocity::default());
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
        System::process(
            &mut self.character_state_observer,
            &(&self.character_states, ForObserverSet()),
        );
        System::process(
            &mut self.character_anim_end_observer,
            &(&self.character_animators, ForObserverSet()),
        );

        System::process(
            &mut self.character_states,
            &(&self.inputs, &self.character_anim_end_observer),
        );
        System::process(&mut self.move_targets, &(&self.teams, &self.positions));
        System::process(&mut self.velocities, &self.inputs);
        System::process(&mut self.velocities, &(&self.positions, &self.move_targets));
        System::process(&mut self.positions, &self.velocities);
        System::process(
            &mut self.character_animators,
            &self.character_state_observer,
        );
        System::process(&mut self.character_animators, &());
        System::process(&mut self.character_views, &self.character_animators);
        System::process(
            &mut self.character_views,
            &(&self.positions, &self.velocities),
        );

        System::process(
            &mut self.character_state_observer,
            &(&self.character_states, ForObserverCheck()),
        );
        System::process(
            &mut self.character_anim_end_observer,
            &(&self.character_animators, ForObserverCheck()),
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
