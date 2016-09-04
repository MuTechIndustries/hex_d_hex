# HexDHex
## Purpose
HexDHex is a Rust Crate encodes and decodes byte data to and from its hexidecimal representation. For instance, one may wish, on ocasion that is, to translate a utf8 or ASCII string of hex byte data such as "0F12FF00A7" to a vector of u8s [15,18,255,0,167] or vise versa.
## Encoding
For the perspective of this crate, encoding is the process of taking a vector of raw byte data, or a vector of u8s, and translating it into an ACSII or UTF-8 compatable string. There are two methods, one that encodes with capital letters and the other that encodes with lower case letters. We seperated these methods as apposed to having a boolean parameter for performance reasons. Why make the ALU click twice when it can click once? Both methods return a box string, we wanted to make it on the heap to avoid unecessary copying. This has performance benifits and only a small dereference on declaration. See examples below.
### Capital Encoding
```rust
let hex_vals: Vec<u8> = vec![15,18,255,0,167];
let hex_string = *capital_hex(&hex_vals);
assert_eq!(hex_string, "0F12FF00A7");
```
### Lower Case Encoding
```rust
let hex_vals: Vec<u8> = vec![15,18,255,0,167];
let hex_string = *lower_hex(&hex_vals);
assert_eq!(hex_string, "0f12ff00a7");
```