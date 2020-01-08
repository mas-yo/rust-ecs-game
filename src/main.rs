use quicksilver::prelude::*;

mod components;
mod systems;

use components::*;
use systems::*;

#[derive(Default)]
struct Game {
    current_entity_id: EntityID,
    inputs: CContainer<Input>,
    static_status: CContainer<Team>,
    move_targets: CContainer<MoveTarget>,
    positions: CContainer<Position>,
    velocities: CContainer<Velocity>,
    character_views: CContainer<CharacterView>,
}

impl Game {
    fn create_hero(&mut self) {
        let entity_id = self.current_entity_id;

        self.inputs.push(entity_id, Input::default());
        self.static_status.push(entity_id, Team::new(0));
        self.positions.push(entity_id, Position{x: 150f32, y:150f32 });
        self.velocities.push(entity_id, Velocity::default());
        self.character_views.push(entity_id, CharacterView{color:Color::GREEN, radius:10f32, ..Default::default()});

        self.current_entity_id = self.current_entity_id + 1;
    }

    fn create_enemy(&mut self) {
        let entity_id = self.current_entity_id;

        self.move_targets.push(entity_id, MoveTarget::default());
        self.static_status.push(entity_id, Team::new(1));
        self.positions.push(entity_id, Position{x: 10f32, y:10f32 });
        self.velocities.push(entity_id, Velocity::default());
        self.character_views.push(entity_id, CharacterView{color:Color::RED, radius:15f32, ..Default::default()});

        self.current_entity_id = self.current_entity_id + 1;
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
        System::process(&(&self.static_status, &self.positions), &mut self.move_targets);
        System::process(&self.inputs, &mut self.velocities);
        System::process(&(&self.positions, &self.move_targets), &mut self.velocities);
        System::process(&self.velocities, &mut self.positions);
        System::process(&self.positions, &mut self.character_views);
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
                        self.inputs.iter_mut().for_each(|(_, i)|{i.left = pressed;});
                    }
                    Key::D => {
                        self.inputs.iter_mut().for_each(|(_, i)|{i.right = pressed;});
                    }
                    Key::W => {
                        self.inputs.iter_mut().for_each(|(_, i)|{i.up = pressed;});
                    }
                    Key::S => {
                        self.inputs.iter_mut().for_each(|(_, i)|{i.down = pressed;});
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
        System::process(&self.character_views, window);
        Ok(())
    }
}

fn main() {
    run::<Game>("Game", Vector::new(800, 600), Settings::default());
}
