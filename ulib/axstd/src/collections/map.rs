
use hashbrown::hash_map as base;
extern crate alloc;
use axhal::time;
use core::borrow::Borrow;

#[allow(deprecated)]
use core::hash::{BuildHasher, Hasher, Hash, SipHasher13};

use spinlock::SpinNoIrq; 
static PARK_MILLER_LEHMER_SEED: SpinNoIrq<u32> = SpinNoIrq::new(0); 
const RAND_MAX: u64 = 2_147_483_647;

pub fn random() -> u128{
     let mut seed = PARK_MILLER_LEHMER_SEED.lock(); 
     if *seed == 0 {
         *seed = time::current_ticks() as u32; 
        }
        let mut ret: u128 = 0; 
        for _ in 0..4 { *seed = ((u64::from(*seed) * 48271) % RAND_MAX) as u32; 
            ret = (ret << 32) | (*seed as u128); 
        }
        ret 
    }

pub struct RandomState {
    k0: u64,
    k1: u64,
}

impl RandomState {
    pub fn new() -> Self {
        let seed = random();
        let k0 = (seed & 0xFFFF_FFFF_FFFF_FFFF) as u64;
        let k1 = ((seed >> 64) & 0xFFFF_FFFF_FFFF_FFFF) as u64;
        RandomState { k0, k1 }
    }
}

impl BuildHasher for RandomState {
    type Hasher = DefaultHasher;
    #[inline]
    #[allow(deprecated)]
    fn build_hasher(&self) -> DefaultHasher {
        DefaultHasher(SipHasher13::new_with_keys(self.k0, self.k1))
    }
}


pub struct DefaultHasher(SipHasher13);

impl DefaultHasher {
    /// 创建一个新的 `DefaultHasher`。
    ///
    /// 不保证此哈希值与所有其他 `DefaultHasher` 实例相同，但与通过 `new` 或 `default` 创建的所有其他 `DefaultHasher` 实例相同。
    ///
    ///
    pub const fn new() -> DefaultHasher {
        DefaultHasher(SipHasher13::new_with_keys(0, 0))
    }
}

impl Default for DefaultHasher {
    /// 使用 [`new`] 创建一个新的 `DefaultHasher`。
    /// 有关更多信息，请参见其文档。
    ///
    /// [`new`]: DefaultHasher::new
    #[inline]
    fn default() -> DefaultHasher {
        DefaultHasher::new()
    }
}

impl Hasher for DefaultHasher {
    // 底层 `SipHasher13` 不会覆盖其他 `write_*` 方法，所以这里不转发也没关系。
    //

    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.write(msg)
    }

    #[inline]
    fn write_str(&mut self, s: &str) {
        self.0.write_str(s);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }
}

impl Default for RandomState {
    /// 创建一个新的 `RandomState`。
    #[inline]
    fn default() -> RandomState {
        RandomState::new()
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    base: base::Iter<'a, K, V>,
}
pub struct Keys<'a, K: 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

pub struct IterMut<'a, K: 'a, V: 'a> {
    base: base::IterMut<'a, K, V>,
}


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

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter { base: self.base.iter() }
    }
    pub fn is_empty(&self) -> bool {
        self.base.is_empty()
    }
    pub fn clear(&mut self) {
        self.base.clear();
    }

}

impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    pub fn reserve(&mut self, additional: usize) {
        self.base.reserve(additional)
    }
    pub fn shrink_to_fit(&mut self) {
        self.base.shrink_to_fit();
    }
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.base.shrink_to(min_capacity);
    }
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.base.get(k)
    }
    pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.base.get_key_value(k)
    }
    pub fn get_many_mut<Q: ?Sized, const N: usize>(&mut self, ks: [&Q; N]) -> Option<[&'_ mut V; N]>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.base.get_many_mut(ks)
    }
    pub unsafe fn get_many_unchecked_mut<Q: ?Sized, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> Option<[&'_ mut V; N]>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.base.get_many_unchecked_mut(ks)
    }
    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.base.contains_key(k)
    }
    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.base.get_mut(k)
    }
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.base.insert(k, v)
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


// impl<K, V, S> IntoIterator for HashMap<K, V, S> {
//     type Item = (K, V);
//     type IntoIter = IntoIter<K, V>;

//     #[inline]
//     #[rustc_lint_query_instability]
//     fn into_iter(self) -> IntoIter<K, V> {
//         IntoIter { base: self.base.into_iter() }
//     }
// }

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.base.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}