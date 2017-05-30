#![feature(collections)]
#![feature(custom_attribute)]
#![no_std]


#[cfg(not(feature = "no_std"))]
extern crate std;

extern crate collections;


#[cfg(not(feature = "no_std"))]
mod hash_map;

mod collection_mut;
mod collection;
mod deque_mut;
mod deque;
mod insert_mut;
mod insert;
mod iterable_mut;
mod iterable;
mod linked_list;
mod map_mut;
mod map;
mod queue_mut;
mod queue;
mod remove_mut;
mod remove;
mod seq_mut;
mod seq;
mod stack_mut;
mod stack;
mod vec;


pub use self::collection_mut::CollectionMut;
pub use self::collection::Collection;
pub use self::deque_mut::DequeMut;
pub use self::deque::Deque;
pub use self::insert_mut::InsertMut;
pub use self::insert::Insert;
pub use self::iterable_mut::IterableMut;
pub use self::iterable::Iterable;
pub use self::map_mut::MapMut;
pub use self::map::Map;
pub use self::queue_mut::QueueMut;
pub use self::queue::Queue;
pub use self::remove_mut::RemoveMut;
pub use self::remove::Remove;
pub use self::seq_mut::SeqMut;
pub use self::seq::Seq;
pub use self::stack_mut::StackMut;
pub use self::stack::Stack;
