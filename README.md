markdown.rs [![Build Status](https://travis-ci.org/johannhof/markdown.rs.svg?branch=master)](https://travis-ci.org/johannhof/markdown.rs)
===========

A native Rust library for parsing Markdown and (outputting HTML) written to be used by my WIP static site generator [Lava](https://github.com/johannhof/lava).

Usage
----------

To include markdown in your project add the following to your Cargo.toml:

```toml
[dependencies.markdown]

git = "https://github.com/johannhof/markdown.rs.git"

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

Short-Term: Implement all John Gruber Markdown features and have them tested

- [ ] Inline HTML
- [ ] Automatic Escaping for Special Characters
- [ ] Backslash Escapes
- [ ] Automatic Links
- [ ] Block Elements
  - [ ] Paragraphs
    - [ ] Parsing
    - [x] HTML
    - [ ] Tests
  - [ ] Line Breaks
    - [x] Parsing
    - [x] HTML
    - [ ] Tests
  - [x] Headers
    - [x] Parsing
    - [x] HTML
    - [x] Tests
  - [ ] Blockquotes
    - [x] Parsing
    - [x] HTML
    - [ ] Tests
  - [ ] Lists
    - [ ] Parsing
    - [ ] HTML
    - [ ] Tests
  - [ ] Code Blocks
    - [x] Parsing
    - [x] HTML
    - [ ] Tests
  - [x] Horizontal Rules
    - [x] Parsing
    - [x] HTML
    - [x] Tests
- [ ] Span Elements
  - [ ] Links
    - [x] Parsing
    - [x] HTML
    - [ ] Tests
  - [x] Emphasis
    - [x] Parsing
    - [x] HTML
    - [x] Tests
  - [x] Strong
    - [x] Parsing
    - [x] HTML
    - [x] Tests
  - [ ] Code
    - [x] Parsing
    - [x] HTML
    - [ ] Tests
  - [ ] Images
    - [x] Parsing
    - [x] HTML
    - [ ] Tests

Long-Term: Modularize all Features and move away from regex

