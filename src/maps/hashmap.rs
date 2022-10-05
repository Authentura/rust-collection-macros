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
