#![cfg_attr(not(feature = "std"), feature(alloc))]
#![no_std]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "hashmap_core")]
extern crate hashmap_core;
#[cfg(feature = "std")]
extern crate std;

mod collection;
mod collection_mut;
mod collections;
mod create;
mod deque;
mod deque_mut;
mod get;
mod get_mut;
mod insert;
mod insert_mut;
#[macro_use]
mod macros;
mod map;
mod map_mut;
mod queue;
mod queue_mut;
mod remove;
mod remove_mut;
mod seq;
mod seq_mut;
mod set;
mod set_mut;
mod stack;
mod stack_mut;

pub use self::collection::Collection;
pub use self::collection_mut::CollectionMut;
pub use self::create::Create;
pub use self::deque::Deque;
pub use self::deque_mut::DequeMut;
pub use self::get::Get;
pub use self::get_mut::GetMut;
pub use self::insert::Insert;
pub use self::insert_mut::InsertMut;
pub use self::map::Map;
pub use self::map_mut::MapMut;
pub use self::queue::Queue;
pub use self::queue_mut::QueueMut;
pub use self::remove::Remove;
pub use self::remove_mut::RemoveMut;
pub use self::seq::Seq;
pub use self::seq_mut::SeqMut;
pub use self::set::Set;
pub use self::set_mut::SetMut;
pub use self::stack::Stack;
pub use self::stack_mut::StackMut;
