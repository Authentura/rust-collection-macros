pub mod btreemap;
pub mod hashmap;

pub use btreemap::*;
pub use hashmap::*;

/// A generic `map` macro that wraps either [`bmap`] or [`hmap`] for convenience
/// depending on feature flags.
///
/// **Default**: Wraps the `hmap` macro (`map-macro-use-hmap` used by default).
///
/// # Optional features
///
/// Enabling the `map-macro-use-bmap` feature and disabling the
/// `map-macro-use-hmap` feature will force the wrapping of the `bmap` macro
/// instead.
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
