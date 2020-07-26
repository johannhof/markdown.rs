markly.rs [![Crate][crate_img]][crate]
===========

A simple native Rust library for parsing Markdown and (outputting HTML).

Forked from the [markdown crate][markdown_crate].

[crate]: https://crates.io/crates/markly "Crate Page Link"
[crate_img]: https://img.shields.io/crates/v/markly.svg?logo=rust "Crate Page Badge"
[docs]: https://docs.rs/markly "Documentation Link"
[docs_img]: https://docs.rs/markly/badge.svg "Documentation Badge"
[markdown_crate]: https://crates.io/crates/markdown "Markdown Crate Page Link"

Usage
----------

To include markly in your project add the following to your Cargo.toml:

```toml
[dependencies]
markly = "0.3"

```

Now you can use the crate in your code with
```rust
use markly;
```

There is no full documentation right now, the only function exported by the library is `to_html`, which takes a markdown `&str` and converts it to an owned `String` containing html.

```rust
let html : String = markly::to_html("__I am markdown__");

assert_eq!(&html, "<strong>I am markdown</strong>")
```

TODO
----------

- [ ] Reference Style Images
- [ ] Inline HTML
- [ ] Backslash Escapes
- [ ] Automatic Links
- [ ] List wrapping
- [ ] HTML Entities
- [ ] Obscure Emails

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
