//! std::collections::HashMapの挙動を実行時に毎回同じにするためのモジュール.

use std::collections::HashMap;

/// ## About
/// 
/// HashMapの挙動を実行時に毎回同じにするための関数.
pub fn to_tuple_vec<K, V>(hashmap: &HashMap<K, V>) -> Vec<(K, V)>
where
    K: Clone + Ord,
    V: Clone,
{
    let mut vec = Vec::new();
    for (key, value) in hashmap.iter() {
        vec.push((key.clone(), value.clone()));
    }
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    vec
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_tuple_vec() {
        let mut hashmap = HashMap::new();
        hashmap.insert(1, 2);
        hashmap.insert(3, 4);
        hashmap.insert(5, 6);
        let vec = to_tuple_vec(&hashmap);
        assert_eq!(vec, vec![(1, 2), (3, 4), (5, 6)]);
    }
}
