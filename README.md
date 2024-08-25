# xorfiles_avx2

A Rust project for XOR file operations using AVX2 instructions.

## Description

`xorfiles_avx2` is a utility that performs XOR operations on files using AVX2 instructions for improved performance. This project aims to provide a **super fast** and efficient way to manipulate binary data through XOR operations.

## Features

- Fast XOR operations using AVX2 SIMD instructions
- Easy-to-use command-line interface
- Supports any file format

## Installation

To use `xorfiles_avx2`, you need to have Rust and Cargo installed. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, you can clone this repository and build the project:

```bash
git clone https://github.com/qnfm/xorfiles_avx2.git
cd xorfiles_avx2
cargo build --release
```
## Usage

After building the project, you can run the executable with the following command:

`./target/release/xorfiles_avx2 <input_file1> <input_file2> <output_file>`

Replace <input_file1> <input_file2> with the path to the files you want to process **which has exactly the same size in bytes** and <output_file> with the path where you want to save the result.

## Contributing

Contributions are welcome! If you have suggestions for improvements or new features, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.
