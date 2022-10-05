/// The `bmap` macro will construct a [`BTreeMap`] using the [`std`]'s
/// implementation.
///
/// [`BTreeMap`]: std::collections::BTreeMap
/// [`std`]: std
#[macro_export]
macro_rules! bmap {
    {} => {{
        use std::collections::BTreeMap;

        BTreeMap::new()
    }};
    { $($key:expr => $value:expr),* $(,)? } => {
       {
           let mut bm = bmap!{};

           $(
               bm.insert($key, $value);
           )*

           bm
       }
    }
}
