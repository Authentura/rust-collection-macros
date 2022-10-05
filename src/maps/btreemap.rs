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
