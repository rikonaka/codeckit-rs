# codeckit


## Base64

```rust
use codeckit::Base64;

fn main() {
    let test_str = "test";
    let encoded = Base64::encode(test_str.as_bytes());
    println!("{}", encoded);
    let original = Base64::decode(&encoded);
    let original = String::from_utf8(original).unwrap();
    println!("{:?}", original);
}
```