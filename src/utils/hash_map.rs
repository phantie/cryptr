use std::{collections::HashMap, hash::Hash};

pub fn invert<K, V>(map: &HashMap<K, V>) -> HashMap<V, K>
where
    V: Eq + Hash + Clone,
    K: Clone,
{
    map.iter()
        .map(|(k, v)| ((*v).clone(), (*k).clone()))
        .collect::<HashMap<_, _>>()
}
