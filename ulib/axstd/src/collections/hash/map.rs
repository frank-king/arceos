#[allow(deprecated)]
use crate::hash::SipHasher;
use crate::hash::{BuildHasher, Hash, Hasher};
use hashbrown::hash_map as base;

mod rand {
    use arceos_api::time;
    use spinlock::SpinNoIrq;

    static PARK_MILLER_LEHMER_SEED: SpinNoIrq<u32> = SpinNoIrq::new(0);
    const RAND_MAX: u64 = 2_147_483_647;

    pub fn random() -> u128 {
        let mut seed = PARK_MILLER_LEHMER_SEED.lock();
        if *seed == 0 {
            *seed = time::ax_current_time().as_nanos() as u32;
        }

        let mut ret = 0u128;
        for _ in 0..4 {
            *seed = ((u64::from(*seed) * 48271) % RAND_MAX) as u32;
            ret = (ret << 32) | (*seed as u128);
        }
        ret
    }
}

pub struct HashMap<K, V, S = RandomState> {
    base: base::HashMap<K, V, S>,
}

impl<K, V> HashMap<K, V, RandomState> {
    #[inline]
    #[must_use]
    pub fn new() -> HashMap<K, V, RandomState> {
        Default::default()
    }
}

impl<K, V, S> HashMap<K, V, S> {
    #[inline]
    pub const fn with_hasher(hash_builder: S) -> HashMap<K, V, S> {
        HashMap {
            base: base::HashMap::with_hasher(hash_builder),
        }
    }
}

#[derive(Clone)]
pub struct RandomState {
    k0: u64,
    k1: u64,
}

impl RandomState {
    #[inline]
    #[must_use]
    pub fn new() -> RandomState {
        let state = rand::random();
        let k0 = (state >> 64) as u64;
        let k1 = state as u64;
        RandomState { k0, k1 }
    }
}

impl BuildHasher for RandomState {
    type Hasher = DefaultHasher;
    #[inline]
    #[allow(deprecated)]
    fn build_hasher(&self) -> DefaultHasher {
        DefaultHasher(SipHasher::new_with_keys(self.k0, self.k1))
    }
}

#[derive(Clone, Debug)]
#[allow(deprecated)]
pub struct DefaultHasher(SipHasher);

impl DefaultHasher {
    #[inline]
    #[allow(deprecated)]
    #[must_use]
    pub fn new() -> DefaultHasher {
        DefaultHasher(SipHasher::new_with_keys(0, 0))
    }
}

impl Default for DefaultHasher {
    /// Creates a new `DefaultHasher` using [`new`].
    /// See its documentation for more.
    ///
    /// [`new`]: DefaultHasher::new
    #[inline]
    fn default() -> DefaultHasher {
        DefaultHasher::new()
    }
}

impl Hasher for DefaultHasher {
    // The underlying `SipHasher13` doesn't override the other
    // `write_*` methods, so it's ok not to forward them here.

    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.write(msg)
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }
}

impl Default for RandomState {
    /// Constructs a new `RandomState`.
    #[inline]
    fn default() -> RandomState {
        RandomState::new()
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
        Self {
            base: self.base.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.base.clone_from(&other.base);
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

impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.base.insert(k, v)
    }
}

impl<K, V, S> HashMap<K, V, S> {
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            base: self.base.iter(),
        }
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    base: base::Iter<'a, K, V>,
}

impl<K, V> Clone for Iter<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Iter {
            base: self.base.clone(),
        }
    }
}

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
impl<K, V> ExactSizeIterator for Iter<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }
}
