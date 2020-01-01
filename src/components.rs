use quicksilver::prelude::*;
use std::collections::*;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::slice::IterMut;

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
}

impl<T> Deref for Component<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for Component<T> {
    fn deref_mut(&mut self) -> &mut T {
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
    pub fn get(&self, entity_id: EntityID) -> Option<&Component<T>> {
        let index = self.map.get(&entity_id)?;
        Some(&self.vec[*index])
    }
    pub fn iter_mut(&mut self) -> IterMut<Component<T>> {
        self.vec.iter_mut()
    }
}

pub(crate) fn get_component2<'a, 'b, T, U>(
    c1: &'a CContainer<T>,
    c2: &'b CContainer<U>,
    entity_id: EntityID,
) -> Option<(&'a Component<T>, &'b Component<U>)> {
    let cmp1 = CContainer::<T>::get(c1, entity_id)?;
    let cmp2 = CContainer::<U>::get(c2, entity_id)?;
    Some((cmp1, cmp2))
}

impl<T> Deref for ComponentContainer<T> {
    type Target = Vec<Component<T>>;
    fn deref(&self) -> &Self::Target {
        &self.vec
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
