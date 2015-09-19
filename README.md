markdown.rs [![](https://travis-ci.org/johannhof/markdown.rs.svg?branch=master)](https://travis-ci.org/johannhof/markdown.rs)
===========

A native Rust library for parsing Markdown and (outputting HTML) written to be used by my WIP static site generator [Lava](https://github.com/johannhof/lava).

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
```

Roadmap
----------

Implement all John Gruber Markdown features and have them tested

- [ ] Inline HTML
- [ ] Automatic Escaping for Special Characters
- [ ] Backslash Escapes
- [ ] Automatic Links
- [ ] Block Elements
  - [x] Paragraphs
    - [x] Parsing
    - [x] HTML
    - [x] Tests
  - [x] Line Breaks
    - [x] Parsing
    - [x] HTML
    - [x] Tests
  - [x] Headers
    - [x] Parsing
    - [x] HTML
    - [x] Tests
  - [x] Blockquotes
    - [x] Parsing
    - [x] HTML
    - [x] Tests
  - [ ] Lists
    - [x] Parsing
    - [x] HTML
    - [ ] Tests
  - [x] Code Blocks
    - [x] Parsing
    - [x] HTML
    - [x] Tests
  - [x] Horizontal Rules
    - [x] Parsing
    - [x] HTML
    - [x] Tests
- [x] Span Elements
  - [x] Links
    - [x] Parsing
    - [x] HTML
    - [x] Tests
  - [x] Emphasis
    - [x] Parsing
    - [x] HTML
    - [x] Tests
  - [x] Strong
    - [x] Parsing
    - [x] HTML
    - [x] Tests
  - [x] Code
    - [x] Parsing
    - [x] HTML
    - [x] Tests
  - [x] Images
    - [x] Parsing
    - [x] HTML
    - [x] Tests

