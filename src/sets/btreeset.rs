/// The `bset` macro will construct a [`BTreeSet`] using the [`std`]'s
/// implementation.
///
/// [`BTreeSet`]: std::collections::BTreeSet
/// [`std`]: std
#[macro_export]
macro_rules! bset {
    {} => {
        {
            std::collections::BTreeSet::new()
        }
    };
    { $($value:expr),* $(,)? } => {
        {
            let mut set = bset!{};

            $(
                set.insert($value);
            )*

            set
        }
    };
}
