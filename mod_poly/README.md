# mod_poly: a modular polynomial arithmetic library

I this directory is the part of the project written in Rust. It only uses satble Rust features, so you can install the daily Rust toolchain.

You can compile and test the library, and compile the documentation, with the usual:
```
cargo test
cargo doc
```

# Usage as a Python module
To compile the library for the whole project, It is advised to use the Python package `maturin`, which should be already installed at this 
point (via `requirements.txt` in the parent folder). Also the code is partly written to take advantage of compiler vectorization,
so to obtain the best performance on your machine, you can compile and install as a Python package using
```
RUSTFLAGS="-C target-cpu=native -C llvm-args=-ffast-math -C opt-level=3" maturin develop -r
```

Every Python-related types and bindings is in the `py_bindings` module, so as to be separate from the rest of the project.
