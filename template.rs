#![feature(error_in_core)]
use self::Entry::*;

use hashbrown::hash_map as base;
use core::borrow::Borrow;
extern crate alloc;


use core::fmt::{self, Debug};
#[allow(deprecated)]
use core::hash::{BuildHasher, Hash, Hasher};
use core::ops::Index;

   

pub struct HashMap<K, V, S = RandomState> {
    base: base::HashMap<K, V, S>,
}

impl<K, V> HashMap<K, V, RandomState> {
    pub fn new() -> HashMap<K, V, RandomState> {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> HashMap<K, V, RandomState> {
        HashMap::with_capacity_and_hasher(capacity, Default::default())
    }
}

impl<K, V, S> HashMap<K, V, S> {


    pub const fn with_hasher(hash_builder: S) -> HashMap<K, V, S> {
        HashMap { base: base::HashMap::with_hasher(hash_builder) }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> HashMap<K, V, S> {
        HashMap { base: base::HashMap::with_capacity_and_hasher(capacity, hasher) }
    }

    pub fn capacity(&self) -> usize {
        self.base.capacity()
    }


    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys { inner: self.iter() }
    }


    pub fn values(&self) -> Values<'_, K, V> {
        Values { inner: self.iter() }
    }


    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter { base: self.base.iter() }
    }


    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut { base: self.base.iter_mut() }
    }

    pub fn len(&self) -> usize {
        self.base.len()
    }

    pub fn is_empty(&self) -> bool {
        self.base.is_empty()
    }




    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.base.retain(f)
    }


    pub fn clear(&mut self) {
        self.base.clear();
    }


    pub fn hasher(&self) -> &S {
        self.base.hasher()
    }
}


impl<K, V, S> Clone for HashMap<K, V, S>
where
    K: Clone,
    V: Clone,
    S: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self { base: self.base.clone() }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.base.clone_from(&other.base);
    }
}


impl<K, V, S> PartialEq for HashMap<K, V, S>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
{
    fn eq(&self, other: &HashMap<K, V, S>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}


impl<K, V, S> Eq for HashMap<K, V, S>
where
    K: Eq + Hash,
    V: Eq,
    S: BuildHasher,
{
}


impl<K, V, S> Debug for HashMap<K, V, S>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}


impl<K, V, S> Default for HashMap<K, V, S>
where
    S: Default,
{
    /// Creates an empty `HashMap<K, V, S>`, with the `Default` value for the hasher.
    #[inline]
    fn default() -> HashMap<K, V, S> {
        HashMap::with_hasher(Default::default())
    }
}


impl<K, Q: ?Sized, V, S> Index<&Q> for HashMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash,
    S: BuildHasher,
{
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `HashMap`.
    #[inline]
    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}


// Note: as what is currently the most convenient built-in way to construct
// a HashMap, a simple usage of this function must not *require* the user
// to provide a type annotation in order to infer the third type parameter
// (the hasher parameter, conventionally "S").
// To that end, this impl is defined using RandomState as the concrete
// type of S, rather than being generic over `S: BuildHasher + Default`.
// It is expected that users who want to specify a hasher will manually use
// `with_capacity_and_hasher`.
// If type parameter defaults worked on impls, and if type parameter
// defaults could be mixed with const generics, then perhaps
// this could be generalized.
// See also the equivalent impl on HashSet.
impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V, RandomState>
where
    K: Eq + Hash,
{
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let map1 = HashMap::from([(1, 2), (3, 4)]);
    /// let map2: HashMap<_, _> = [(1, 2), (3, 4)].into();
    /// assert_eq!(map1, map2);
    /// ```
    fn from(arr: [(K, V); N]) -> Self {
        Self::from_iter(arr)
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    base: base::Iter<'a, K, V>,
}

// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
 
impl<K, V> Clone for Iter<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Iter { base: self.base.clone() }
    }
}

impl<K: Debug, V: Debug> fmt::Debug for Iter<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

pub struct IterMut<'a, K: 'a, V: 'a> {
    base: base::IterMut<'a, K, V>,
}

impl<'a, K, V> IterMut<'a, K, V> {
    /// Returns an iterator of references over the remaining items.
    #[inline]
    pub(super) fn iter(&self) -> Iter<'_, K, V> {
        Iter { base: self.base.rustc_iter() }
    }
}

pub struct IntoIter<K, V> {
    base: base::IntoIter<K, V>,
}

impl<K, V> IntoIter<K, V> {
    /// Returns an iterator of references over the remaining items.
    #[inline]
    pub(super) fn iter(&self) -> Iter<'_, K, V> {
        Iter { base: self.base.rustc_iter() }
    }
}


   
pub struct Keys<'a, K: 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
   
impl<K, V> Clone for Keys<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Keys { inner: self.inner.clone() }
    }
}

impl<K: Debug, V> fmt::Debug for Keys<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}


   
pub struct Values<'a, K: 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
   
impl<K, V> Clone for Values<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Values { inner: self.inner.clone() }
    }
}

impl<K, V: Debug> fmt::Debug for Values<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This `enum` is constructed from the [`entry`] method on [`HashMap`].
///
/// [`entry`]: HashMap::entry
pub enum Entry<'a, K: 'a, V: 'a> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V>),

    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V>),
}

impl<K: Debug, V: Debug> Debug for Entry<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Vacant(ref v) => f.debug_tuple("Entry").field(v).finish(),
            Occupied(ref o) => f.debug_tuple("Entry").field(o).finish(),
        }
    }
}

/// A view into an occupied entry in a `HashMap`.
/// It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, K: 'a, V: 'a> {
    base: base::RustcOccupiedEntry<'a, K, V>,
}

impl<K: Debug, V: Debug> Debug for OccupiedEntry<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish_non_exhaustive()
    }
}

/// A view into a vacant entry in a `HashMap`.
/// It is part of the [`Entry`] enum.
pub struct VacantEntry<'a, K: 'a, V: 'a> {
    base: base::RustcVacantEntry<'a, K, V>,
}

impl<K: Debug, V> Debug for VacantEntry<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("VacantEntry").field(self.key()).finish()
    }
}

/// The error returned by [`try_insert`](HashMap::try_insert) when the key already exists.
///
/// Contains the occupied entry, and the value that was not inserted.
pub struct OccupiedError<'a, K: 'a, V: 'a> {
    /// The entry in the map that was already occupied.
    pub entry: OccupiedEntry<'a, K, V>,
    /// The value which was not inserted, because the entry was already occupied.
    pub value: V,
}

impl<K: Debug, V: Debug> Debug for OccupiedError<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedError")
            .field("key", self.entry.key())
            .field("old_value", self.entry.get())
            .field("new_value", &self.value)
            .finish_non_exhaustive()
    }
}

impl<'a, K: Debug, V: Debug> fmt::Display for OccupiedError<'a, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to insert {:?}, key {:?} already exists with value {:?}",
            self.value,
            self.entry.key(),
            self.entry.get(),
        )
    }
}
