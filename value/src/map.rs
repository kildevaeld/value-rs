#[cfg(not(feature = "std"))]
use alloc::{
    collections::{btree_map::IntoIter, BTreeMap},
    string::{String, ToString},
};
#[cfg(feature = "std")]
use std::{
    collections::{btree_map::IntoIter, BTreeMap},
    string::String,
};

use core::ops;

use crate::Value;

#[cfg_attr(
    not(feature = "ordered_float"),
    derive(Debug, Clone, PartialEq, PartialOrd, Default)
)]
#[cfg_attr(
    feature = "ordered_float",
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)
)]
pub struct Map {
    inner: BTreeMap<String, Value>,
}

impl Map {
    pub fn set(&mut self, name: impl ToString, value: impl Into<Value>) -> Option<Value> {
        self.inner.insert(name.to_string(), value.into())
    }

    pub fn get(&mut self, name: impl AsRef<str>) -> Option<&Value> {
        self.inner.get(name.as_ref())
    }

    pub fn get_mut(&mut self, name: impl AsRef<str>) -> Option<&mut Value> {
        self.inner.get_mut(name.as_ref())
    }

    pub fn contains(&self, name: impl AsRef<str>) -> bool {
        self.inner.contains_key(name.as_ref())
    }

    pub fn remove(&mut self, name: impl AsRef<str>) -> Option<Value> {
        self.inner.remove(name.as_ref())
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a String, &'a Value)> + 'a {
        self.inner.iter()
    }

    pub fn iter_mut<'a>(&'a self) -> impl Iterator<Item = (&'a String, &'a Value)> + 'a {
        self.inner.iter()
    }
}

impl IntoIterator for Map {
    type Item = (String, Value);
    type IntoIter = IntoIter<String, Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> ops::Index<&'a str> for Map {
    type Output = Value;

    fn index(&self, index: &'a str) -> &Value {
        static NULL: Value = Value::None;
        self.inner.get(index).unwrap_or(&NULL)
    }
}

impl<'a> ops::IndexMut<&'a str> for Map {
    fn index_mut(&mut self, index: &'a str) -> &mut Value {
        if !self.contains(index) {
            self.inner.insert(index.to_string(), Value::None);
        }
        self.inner.get_mut(index).unwrap()
    }
}

impl From<Map> for Value {
    fn from(map: Map) -> Value {
        Value::Map(map.inner)
    }
}
