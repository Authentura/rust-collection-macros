#[macro_export]
macro_rules! hmap {
    {} => {
        {
            #[cfg(not(dashmap))]
            use std::collections::HashMap;
            #[cfg(dashmap)]
            use dashmap::DashMap as HashMap;

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
