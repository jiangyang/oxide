#[macro_export]
macro_rules! vals [
    ($($x:expr),*) => (vec!($($x.into()),*));
    ($($x:expr),+,) => (vals!($($x),+));
];

#[macro_export]
macro_rules! matches [
    ($($x:expr),*) => (vec!($($x.into()),*));
    ($($x:expr),+,) => (matches!($($x),+));
];