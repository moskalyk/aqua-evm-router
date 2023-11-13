#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

module_manifest!();

pub fn main() {}

#[marine]
pub fn deterministic_index(address: &str, n: i32) -> i32 {
    // Create a hasher
    let mut hasher = DefaultHasher::new();

    // Hash the address
    address.hash(&mut hasher);

    // Get the hash value as a u64
    let hash_value = hasher.finish();

    // Calculate the index by taking the remainder when dividing the hash value by N
    let index = hash_value as usize % n as usize;

    // Cast the index to i32
    index as i32
}