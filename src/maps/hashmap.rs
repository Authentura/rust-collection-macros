/// The `hmap` macro will construct a [`HashMap`] using the [`std`]'s
/// implementation by default.
///
/// # Optional features
///
/// If the `dashmap` feature is enabled, then the [`DashMap`] backend will be
/// used.
///
/// If the `thincollections` feature is enabled, then the [`ThinMap`] backend
/// will be used.
///
/// [`HashMap`]: std::collections::HashMap
/// [`DashMap`]: dashmap::DashMap
/// [`ThinMap`]: thincollections::thin_map::ThinMap
/// [`std`]: std
#[macro_export]
macro_rules! hmap {
    {} => {
        {
            #[cfg(all(not(dashmap), not(thincollections)))]
            use std::collections::HashMap;
            #[cfg(dashmap)]
            use dashmap::DashMap as HashMap;
            #[cfg(thincollections)]
            use thincollections::thin_map::ThinMap as HashMap;

            HashMap::new()
        }
    };
    { $($key:expr => $value:expr),* $(,)? } => {
       {
           let mut hm = hmap!{};

           $(
               hm.insert($key, $value);
           )*

           hm
       }
    }
}
