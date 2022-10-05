pub mod btreeset;
pub mod hashset;

pub use btreeset::*;
pub use hashset::*;

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
