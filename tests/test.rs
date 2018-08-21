#[macro_use]
extern crate data_structure_traits;

#[cfg(feature = "hashmap_core")]
extern crate hashmap_core;

use std::collections::{BTreeMap, BTreeSet};

#[cfg(feature = "hashmap_core")]
use hashmap_core::{FnvHashMap as HashMap, FnvHashSet as HashSet};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

use data_structure_traits::*;

fn get<C, I, T>(collection: &C, index: I) -> Option<&T>
where
    C: Get<I, Output = T>,
{
    collection.get(index)
}

fn count_len<'a, S, T>(seq: &'a S) -> usize
where
    S: Seq<'a, T>,
    &'a S: IntoIterator<Item = &'a T>,
    T: 'a,
{
    let mut count = 0;
    for _ in seq {
        count += 1;
    }
    count
}

#[test]
fn test_get() {
    let vec: Vec<usize> = collection![0];
    assert_eq!(get(&vec, 0), Some(&0));

    let map: BTreeMap<usize, usize> = collection!{0 => 0};
    assert_eq!(get(&map, &0), Some(&0));

    let map: HashMap<String, String> = collection!{"0".into() => "0".into()};
    assert_eq!(get(&map, "0"), Some(&String::from("0")));

    let map: BTreeSet<usize> = collection!{0};
    assert_eq!(get(&map, &0), Some(&0));

    let map: HashSet<usize> = collection!{0};
    assert_eq!(get(&map, &0), Some(&0));
}

#[test]
fn test_count_len() {
    let vec: Vec<usize> = collection![0, 1, 2, 3];
    assert_eq!(count_len(&vec), 4);
}
