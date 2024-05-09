# Image to ASCII Converter (Rust)

This is a command-line tool written in Rust that converts images into ASCII art. It takes an image file as input and generates ASCII art output on the console.

## Features

- Converts images into ASCII art.
- Supports various image formats, such as JPEG, PNG, GIF, etc.
- Customizable output: Users can adjust the resolution and ASCII character set for generating ASCII art.
- Lightweight and fast processing.

## Installation

1. Make sure you have Rust installed. If not, you can install it from [the official Rust website](https://www.rust-lang.org/).
2. Clone this repository:

   ```
   git clone https://github.com/variablevar/img2ascii.git
   ```

3. Navigate to the project directory:

   ```
   cd img2ascii
   ```

4. Build the project:

   ```
   cargo build --release
   ```

5. The binary will be located in the `target/release/` directory.

## Usage

```sh
img2ascii [OPTIONS] <image_path> <scale_factor>
```
