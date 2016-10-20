use std::collections::hash_map::RandomState;
use std::hash::{Hash, Hasher, BuildHasher};

use layout::Rect;

pub fn hash<T: Hash>(t: &T, area: &Rect) -> u64 {
    let state = RandomState::new();
    let mut hasher = state.build_hasher();
    t.hash(&mut hasher);
    area.hash(&mut hasher);
    hasher.finish()
}
