This is just a small repository that contains simple macros for creating maps
and sets of differing types and backends.

There are three different kinds of backends for `HashMap`s:

- The `std` backend - the standard `std::collections::HashMap`
- The `dashmap` backend - `dashmap::DashMap`
- The `thincollections` backend - `thincollections::thin_map::ThinMap`

The `std` backend is chosen by default when neither the `dashmap` feature or
the `thincollections` feature are enabled. Additionally, the `dashmap` and
`thincollections` features are mutually exclusive and a compiler error will
occur if both features are enabled.

This crate defines the following macros:

- `hmap` - A `HashMap` macro using one of the specified backends.
- `bmap` - A `BTreeMap` macro using the `std`'s implementation.
- `map` - A generic map macro that will construct either a `HashMap` (default)
  or `BTreeMap` depending on feature selection.
- `hset` - A `HashSet` macro using one of the specified backends.
- `bset` - A `BTreeSet` macro using the `std`'s implementation.
- `set` - A generic set macro that will construct either a `HashSet` (default)
  or `BTreeSet` depending on feature selection.

Default `map` selection can be defined with the `map-macro-use-hmap` or
`map-macro-use-bmap`, both are mutually exclusive with eachother.

Default `set` selection can be defined with the `set-macro-use-hset` or
`set-macro-use-bset`, both are mutually exclusive with eachother.
