use quicksilver::prelude::*;

mod components;
mod systems;
use components::*;
use systems::*;

#[derive(Default)]
struct Game {
    input: Input,
    position: Position,
    velocity: Velocity,
    character_view: CharacterView,
}

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Self{position: Position{x: 50f32, y:50f32}, ..Default::default()})
    }

    /// Will happen at a fixed rate of 60 ticks per second under ideal conditions. Under non-ideal conditions,
    /// the game loop will do its best to still call the update at about 60 TPS.
    ///
    /// By default it does nothing
    fn update(&mut self, _window: &mut Window) -> Result<()> {
        update_velocity(&self.input, &mut self.velocity);
        update_position(&self.velocity, &mut self.position);
        update_character_view(&self.position, &mut self.character_view);
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
                        self.input.left = pressed;
                    }
                    Key::D => {
                        self.input.right = pressed;
                    }
                    Key::W => {
                        self.input.up = pressed;
                    }
                    Key::S => {
                        self.input.down = pressed;
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
        update_window(&self.character_view, window);
        Ok(())
    }
}

fn main() {
    run::<Game>("Game", Vector::new(800, 600), Settings::default());
}
