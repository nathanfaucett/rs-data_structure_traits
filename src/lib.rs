#![feature(alloc)]
#![feature(core_slice_ext)]
#![feature(custom_attribute)]
#![no_std]


#[cfg(feature = "use_std")] extern crate std;

extern crate alloc;


mod collections;
mod collection_mut;
mod collection;
mod create;
mod deque_mut;
mod deque;
mod get_mut;
mod get;
mod insert_mut;
mod insert;
mod iterable_mut;
mod iterable;
#[macro_use] mod macros;
mod map_mut;
mod map;
mod queue_mut;
mod queue;
mod remove_mut;
mod remove;
mod seq_mut;
mod seq;
mod set_mut;
mod set;
mod stack_mut;
mod stack;


pub use self::collection_mut::CollectionMut;
pub use self::collection::Collection;
pub use self::create::Create;
pub use self::deque_mut::DequeMut;
pub use self::deque::Deque;
pub use self::get_mut::GetMut;
pub use self::get::Get;
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
pub use self::set_mut::SetMut;
pub use self::set::Set;
pub use self::stack_mut::StackMut;
pub use self::stack::Stack;
