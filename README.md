# codeckit

|     Function     |                Description                 |
| :--------------: | :----------------------------------------: |
| `Base32::encode` | Encodes a byte slice into a Base32 string  |
| `Base32::decode` | Decodes a Base32 string into a byte vector |
| `Base58::encode` | Encodes a byte slice into a Base58 string  |
| `Base58::decode` | Decodes a Base58 string into a byte vector |
| `Base64::encode` | Encodes a byte slice into a Base64 string  |
| `Base64::decode` | Decodes a Base64 string into a byte vector |

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
