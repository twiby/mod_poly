# mod_poly: a modular polynomial arithmetic library
Whether this will be ditributed as a Rust crate or a Python library, the bulk of the code will be in Rust, 
so be sure you have rust installed (check https://www.rust-lang.org/learn/get-started).

As usual, you can install dependencies using:
```
pip install -r requirements.txt
```
And it is advised to use a virtual environment for this.

You must also compile and install additional software components written in Rust: check the `README.md` of the `mod_poly` directory.

# Usage as a library
the `mod_poly` subfolder is a Rust project that can be used as a crate as is. Every type is public and domcumented. It can also be used as a Python module (if compiled according to instructions found there), every type being binded to a Python equivalent. With the binding comes a small lack of generality in types. 
