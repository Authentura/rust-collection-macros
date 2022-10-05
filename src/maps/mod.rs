pub mod btreemap;
pub mod hashmap;

pub use btreemap::*;
pub use hashmap::*;

#[macro_export]
macro_rules! map {
    {} => {
        {
            #[cfg(map-macro-use-hmap)]
            hmap!{}
            #[cfg(map-macro-use-bmap)]
            hmap!{}
        }
    };
    { $($key:expr => $value:expr),* $(,)? } => {
       {
           let mut m = map!{};

           $(
               m.insert($key, $value);
           )*

           m
       }
    }

}
