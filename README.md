markdown.rs
===========

A native Rust library for parsing Markdown and (outputting HTML) written to be used by my WIP static site generator [Lava](https://github.com/johannhof/lava).

Usage
----------

To include markdown in your project add the following to your Cargo.toml:

```
[dependencies.markdown]

git = "https://github.com/johannhof/markdown.rs.git"

```

Now you can use the crate in your code with
```
extern crate markdown;
```
