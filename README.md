collection_traits [![Build Status](https://travis-ci.org/nathanfaucett/rs-collection_traits.svg?branch=master)](https://travis-ci.org/nathanfaucett/rs-collection_traits)
=====

collection traits

## trait Collection

```rust
pub trait Collection {
    fn len(&self) -> usize;
    fn clear(&mut self);
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
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
    fn peek_mut(&mut self) -> Option<&mut T>;
}
```

## trait Stack

```rust
pub trait Stack<T>: Collection {
    fn push(&mut self, element: T);
    fn pop(&mut self) -> Option<T>;

    fn top(&self) -> Option<&T>;
    fn top_mut(&mut self) -> Option<&mut T>;
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
pub trait Iterable<'a, T> {
    type Iter: Iterator<Item = T>;

    fn iter(&'a self) -> Self::Iter;
}
```

## trait IterableMut

```rust
pub trait IterableMut<'a, T> {
    type IterMut: Iterator<Item = T>;

    fn iter_mut(&'a mut self) -> Self::IterMut;
}
```

## trait Map

```rust
pub trait Map<'a, Key, BorrowKey, Value>:
    Collection +
    Index<&'a Key, Output = Value> +
    Iterable<'a, (&'a Key, &'a Value)>

    where Key: 'a + Borrow<BorrowKey>,
          BorrowKey: 'a + ?Sized,
          Value: 'a,
{
    fn contains_key(&self, k: &BorrowKey) -> bool;
}
```

## trait MapMut

```rust
pub trait MapMut<'a, Key, BorrowKey, Value>:
    Map<'a, Key, BorrowKey, Value> +
    Insert<Key, Value, Output = Option<Value>> +
    Remove<&'a Key, Output = Option<Value>> +
    IndexMut<&'a Key, Output = Value> +
    IterableMut<'a, (&'a Key, &'a mut Value)>

    where Key: 'a + Borrow<BorrowKey>,
          BorrowKey: 'a + ?Sized,
          Value: 'a
    {}
```

## trait Seq

```rust
pub trait Seq<'a, Value: 'a>:
    Collection +
    Index<usize, Output = Value> +
    Iterable<'a, &'a Value> {}
```

## trait SeqMut

```rust
pub trait SeqMut<'a, Value: 'a>:
    Seq<'a, Value> +
    Insert<usize, Value> +
    Remove<usize, Output = Value> +
    IndexMut<usize, Output = Value> +
    IterableMut<'a, &'a mut Value> {}
```
