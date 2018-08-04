// Count the number of arguments
// FIXME (rust-lang/rfcs#88) Remove this macro in favor of the `$#$($arg)` syntax
#[macro_export]
macro_rules! collection_count_args {
    () => { 0 };
    ($x:expr) => { 1 };
    ($head:expr, $($tail:expr),+) => { 1 + collection_count_args!($($tail),+) };
}

/// Anything that implements the `Create` trait can be used with this
/// # Examples
/// ```
/// #[macro_use]
/// extern crate data_structure_traits;
/// #[cfg(not(feature = "use_std"))]
/// extern crate hashmap_core;
///
/// #[cfg(feature = "use_std")]
/// use std::collections::{HashMap, HashSet};
/// #[cfg(not(feature = "use_std"))]
/// use hashmap_core::{FnvHashMap as HashMap, FnvHashSet as HashSet};
///
/// fn main() {
///     // HashMap
///     let map: HashMap<&str, usize> = collection!{
///         "a" => 1,
///         "b" => 2,
///         "c" => 3,
///     };
///     // Vec
///     let vec: Vec<usize> = collection![0, 1, 2, 3];
/// }
/// ```
#[macro_export]
macro_rules! collection {
    // collection![1, 2, 3]
    ($($x:expr),*) => ({
        let mut temp = $crate::Create::create_with_capacity(collection_count_args!($($x),*));

        $(temp = $crate::Create::add_element(temp, $x);)*

        temp
    });
    // collection!{"I" => 1, "II" => 2}
    ($($k:expr => $v:expr),*) => ({
        let mut temp = $crate::Create::create_with_capacity(collection_count_args!($(($k, $v)),*));

        $(temp = $crate::Create::add_element(temp, ($k, $v));)*

        temp
    });
    ($($x:expr),+,) => {
        collection!($($x),+)
    };
    ($($k:expr => $v:expr),+,) => {
        collection!($($k => $v),+)
    };
}
