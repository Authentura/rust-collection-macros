#[macro_export]
macro_rules! hset {
    {} => {
        {
            std::collections::HashSet::new()
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
