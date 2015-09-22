markdown.rs [![](https://travis-ci.org/johannhof/markdown.rs.svg?branch=master)](https://travis-ci.org/johannhof/markdown.rs)
===========

A simple native Rust library for parsing Markdown and (outputting HTML).

Usage
----------

To include markdown in your project add the following to your Cargo.toml:

```toml
[dependencies]
markdown = "0.1"

```

Now you can use the crate in your code with
```rust
extern crate markdown;
```

There is no full documentation right now, the only function exported by the library is `to_html`, which takes a markdown `&str` and converts it to an owned `String` containing html.

```rust
let html : String = markdown::to_html("__I am markdown__");

assert_eq!(&html, "<strong>I am markdown</strong>")
```

TODO
----------

- [ ] Inline HTML
- [ ] Backslash Escapes
- [ ] Automatic Links
- [ ] Reference-Style Links
