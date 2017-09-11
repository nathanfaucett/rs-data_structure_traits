#[macro_use] extern crate data_structure_traits;


use std::collections::{BTreeMap, HashMap, BTreeSet, HashSet};

use data_structure_traits::*;


fn get<C, I, T>(collection: &C, index: I) -> Option<&T>
    where C: Get<I, Output = T>,
{
    collection.get(index)
}


#[test]
fn test_get() {
    let vec: Vec<usize> = collection![0];
    assert_eq!(get(&vec, 0), Some(&0));

    let map: BTreeMap<usize, usize> = collection!{0 => 0};
    assert_eq!(get(&map, &0), Some(&0));
    let map: HashMap<usize, usize> = collection!{0 => 0};
    assert_eq!(get(&map, &0), Some(&0));

    let _: BTreeSet<usize> = collection![0];
    let _: HashSet<usize> = collection![0];
}
