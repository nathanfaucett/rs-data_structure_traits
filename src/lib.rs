#![feature(alloc)]
#![no_std]

#[cfg(not(feature = "use_std"))]
extern crate hashmap_core;
#[cfg(feature = "use_std")]
extern crate std;

extern crate alloc;

mod collection;
mod collection_mut;
mod collections;
mod create;
mod deque;
mod deque_immut;
mod get;
mod get_mut;
mod insert;
mod insert_immut;
#[macro_use]
mod macros;
mod map;
mod map_immut;
mod map_mut;
mod queue;
mod queue_immut;
mod remove;
mod remove_immut;
mod seq;
mod seq_immut;
mod seq_mut;
mod set;
mod set_immut;
mod set_mut;
mod stack;
mod stack_immut;

pub use self::collection::Collection;
pub use self::collection_mut::CollectionMut;
pub use self::create::Create;
pub use self::deque::Deque;
pub use self::deque_immut::DequeImmut;
pub use self::get::Get;
pub use self::get_mut::GetMut;
pub use self::insert::Insert;
pub use self::insert_immut::InsertImmut;
pub use self::map::Map;
pub use self::map_immut::MapImmut;
pub use self::map_mut::MapMut;
pub use self::queue::Queue;
pub use self::queue_immut::QueueImmut;
pub use self::remove::Remove;
pub use self::remove_immut::RemoveImmut;
pub use self::seq::Seq;
pub use self::seq_immut::SeqImmut;
pub use self::seq_mut::SeqMut;
pub use self::set::Set;
pub use self::set_immut::SetImmut;
pub use self::set_mut::SetMut;
pub use self::stack::Stack;
pub use self::stack_immut::StackImmut;
