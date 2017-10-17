data_structure_traits
=====

data structure collection traits

```rust
#[macro_use] extern crate data_structure_traits;


use data_structure_traits::*;



fn use_seq<'a, S>(s: &'a mut S) -> usize
    where S: SeqMut<'a, usize>,
          &'a S: IntoIterator<Item = &'a usize>,
          &'a mut S: IntoIterator<Item = &'a mut usize>,
{
    if s.len() >= 5 {
        s.get_mut(4).map(|v| { *v = 0; });        
    } else {
        s.insert(0, 1);
    }

    let mut avg = 0;
    let len = s.len();

    for v in s.into_iter() {
        avg += *v;
    }

    avg / len
}


fn main() {
    let mut v: Vec<usize> = collection![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}", use_seq(&mut v));
}
```
