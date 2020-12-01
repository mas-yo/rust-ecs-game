use quicksilver::prelude::*;
use std::collections::*;
use std::hash::Hash;
use std::marker::PhantomData;

use static_ecs::*;

// pub(crate) type EntityID = u32;

// pub(crate) struct Component<T> {
//     entity_id: EntityID,
//     inner: T,
// }

// impl<T> Component<T> {
//     pub fn new(entity_id: EntityID, inner: T) -> Self {
//         Self {
//             entity_id: entity_id,
//             inner: inner,
//         }
//     }
//     pub fn entity_id(&self) -> EntityID {
//         self.entity_id
//     }
//     pub fn inner(&self) -> &T {
//         &self.inner
//     }
//     pub fn inner_mut(&mut self) -> &mut T {
//         &mut self.inner
//     }
// }

// pub(crate) type CContainer<T> = ComponentContainer<T>;

// // #[derive(Default,Clone)]
// pub(crate) struct ComponentContainer<T> {
//     map: HashMap<EntityID, usize>,
//     vec: Vec<Component<T>>,
// }

// impl<T> Default for ComponentContainer<T> {
//     fn default() -> Self {
//         Self {
//             map: HashMap::new(),
//             vec: Vec::new(),
//         }
//     }
// }

// impl<T> ComponentContainer<T> {
//     pub fn push(&mut self, entity_id: EntityID, item: T) {
//         self.vec.push(Component::<T>::new(entity_id, item));
//         self.map.insert(entity_id, self.vec.len() - 1);
//     }
//     pub fn get(&self, entity_id: EntityID) -> Option<&T> {
//         let index = self.map.get(&entity_id)?;
//         Some(self.vec[*index].inner())
//     }
//     pub fn get_mut(&mut self, entity_id: EntityID) -> Option<&mut T> {
//         let index = self.map.get(&entity_id)?;
//         Some(self.vec[*index].inner_mut())
//     }
//     pub fn iter(&self) -> ComponentIter<T> {
//         ComponentIter {
//             iter: self.vec.iter(),
//         }
//     }
//     pub fn iter_mut(&mut self) -> ComponentIterMut<T> {
//         ComponentIterMut {
//             iter: self.vec.iter_mut(),
//         }
//     }
// }

// pub(crate) struct ComponentIter<'a, T>
// where
//     T: 'a,
// {
//     iter: std::slice::Iter<'a, Component<T>>,
// }
// impl<'a, T> Iterator for ComponentIter<'a, T> {
//     type Item = (EntityID, &'a T);
//     fn next(&mut self) -> Option<Self::Item> {
//         let next = self.iter.next()?;
//         Some((next.entity_id(), next.inner()))
//     }
// }
// impl<'a, T> ComponentIter<'a, T> {
//     pub fn zip_entity<U>(self, other: &'a CContainer<U>) -> ZipEntity<'a, T, U> {
//         ZipEntity {
//             base: self,
//             other: other,
//         }
//     }
//     pub fn zip_entity2<U, V>(
//         self,
//         other1: &'a CContainer<U>,
//         other2: &'a CContainer<V>,
//     ) -> ZipEntity2<'a, T, U, V> {
//         ZipEntity2 {
//             base: self,
//             other1: other1,
//             other2: other2,
//         }
//     }
// }
// pub(crate) struct ComponentIterMut<'a, T>
// where
//     T: 'a,
// {
//     iter: std::slice::IterMut<'a, Component<T>>,
// }
// impl<'a, T> Iterator for ComponentIterMut<'a, T> {
//     type Item = (EntityID, &'a mut T);
//     fn next(&mut self) -> Option<Self::Item> {
//         let next = self.iter.next()?;
//         Some((next.entity_id(), next.inner_mut()))
//     }
// }
// impl<'a, T> ComponentIterMut<'a, T> {
//     pub fn zip_entity<U>(self, other: &'a CContainer<U>) -> ZipEntityMut<'a, T, U> {
//         ZipEntityMut {
//             base: self,
//             other: other,
//         }
//     }
//     pub fn zip_entity2<U, V>(
//         self,
//         other1: &'a CContainer<U>,
//         other2: &'a CContainer<V>,
//     ) -> ZipEntity2Mut<'a, T, U, V> {
//         ZipEntity2Mut {
//             base: self,
//             other1: other1,
//             other2: other2,
//         }
//     }

