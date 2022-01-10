use std::hash::{Hash, Hasher};
use std::collections::hash_map::{HashMap, RawEntryMut};
use std::hash::BuildHasher;

use crate::HashEq;

/// An extension trait which allows using any key type `Q` that
/// implements `HashEq<K>` and `PartialEq<K>` to perform lookups in a
/// `HashMap`.
pub trait HashMapExt<K: Hash, V, S, Q> {
    /// Looks up a `RawEntryMut` based on a key.
    fn raw_entry_mut_hasheq(&mut self, key: &Q) -> RawEntryMut<'_, K, V, S>;

    /// Looks up a key-value pair based on a key.
    fn get_key_value_hasheq(&self, key: &Q) -> Option<(&K, &V)>;
}

impl<K, V, S, Q> HashMapExt<K, V, S, Q> for HashMap<K, V, S>
    where
    K: Eq + Hash,
    S: BuildHasher,
    Q: Hash + HashEq<K> + PartialEq<K>
{

    fn raw_entry_mut_hasheq(&mut self, key: &Q) -> RawEntryMut<'_, K, V, S>
    {
        let mut h = self.hasher().build_hasher();
        key.hash(&mut h);
        self.raw_entry_mut().from_hash(h.finish(), |k| key == k)
    }

    fn get_key_value_hasheq(&self, key: &Q) -> Option<(&K, &V)> {
        let mut h = self.hasher().build_hasher();
        key.hash(&mut h);
        self.raw_entry().from_hash(h.finish(), |k| key == k)
    }
}

