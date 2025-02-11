use std::hash::Hash;

use crate::Data;

pub trait Store<K: Eq + Hash> {

    fn put<V: Data>(key: K, value: V) -> bool;

    fn get<V: Data>(key: K) -> Option<V>;

    fn update<V: Data>(key: K, value: V) -> bool;

    fn has(key: K) -> bool;
    
    fn delete(key: K) -> bool;

    fn size() -> u64;

}