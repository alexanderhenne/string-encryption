# string-encryption

Using string encryption libraries, your string literals can be hidden from tools that extract strings from binaries. 

This implementation encrypts strings by making use of the one-time pad technique.

The library consists of two parts:

* **string-encryption-plugin**: A compiler plugin library that gives access to an encryption macro.
* **string-decryption**: A library that gives access to the decryption function that the encryption macro accesses.

### Usage

[Example usage](src/main.rs)

Your [Cargo.toml](Cargo.toml) must contain references to the library paths for you to be able to import them:

```toml
[dependencies.string_encryption_plugin]
path = "string-encryption-plugin"

[dependencies.string_decryption]
path = "string-decryption"
```

Then, import the libraries:

```rust
#![feature(plugin)]
#![plugin(string_encryption_plugin)]

extern crate string_decryption;
use string_decryption::d;
```

Now you are free to use the "e" macro:

```rust
fn main() {
    let string = e!("Hello, world!");
    println!("{} {}", string, "blah"); // Hello world!
}
```
