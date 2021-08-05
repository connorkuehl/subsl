# subsl

subsl extends the slices you know and love with some additional
functionality.

For example, usually, you can't split a byte slice on a subslice;
with subsl, you can!

## Examples

```rust
use subsl::SubslSplitter;

let http_get: &[u8] = &*b"GET / HTTP/1.0\r\n\r\nsome data in the body";
let sep = b"\r\n\r\n";
let mut iter = http_get.subsl_split(sep);
let headers = iter.next().unwrap();
let body = iter.next().unwrap();

assert_eq!(headers, b"GET / HTTP/1.0");
assert_eq!(body, b"some data in the body");
```

License: Apache-2.0
