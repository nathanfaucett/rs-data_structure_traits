# data_structure_traits [![docs](https://docs.rs/data_structure_traits/badge.svg)](https://docs.rs/crate/data_structure_traits)

data structure collection traits

```rust
#[macro_use]
extern crate data_structure_traits;

use data_structure_traits::*;

fn use_seq<'a, S>(s: &'a mut S) -> f32
where
    S: SeqMut<'a, f32>,
    &'a S: IntoIterator<Item = &'a f32>,
    &'a mut S: IntoIterator<Item = &'a mut f32>,
{
    if s.len() >= 5 {
        s.get_mut(4).map(|v| {
            *v = 0.0;
        });
    } else {
        s.insert(0, 1.0);
    }

    let mut avg = 0.0;
    let len = s.len();

    for v in s.into_iter() {
        avg += *v;
    }

    avg / (len as f32)
}

fn main() {
    let mut v: Vec<f32> = collection![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    println!("{:?}", use_seq(&mut v));
}
```
