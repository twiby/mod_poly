# mod_poly: a modular polynomial arithmetic library

I this directory is the part of the project written in Rust. It only uses satble Rust features, so you can install the daily Rust toolchain.

You can compile and test the library, and compile the documentation, with the usual:
```
cargo test
cargo doc
```

# Usage as a Python module
To compile the library for the whole project, It is advised to use the Python package `maturin`, which should be already installed at this 
point (via `requirements.txt` in the parent folder). 
To compile and install as a python module, run 
```
maturin develop -r
```

If your compiler supports AVX instructions or other modern feature, you can suggest to the rustc compiler to take advantage of them via the following 
command. However a few experimentations suggest there is not much to be gained.
```
RUSTFLAGS="-C target-cpu=native -C llvm-args=-ffast-math -C opt-level=3" maturin develop -r
```

Every Python-related types and bindings is in the `py_bindings` module, so as to be separate from the rest of the project.
