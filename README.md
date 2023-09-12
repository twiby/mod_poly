# mod_poly: a modular polynomial arithmetic library
Whether this will be ditributed as a Rust crate or a Python library, the bulk of the code will be in Rust, 
so be sure you have rust installed (check https://www.rust-lang.org/learn/get-started).

# Usage as a Rust crate
Every type is public and documented. You can run all the usuals:
```shell
cargo build
cargo test
cargo doc
``` 

# Usage as a Python module
You can install dependencies and compile the Python wheel using:
```
pip install .
```
And it is advised to use a virtual environment for this.

# Usage as Python app
The file `mod_poly.py` can also serve as an app, partly to benchmark the underlying library performance, partly to launch an interface to manipulate complex numbers. This can also serve as examples on how to use the generated Python module.
Use `python3 mod_poly.py --help` to get help about options.
