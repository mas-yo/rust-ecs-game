use quicksilver::prelude::*;
use std::collections::*;

pub(crate) type EntityID = u32;

pub(crate) struct Component<T> {
    entity_id: EntityID,
    inner: T,
}

impl<T> Component<T> {
    pub fn new(entity_id: EntityID, inner: T) -> Self {
        Self {
            entity_id: entity_id,
            inner: inner,
        }
    }
    pub fn entity_id(&self) -> EntityID {
        self.entity_id
    }
    pub fn inner(&self) -> &T {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

pub(crate) type CContainer<T> = ComponentContainer<T>;

#[derive(Default)]
pub(crate) struct ComponentContainer<T> {
    map: HashMap<EntityID, usize>,
    vec: Vec<Component<T>>,
}

impl<T> ComponentContainer<T> {
    pub fn push(&mut self, entity_id: EntityID, item: T) {
        self.vec.push(Component::<T>::new(entity_id, item));
        self.map.insert(entity_id, self.vec.len() - 1);
    }
    pub fn get(&self, entity_id: EntityID) -> Option<&T> {
        let index = self.map.get(&entity_id)?;
        Some(self.vec[*index].inner())
    }
    pub fn get_mut(&mut self, entity_id: EntityID) -> Option<&mut T> {
        let index = self.map.get(&entity_id)?;
        Some(self.vec[*index].inner_mut())
    }
    pub fn iter(&self) -> ComponentIter<T> {
        ComponentIter {
            iter: self.vec.iter(),
        }
    }
    pub fn iter_mut(&mut self) -> ComponentIterMut<T> {
        ComponentIterMut {
            iter: self.vec.iter_mut(),
        }
    }
}

pub(crate) struct ComponentIter<'a, T>
where
    T: 'a,
{
    iter: std::slice::Iter<'a, Component<T>>,
}
impl<'a, T> Iterator for ComponentIter<'a, T> {
    type Item = (EntityID, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        Some((next.entity_id(), next.inner()))
    }
}
impl<'a, T> ComponentIter<'a, T> {
    pub fn zip_entity<U>(self, other: &'a CContainer<U>) -> ZipEntity<'a, T, U> {
        ZipEntity {
            base: self,
            other: other,
        }
    }
    pub fn zip_entity2<U, V>(
        self,
        other1: &'a CContainer<U>,
        other2: &'a CContainer<V>,
    ) -> ZipEntity2<'a, T, U, V> {
        ZipEntity2 {
            base: self,
            other1: other1,
            other2: other2,
        }
    }
}
pub(crate) struct ComponentIterMut<'a, T>
where
    T: 'a,
{
    iter: std::slice::IterMut<'a, Component<T>>,
}
impl<'a, T> Iterator for ComponentIterMut<'a, T> {
    type Item = (EntityID, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        Some((next.entity_id(), next.inner_mut()))
    }
}
impl<'a, T> ComponentIterMut<'a, T> {
    pub fn zip_entity<U>(self, other: &'a CContainer<U>) -> ZipEntityMut<'a, T, U> {
        ZipEntityMut {
            base: self,
            other: other,
        }
    }
    pub fn zip_entity2<U, V>(
        self,
        other1: &'a CContainer<U>,
        other2: &'a CContainer<V>,
    ) -> ZipEntity2Mut<'a, T, U, V> {
        ZipEntity2Mut {
            base: self,
            other1: other1,
            other2: other2,
        }
    }
}

pub(crate) struct ZipEntity<'a, T, U>
where
    T: 'a,
    U: 'a,
{
    base: ComponentIter<'a, T>,
    other: &'a CContainer<U>,
}

impl<'a, T, U> Iterator for ZipEntity<'a, T, U> {
    type Item = (&'a T, &'a U);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((entity_id, base)) = self.base.next() {
            if let Some(other_item) = self.other.get(entity_id) {
                return Some((base, other_item));
            }
        }
        None
    }
}

pub(crate) struct ZipEntityMut<'a, T, U>
where
    T: 'a,
    U: 'a,
{
    base: ComponentIterMut<'a, T>,
    other: &'a CContainer<U>,
}

impl<'a, T, U> Iterator for ZipEntityMut<'a, T, U> {
    type Item = (&'a mut T, &'a U);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((entity_id, base)) = self.base.next() {
            if let Some(other_item) = self.other.get(entity_id) {
                return Some((base, other_item));
            }
        }
        None
    }
}
pub(crate) struct ZipEntity2<'a, T, U, V>
where
    T: 'a,
    U: 'a,
    V: 'a,
{
    base: ComponentIter<'a, T>,
    other1: &'a CContainer<U>,
    other2: &'a CContainer<V>,
}

impl<'a, T, U, V> Iterator for ZipEntity2<'a, T, U, V> {
    type Item = (&'a T, &'a U, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((entity_id, base)) = self.base.next() {
            let other1_item = self.other1.get(entity_id);
            let other2_item = self.other2.get(entity_id);
            if other1_item.is_some() && other2_item.is_some() {
                return Some((base, other1_item.unwrap(), other2_item.unwrap()));
            }
        }
        None
    }
}

pub(crate) struct ZipEntity2Mut<'a, T, U, V>
where
    T: 'a,
    U: 'a,
    V: 'a,
{
    base: ComponentIterMut<'a, T>,
    other1: &'a CContainer<U>,
    other2: &'a CContainer<V>,
}

impl<'a, T, U, V> Iterator for ZipEntity2Mut<'a, T, U, V> {
    type Item = (&'a mut T, &'a U, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((entity_id, base)) = self.base.next() {
            let other1_item = self.other1.get(entity_id);
            let other2_item = self.other2.get(entity_id);
            if other1_item.is_some() && other2_item.is_some() {
                return Some((base, other1_item.unwrap(), other2_item.unwrap()));
            }
        }
        None
    }
}

#[derive(Default)]
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

#[derive(Default)]
pub(crate) struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

pub(crate) type MoveTarget = Vector;

pub(crate) type Velocity = Vector;

pub(crate) type Position = Vector;

#[derive(Default)]
pub(crate) struct CharacterView {
    pub position: Vector,
    pub direction: f32,
    pub radius: f32,
    pub color: Color,
}
