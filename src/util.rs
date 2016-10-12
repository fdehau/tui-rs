use std::hash::{Hash, SipHasher, Hasher};

use layout::Rect;

pub fn hash<T: Hash>(t: &T, area: &Rect) -> u64 {
    let mut s = SipHasher::new();
    t.hash(&mut s);
    area.hash(&mut s);
    s.finish()
}
