pub mod btreeset;
pub mod hashset;

pub use btreeset::*;
pub use hashset::*;

/// A generic `set` macro that wraps either [`bset`] or [`hset`] for convenience
/// depending on feature flags.
///
/// **Default**: Wraps the `hset` macro (`set-macro-use-hset` used by default).
///
/// # Optional features
///
/// Enabling the `set-macro-use-bset` feature and disabling the
/// `set-macro-use-hset` feature will force the wrapping of the `bset` macro
/// instead.
#[macro_export]
macro_rules! set {
    {} => {
        {
            #[cfg(set-macro-use-hset)]
            hset!{}
            #[cfg(set-macro-use-bset)]
            bset!{}
        }
    };
    { $($value:expr),* $(,)? } => {
        {
            let mut set = set!{};

            $(
                set.insert($value);
            )*

            set
        }
    }
}
