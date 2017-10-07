

// Count the number of arguments
// FIXME (rust-lang/rfcs#88) Remove this macro in favor of the `$#$($arg)` syntax
#[macro_export]
macro_rules! collection_count_args {
    () => { 0 };
    ($x:expr) => { 1 };
    ($head:expr, $($tail:expr),+) => { 1 + collection_count_args!($($tail),+) };
}

#[macro_export]
macro_rules! collection {
    // collection![1, 2, 3]
    ($($x:expr),*) => ({
        let mut temp = $crate::Create::create_with_capacity(collection_count_args!($($x),*));

        $($crate::Create::add_element(&mut temp, $x);)*

        temp
    });
    // collection!{"I" => 1, "II" => 2}
    ($($k:expr => $v:expr),*) => ({
        let mut temp = $crate::Create::create_with_capacity(collection_count_args!($(($k, $v)),*));

        $($crate::Create::add_element(&mut temp, ($k, $v));)*

        temp
    });
    ($($x:expr),+,) => {
        collection!($($x),+)
    };
    ($($k:expr => $v:expr),+,) => {
        collection!($($k => $v),+)
    };
}
