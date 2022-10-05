#![cfg(test)]

use crate::*;
#[cfg(dashmap)]
use dashmap::DashMap as HashMap;
use std::collections::BTreeMap;
#[cfg(not(dashmap))]
use std::collections::HashMap;

#[test]
fn hashmap_none() {
    let hashmap: HashMap<i32, i32> = hmap! {};

    assert_eq!(hashmap, HashMap::new());
}

#[test]
fn hashmap_few() {
    let hashmap = hmap! {
        "hello" => 1,
        "world" => 2
    };

    assert_eq!(hashmap, HashMap::from([("hello", 1), ("world", 2)]));
}

#[test]
fn btreemap_none() {
    let btreemap: BTreeMap<i32, i32> = bmap! {};

    assert_eq!(btreemap, BTreeMap::new());
}

#[test]
fn btreemap_few() {
    let btreemap = bmap! {
        "hello" => 1,
        "world" => 2
    };

    assert_eq!(btreemap, BTreeMap::from([("hello", 1), ("world", 2)]));
}
