Provides a macro allow natural usage of checked math.

[Documentation.](https://docs.rs/cheque/)

```rust
let a = 5u8;
let b = 20u8;
let z = 0u8;
 
checked_wrap![a, b, z];
 
assert_eq!(*(a + b), Some(25));
assert_eq!(*(b * b), None);
assert_eq!(*(a - b), None);
assert_eq!(*(b / z), None);
assert_eq!(*(a - 20), None);
assert_eq!(*((a - b) + 1), None);
```

