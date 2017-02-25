use core::ops::IndexMut;

use super::seq::Seq;
use super::insert::Insert;
use super::remove::Remove;


pub trait SeqMut<Value>:
    Seq<Value> +
    Insert<usize, Value> +
    Remove<usize, Output = Value> +
    IndexMut<usize, Output = Value> {}
