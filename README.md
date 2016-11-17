# Lishp

![Build Status](https://travis-ci.org/Michael-F-Bryan/lishp.svg?branch=master)
![MIT License](https://img.shields.io/github/license/Michael-F-Bryan/lishp.svg)

Yet another lisp interpreter. This time, written in `Rust`.

Documentation for the `interpreter` is automatically generated by travis-ci and
is hosted using [here][docs].

## Build & Run

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

You can then run the interpreter (located at `src/bin/main.rs`) using cargo.

```
$ cargo run
```


## Features

* [x] Parse some source code and generate an Abstract Syntax Tree.
* [ ] Evaluate some basic code (hello world example).
* [ ] Define functions.
* [ ] Add a global environment.
* [ ] Support lexical scoping.
* [ ] Add class support (everything is an object).
* [ ] Garbage collection.
* [ ] Use duck typing.
* [ ] Proper lisp macros.
* [ ] Libraries and other code files are loadable at runtime.
* [ ] Create a basic standard library.
* [ ] REPL.
* [ ] Compile code to LLVM or binary.


[docs]: https://michael-f-bryan.github.io/lishp/
