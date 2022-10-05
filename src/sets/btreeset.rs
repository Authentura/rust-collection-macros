#[macro_export]
macro_rules! bset {
    {} => {
        {
            std::collections::BTreeMap::new()
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
