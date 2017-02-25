collection_traits [![Build Status](https://travis-ci.org/nathanfaucett/rs-collection_traits.svg?branch=master)](https://travis-ci.org/nathanfaucett/rs-collection_traits)
=====

collection traits

## trait Collection

```rust
pub trait Collection {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
```

## trait Deque

```rust
pub trait Deque<T>: Collection {

    fn push_back(&mut self, element: T);
    fn push_front(&mut self, element: T);
    fn pop_back(&mut self) -> Option<T>;
    fn pop_front(&mut self) -> Option<T>;

    fn front(&self) -> Option<&T>;
    fn back(&self) -> Option<&T>;

    fn front_mut(&mut self) -> Option<&mut T>;
    fn back_mut(&mut self) -> Option<&mut T>;
}
```

## trait Queue

```rust
pub trait Queue<T>: Collection {
    fn enqueue(&mut self, element: T);
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}
```

## trait Stack

```rust
pub trait Stack<T>: Collection {
    fn push(&mut self, element: T);
    fn pop(&mut self) -> Option<T>;
    fn top(&self) -> Option<&T>;
}
```

## trait Insert

```rust
pub trait Insert<Key, Value>: Collection {
    type Output;

    fn insert(&mut self, key: Key, element: Value) -> Self::Output;
}
```

## trait Remove

```rust
pub trait Remove<Key: ?Sized> {
    type Output: ?Sized;

    fn remove(&mut self, key: Key) -> Self::Output;
}
```

## trait Iterable

```rust
pub trait Iterable<'a> {
    type Output;

    fn iter(&'a self) -> Self::Output;
}
```

## trait IterableMut

```rust
pub trait IterableMut<'a> {
    type Output;

    fn iter_mut(&'a mut self) -> Self::Output;
}

```

## trait Map

```rust
pub trait Map<'a, Key: 'a, Value>:
    Collection +
    Index<&'a Key>
{
    fn contains_key(&self, key: &'a Key) -> bool;
}
```

## trait MapMut

```rust
pub trait MapMut<'a, Key: 'a, Value>:
    Map<'a, Key, Value> +
    Insert<Key, Value, Output = Option<Value>> +
    Remove<&'a Key, Output = Value> +
    IndexMut<&'a Key, Output = Value> {}
```

## trait Seq

```rust
pub trait Seq<Value>:
    Collection +
    Index<usize, Output = Value> {}
```

## trait SeqMut

```rust
pub trait SeqMut<Value>:
    Seq<Value> +
    Insert<usize, Value> +
    Remove<usize, Output = Value> +
    IndexMut<usize, Output = Value> {}
```
