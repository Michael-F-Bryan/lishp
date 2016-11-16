# Lishp

![Build Status](https://travis-ci.org/Michael-F-Bryan/lishp.svg?branch=master)
![MIT License](https://img.shields.io/github/license/Michael-F-Bryan/lishp.svg)

Yet another lisp interpreter. This time, written in `Rust`.


## Build & install

You'll probably need to use a nightly version of the Rust compiler to build
this. For more information, visit https://rustup.rs/.

Next, get the source code:

```
$ git clone https://github.com/Michael-F-Bryan/lishp
```

And build,

```
$ cargo build
```

The initial build may take a while because `lalrpop` automatically generates a
~40,000 line parser.