//     pub fn zip_entity3<U, V, W>(
//         self,
//         other1: &'a CContainer<U>,
//         other2: &'a CContainer<V>,
//         other3: &'a CContainer<W>,
//     ) -> ZipEntity3Mut<'a, T, U, V, W> {
//         ZipEntity3Mut {
//             base: self,
//             other1: other1,
//             other2: other2,
//             other3: other3,
//         }
//     }
// }

// pub(crate) struct ZipEntity<'a, T, U>
// where
//     T: 'a,
//     U: 'a,
// {
//     base: ComponentIter<'a, T>,
//     other: &'a CContainer<U>,
// }

// impl<'a, T, U> Iterator for ZipEntity<'a, T, U> {
//     type Item = (EntityID, &'a T, &'a U);
//     fn next(&mut self) -> Option<Self::Item> {
//         while let Some((entity_id, base)) = self.base.next() {
//             if let Some(other_item) = self.other.get(entity_id) {
//                 return Some((entity_id, base, other_item));
//             }
//         }
//         None
//     }
// }

// pub(crate) struct ZipEntityMut<'a, T, U>
// where
//     T: 'a,
//     U: 'a,
// {
//     base: ComponentIterMut<'a, T>,
//     other: &'a CContainer<U>,
// }

// impl<'a, T, U> Iterator for ZipEntityMut<'a, T, U> {
//     type Item = (EntityID, &'a mut T, &'a U);
//     fn next(&mut self) -> Option<Self::Item> {
//         while let Some((entity_id, base)) = self.base.next() {
//             if let Some(other_item) = self.other.get(entity_id) {
//                 return Some((entity_id, base, other_item));
//             }
//         }
//         None
//     }
// }
// pub(crate) struct ZipEntity2<'a, T, U, V>
// where
//     T: 'a,
//     U: 'a,
//     V: 'a,
// {
//     base: ComponentIter<'a, T>,
//     other1: &'a CContainer<U>,
//     other2: &'a CContainer<V>,
// }

// impl<'a, T, U, V> Iterator for ZipEntity2<'a, T, U, V> {
//     type Item = (EntityID, &'a T, &'a U, &'a V);
//     fn next(&mut self) -> Option<Self::Item> {
//         while let Some((entity_id, base)) = self.base.next() {
//             let other1_item = self.other1.get(entity_id);
//             let other2_item = self.other2.get(entity_id);
//             if other1_item.is_some() && other2_item.is_some() {
//                 return Some((entity_id, base, other1_item.unwrap(), other2_item.unwrap()));
//             }
//         }
//         None
//     }
// }

// pub(crate) struct ZipEntity2Mut<'a, T, U, V>
// where
//     T: 'a,
//     U: 'a,
//     V: 'a,
// {
//     base: ComponentIterMut<'a, T>,
//     other1: &'a CContainer<U>,
//     other2: &'a CContainer<V>,
// }

// impl<'a, T, U, V> Iterator for ZipEntity2Mut<'a, T, U, V> {
//     type Item = (EntityID, &'a mut T, &'a U, &'a V);
//     fn next(&mut self) -> Option<Self::Item> {
//         while let Some((entity_id, base)) = self.base.next() {
//             let other1_item = self.other1.get(entity_id);
//             let other2_item = self.other2.get(entity_id);
//             if other1_item.is_some() && other2_item.is_some() {
//                 return Some((entity_id, base, other1_item.unwrap(), other2_item.unwrap()));
//             }
//         }
//         None
//     }
// }

// pub(crate) struct ZipEntity3Mut<'a, T, U, V, W>
// where
//     T: 'a,
//     U: 'a,
//     V: 'a,
//     W: 'a,
// {
//     base: ComponentIterMut<'a, T>,
//     other1: &'a CContainer<U>,
//     other2: &'a CContainer<V>,
//     other3: &'a CContainer<W>,
// }

// impl<'a, T, U, V, W> Iterator for ZipEntity3Mut<'a, T, U, V, W> {
//     type Item = (&'a mut T, &'a U, &'a V, &'a W);
//     fn next(&mut self) -> Option<Self::Item> {
//         while let Some((entity_id, base)) = self.base.next() {
//             let other1_item = self.other1.get(entity_id);
//             let other2_item = self.other2.get(entity_id);
//             let other3_item = self.other3.get(entity_id);
//             if other1_item.is_some() && other2_item.is_some() && other3_item.is_some() {
//                 return Some((
//                     base,
//                     other1_item.unwrap(),
//                     other2_item.unwrap(),
//                     other3_item.unwrap(),
//                 ));
//             }
//         }
//         None
//     }
// }

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
