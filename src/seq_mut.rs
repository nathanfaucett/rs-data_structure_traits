use core::ops::IndexMut;

use super::insert::Insert;
use super::iterable_mut::IterableMut;
use super::remove::Remove;
use super::seq::Seq;


pub trait SeqMut<'a, Value: 'a>:
    Seq<'a, Value> +
    Insert<usize, Value> +
    Remove<usize, Output = Value> +
    IndexMut<usize, Output = Value> +
    IterableMut<'a, &'a mut Value> {}
