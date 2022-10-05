#[macro_export]
macro_rules! hset {
    {} => {
        {
            #[cfg(all(not(dashmap), not(thincollections)))]
            use std::collections::HashSet;
            #[cfg(dashmap)]
            use dashmap::DashSet as HashSet;
            #[cfg(thincollections)]
            use thincollections::thin_map::ThinSet as HashSet;

            HashSet::new()
        }
    };
    { $($value:expr),* $(,)? } => {
        {
            let mut set = hset!{};

            $(
                set.insert($value);
            )*

            set
        }
    };
}
