## BytePrefix
[![Rust](https://github.com/charleschege/BytePrefix/actions/workflows/rust.yml/badge.svg)](https://github.com/charleschege/BytePrefix/actions/workflows/rust.yml)

A library to format bytes as specified by The NIST Reference on Constants, Units, and Uncertainty referenced at [Prefixes for Binary Multiples](https://web.archive.org/web/20070808000831/http://physics.nist.gov/cuu/Units/binary.html)

#### Adding the crate to your crate
```sh
$ cargo add byte_prefix
```

#### Usage
```rust
    // Import the crate
    use byte_prefix::calc_bytes;

    // The bytes to convert
    let size = 1024usize as f32;

    // Call `calc_bytes()` function passing the `size` as arguments
    let formatted_file_size = calc_bytes(size);

```

#### LICENSE
This crate is licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT) and all contributions will bear the same licenses.

#### Code of Conduct
All contributions and discussions must adhere to the [Rust Code Of Conduct](https://www.rust-lang.org/policies/code-of-conduct)