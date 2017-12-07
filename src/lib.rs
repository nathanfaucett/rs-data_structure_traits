#![feature(alloc)]
#![feature(core_slice_ext)]
#![no_std]


#[cfg(feature = "use_std")] extern crate std;
#[cfg(not(feature = "use_std"))] extern crate hashmap_core;

extern crate alloc;


mod collections;
mod collection_mut;
mod collection;
mod create;
mod deque_immut;
mod deque;
mod get_mut;
mod get;
mod insert_immut;
mod insert;
#[macro_use] mod macros;
mod map_immut;
mod map_mut;
mod map;
mod queue_immut;
mod queue;
mod remove_immut;
mod remove;
mod seq_immut;
mod seq_mut;
mod seq;
mod set_immut;
mod set_mut;
mod set;
mod stack_immut;
mod stack;


pub use self::collection_mut::CollectionMut;
pub use self::collection::Collection;
pub use self::create::Create;
pub use self::deque_immut::DequeImmut;
pub use self::deque::Deque;
pub use self::get_mut::GetMut;
pub use self::get::Get;
pub use self::insert_immut::InsertImmut;
pub use self::insert::Insert;
pub use self::map_immut::MapImmut;
pub use self::map_mut::MapMut;
pub use self::map::Map;
pub use self::queue_immut::QueueImmut;
pub use self::queue::Queue;
pub use self::remove_immut::RemoveImmut;
pub use self::remove::Remove;
pub use self::seq_immut::SeqImmut;
pub use self::seq_mut::SeqMut;
pub use self::seq::Seq;
pub use self::set_immut::SetImmut;
pub use self::set_mut::SetMut;
pub use self::set::Set;
pub use self::stack_immut::StackImmut;
pub use self::stack::Stack;
