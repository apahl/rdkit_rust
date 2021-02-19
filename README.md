# Rust Bindings to the RDKit

This project is the beginning of an effort to create bindings to the Open Source C++ cheminformatics toolkit [RDKit](http://rdkit.org/) for the Rust programming language.
The bindings are completely manual and are making heavy use of the [cpp](https://github.com/mystor/rust-cpp) crate.
More than everything, for me this is an excercise in learning Rust, which means, that these bindings are probably of limited use for others.

## Experiences so far

Creating the manual bindings is really quite laborious.
A [similar attempt](https://github.com/apahl/rdkit_nim) using the [Nim](https://nim-lang.org/) language was actually much less painful. 
The major roadblock encountered so far was the inability to pass a C++ string to Rust.
Because of the volatility of the C++ std::string, this was not even possible to do via a C string pointer (`c_str()`).
The workaround was to allocate a temporary char buffer (of fixed size!) and copy the C++ string into it (see [`Mol::to_smiles()`](https://github.com/apahl/rdkit_rust/blob/9de90f2e2a545fbe22f424005e63d350530bbec3/src/romol.rs#L78)).

## Status

The bindings in their current state do work on Linux, but not much functionality has been wrapped, yet. Please have a look at the tests for what is available.
In order to compile this project, you need to have a conda installation of the RDKit and define the environment variable `RDKIT_CONDA` with the path to it, e.g. in `~/.profile`:

    export RDKIT_CONDA=/home/pahl/anaconda3/envs/chem

In addition, you will need the Rust toolchain and a C compiler (e.g. gcc).
After downloading this project, the tests can then be executed in the following way:

    $ LD_LIBRARY_PATH=$RDKIT_CONDA/lib cargo test
