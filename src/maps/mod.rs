pub mod btreemap;
pub mod hashmap;

pub use btreemap::*;
pub use hashmap::*;

#[macro_export]
macro_rules! map {
    {} => {{
        hmap!{}
    }};
    { $( $key:tt => $value:tt ),* } => {{
        hmap!{}
    }};
    { $( $key:tt => $values:expr ),* } => {{
        hmap!{}
    }};
    { $( $range:expr => $value:tt ),* } => {{
        hmap!{}
    }};
}
