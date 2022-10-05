#[cfg(all(feature = "map-macro-use-hmap", feature = "map-macro-use-bmap"))]
compile_error!(
    "Feature \"map-macro-use-hmap\" and \"map-macro-use-bmap\" are not compatible with \
     eachother."
);

#[cfg(all(feature = "set-macro-use-hset", feature = "set-macro-use-bset"))]
compile_error!(
    "Feature \"set-macro-use-hset\" and \"set-macro-use-bset\" are not compatible with \
     eachother."
);

#[cfg(all(feature = "dashmap", feature = "thincollections"))]
compile_error!(
    "Feature \"dashmap\" and \"thincollections\" are not compatible with eachother."
);

#[macro_use]
pub mod maps;
#[macro_use]
pub mod sets;
