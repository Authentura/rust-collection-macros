#[macro_export]
macro_rules! hmap {
    {} => {{
        #[cfg(not(dashmap))]
        use std::collections::HashMap;
        #[cfg(dashmap)]
        use dashmap::DashMap as HashMap;

        HashMap::new()
    }};
    { $( $key:tt => $value:tt ),* } => {{
        let mut hm = hmap!{};

        $(
            hm.insert($key, $value);
        )*

        hm
    }};
    { $( $key:tt => $values:expr ),* } => {{
        let mut hm = hmap!{};

        $(
            hm.insert($key, $values.collect::<Vec<_>>());
        )*

        hm
    }};
    { $( $range:expr => $value:tt ),* } => {{
        let mut hm = hmap!{};

        $(
            ($range).for_each(|key| {
                hm.insert(key, $value);
            });
        )*

        hm
    }};
}
