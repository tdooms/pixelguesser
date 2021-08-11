use std::ops::Range;
use rand::Rng;

pub fn generate_id(validator: impl Fn(u64) -> bool, range: Range<u64>) -> Option<u64> {
    // Generate random ID's and try to make a session
    for _ in 0..10 {
        let id = rand::thread_rng().gen_range(range.clone());
        if validator(id) {
            return Some(id)
        }
    }
    None
}