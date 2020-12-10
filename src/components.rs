use quicksilver::prelude::*;
use std::collections::*;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Default, Clone)]
pub(crate) struct Team {
    team_id: u32,
}

impl Team {
    pub fn new(team_id: u32) -> Self {
        Self { team_id: team_id }
    }
    pub fn team_id(&self) -> u32 {
        self.team_id
    }
}

#[derive(Default, Clone)]
pub(crate) struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub attack: bool,
}

#[derive(Default, Clone)]
pub(crate) struct MoveTarget(pub Vector);

#[derive(Default, Clone)]
pub(crate) struct Velocity(pub Vector);

#[derive(Default, Clone)]
pub(crate) struct Position(pub Vector);

pub(crate) type Direction = f32;

#[derive(Default, Clone)]
pub(crate) struct CharacterView {
    pub position: Vector,
    pub direction: f32,
    pub radius: f32,
    pub radius_scale: f32,
    pub color: Color,
    pub weapon_direction: f32,
}

pub(crate) mod StatusBarType {
    #[derive(Default, Clone)]
    pub struct Health();
    // Heat(()),
}
#[derive(Default, Clone)]
pub(crate) struct StatusBarView<T> {
    pub position: Vector,
    pub color: Color,
    pub frame_length: i32,
    pub animated_length: i32,
    pub current_length: i32,
    phantom: PhantomData<T>,
}

impl<T> StatusBarView<T> {
    pub fn new(length: i32, color: Color) -> Self {
        Self {
            position: Vector::new(0f32, 0f32),
            color: color,
            frame_length: length,
            current_length: length,
            animated_length: length,
            phantom: PhantomData,
        }
    }
}

#[derive(Default, Clone)]
pub(crate) struct CharacterAnimFrame {
    pub radius_scale: f32,
    pub weapon_direction: f32,
    pub move_forward: f32,
}

#[derive(Default, Clone)]
pub(crate) struct Animator<K, V>
where
    K: Hash + Eq,
{
    playing_id: Option<K>,
    current_frame: usize,
    animations: HashMap<K, Animation<V>>,
}

impl<K, V> Animator<K, V>
where
    K: Hash + Eq + Copy,
{
    pub fn play(&mut self, animation_id: K) {
        if self.animations.contains_key(&animation_id) {
            self.playing_id = Some(animation_id);
            self.current_frame = 0;
        }
    }
    pub fn is_end(&self) -> bool {
        if let Some(id) = self.playing_id {
            let anim = self.animations.get(&id).unwrap();
            return !anim.looped && self.current_frame == anim.values.len();
        }
        return false;
    }
    pub fn update(&mut self) {
        if let Some(id) = self.playing_id {
            if let Some(anim) = self.animations.get(&id) {
                self.current_frame += 1;
                if anim.values.len() <= self.current_frame && anim.looped {
                    self.current_frame = 0;
                }
            }
        }
    }
    pub fn register(&mut self, id: K, anim: Animation<V>) {
        self.animations.insert(id, anim);
    }
    pub fn value(&self) -> Option<&V> {
        let id = self.playing_id?;
        let anim = self.animations.get(&id)?;
        anim.values.get(self.current_frame)
    }
    pub fn playing_id(&self) -> Option<K> {
        self.playing_id
    }
}

#[derive(Default, Clone)]
pub(crate) struct Animation<T> {
    looped: bool,
    values: Vec<T>,
}

impl<T> Animation<T> {
    pub fn new(looped: bool, values: Vec<T>) -> Self {
        Self { looped, values }
    }
}

// #[derive(Default,Clone)]
// pub(crate) struct WeaponHit {
//     pub hit: bool,
// }

#[derive(Default, Clone)]
pub(crate) struct SwordCollider {
    pub active: bool,
    pub line: quicksilver::geom::Line,
}
impl SwordCollider {
    pub fn is_collided(&self, body: &BodyDefenseCollider) -> bool {
        if self.active == false {
            false
        } else {
            body.circle.overlaps(&self.line)
        }
    }
}

#[derive(Default, Clone)]
pub(crate) struct BodyWeaponCollider {
    pub circle: quicksilver::geom::Circle,
}

impl BodyWeaponCollider {
    pub fn is_collided(&self, body: &BodyDefenseCollider) -> bool {
        body.circle.overlaps(&self.circle)
    }
}

#[derive(Default, Clone)]
pub(crate) struct BodyDefenseCollider {
    pub hit: bool,
    pub circle: quicksilver::geom::Circle,
}

#[derive(Default, Clone)]
pub(crate) struct Health {
    pub max_health: i32,
    pub current_health: i32,
}

impl Health {
    pub fn new(health: i32) -> Self {
        Self {
            max_health: health,
            current_health: health,
        }
    }
    pub fn ratio(&self) -> f32 {
        self.current_health as f32 / self.max_health as f32
    }
}

pub(crate) struct ValueObserver<V, C> {
    prev_changed: bool,
    changed: bool,
    value: V,
    check_fn: fn(&C) -> V,
}

impl<V, C> ValueObserver<V, C>
where
    V: PartialEq + Copy,
{
    pub fn new(value: V, check_fn: fn(&C) -> V) -> Self {
        Self {
            prev_changed: false,
            changed: false,
            value: value,
            check_fn: check_fn,
        }
    }
    pub fn set(&mut self, component: &C) {
        self.prev_changed = self.changed;
        self.changed = false;
        self.value = (self.check_fn)(component);
    }
    pub fn check(&mut self, component: &C) -> bool {
        self.changed = (self.check_fn)(component) != self.value;
        self.changed
    }
    pub fn value(&self) -> V {
        self.value
    }
    pub fn is_changed(&self) -> bool {
        self.prev_changed
    }
}
