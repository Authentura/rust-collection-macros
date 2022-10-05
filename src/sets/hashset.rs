/// The `hset` macro will construct a [`HashSet`] using the [`std`]'s
/// implementation by default.
///
/// # Optional features
///
/// If the `dashset` feature is enabled, then the [`DashSet`] backend will be
/// used.
///
/// If the `thincollections` feature is enabled, then the [`ThinSet`] backend
/// will be used.
///
/// [`HashSet`]: std::collections::HashSet
/// [`DashSet`]: dashset::DashSet
/// [`ThinSet`]: thincollections::thin_set::ThinSet
/// [`std`]: std
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
