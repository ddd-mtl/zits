/// test/rust.rs

#[serde]
/// Test integer
const CONST_TEST_1: i32 = 0;

#[serde]
/// Shouldn't compile but should convert
const CONST_TEST_2: i32 = 0.0;

#[serde]
/// Valid Rust but not valid typescript would be misleading if it made it into normal string ?
const CONST_TEST_3: &'static [u8] = b"Hello";

#[serde]
/// Test serde_json
const SERDE_JSON_1: serde_json::Value = serde_json::json!({ "a": "b" });

#[serde]
const SERDE_JSON_2: serde_json::Value = json!({ "a": "b" });
