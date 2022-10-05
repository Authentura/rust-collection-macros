#![cfg(test)]

use crate::*;
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
