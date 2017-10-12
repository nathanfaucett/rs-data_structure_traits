#[macro_use] extern crate lazy_static;
#[macro_use] extern crate data_structure_traits;


use std::collections::{BTreeMap, HashMap, BTreeSet, HashSet};

use data_structure_traits::*;


fn get<C, I, T>(collection: &C, index: I) -> Option<&T>
    where C: Get<I, Output = T>,
{
    collection.get(index)
}

fn count_len<'a, S, T>(seq: &'a S) -> usize
    where &'a S: IntoIterator<Item = T>,
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

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = collection!{
        0 => "foo",
        1 => "bar",
        2 => "baz"
    };
}

#[test]
fn test_count_len() {
    let vec: Vec<usize> = collection![0, 1, 2, 3];
    assert_eq!(count_len(&vec), 4);

    assert_eq!(count_len(&*HASHMAP), 3);
}
