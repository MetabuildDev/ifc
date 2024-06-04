pub mod deserialize;
mod serialize;

use std::{any::Any, collections::BTreeMap, fmt::Display, mem::transmute};

use crate::id::Id;

pub trait ParsedIfcType: Any + Display {}

/// CRITICAL: split up the index map into a proper struct with fields which hold Hashmaps mapping
/// indices to one specific type instead of an enum
pub struct DataMap(BTreeMap<Id, Box<dyn Display>>);

impl DataMap {
    pub fn insert<T: Display + 'static>(&mut self, id: Id, value: T) -> Option<T> {
        self.0
            .insert(id, Box::new(value))
            .map(|any| *Self::downcast_unchecked(any))
    }

    pub fn insert_if_not_exists<T: Default + Display + 'static>(&mut self, id: Id) {
        if !self.contains(id) {
            self.insert(id, T::default());
        }
    }

    pub fn remove<T: Display>(&mut self, id: Id) -> Option<T> {
        self.0.remove(&id).map(|any| *Self::downcast_unchecked(any))
    }

    pub fn remove_untyped(&mut self, id: Id) -> Option<Box<dyn Display>> {
        self.0.remove(&id)
    }

    pub fn get<T: Display>(&self, id: Id) -> &T {
        self.0
            .get(&id)
            .map(|any| Self::downcast_ref_unchecked(any))
            .unwrap()
    }

    pub fn get_mut<T: Display>(&mut self, id: Id) -> &mut T {
        self.0
            .get_mut(&id)
            .map(|any| Self::downcast_mut_unchecked(any))
            .unwrap()
    }

    pub fn contains(&self, id: Id) -> bool {
        self.0.contains_key(&id)
    }
}

impl DataMap {
    fn downcast_unchecked<T: Display>(boxed: Box<dyn Display>) -> Box<T> {
        unsafe { Box::from_raw(Box::into_raw(boxed) as *mut T) }
    }

    fn downcast_ref_unchecked<T: Display>(boxed_ref: &Box<dyn Display>) -> &T {
        unsafe {
            let ptr_to_ptr: *const *const T =
                transmute(destructure_traitobject::data(boxed_ref as *const _));

            &**ptr_to_ptr
        }
    }

    fn downcast_mut_unchecked<T: Display>(boxed_ref: &mut Box<dyn Display>) -> &mut T {
        unsafe {
            let ptr_to_ptr: *mut *mut T =
                transmute(destructure_traitobject::data(boxed_ref as *mut _));

            &mut **ptr_to_ptr
        }
    }
}

impl<I> From<I> for DataMap
where
    I: IntoIterator<Item = (Id, Box<dyn Display>)>,
{
    fn from(value: I) -> Self {
        Self(value.into_iter().collect())
    }
}
