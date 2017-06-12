extern crate collection_traits;


use collection_traits::*;



fn push<T, V>(s: &mut T, v: V)
    where T: StackMut<V>,
{
    s.push(v);
}

#[test]
fn test() {
    let mut v = Vec::new();
    push(&mut v, 0);
}
