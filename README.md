# xorstring-rs
Simple PoC of using the XOR Cipher in Rust

# Info
- You might want to evaluate the string at compile time if you use this project in production.
- Code heavily relies on the usage of higher-order functions (HOFs)

# If you want to use a version which doesn't rely on HOF(s) heres a version without them:

```rust
fn decrypt(&self) -> String {
    let mut decryped_string: String = String::new();
    for c in self.text.chars() {
        let xored = c as u8 ^ KEY;
        decryped_string.push(xored as char);
    } 
    decryped_string
}
```
