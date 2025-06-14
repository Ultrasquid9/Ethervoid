# Ethervoid
Ethervoid is a top-down character action metroidvania, inspired by Hollow Knight and Ultrakill. As you can probably tell, it is not done. 

## Features
- Fully data-driven design, with all content declared via [.ron](https://crates.io/crates/ron) files. 
- Scriptable enemy behavior, allowing easy modding.
- More to come in the future!

## Contributing
Please see the [style guide (WIP)](./style_guide.md) for information on how to follow the project's coding standards. Fair warning: as the primary developer of this project is new to coding and very inexperienced, expect messy and unreadable code. 

Contributions must be under the Artistic 2.0 License, or another license that allows relicensing to Artistic 2.0. This is to allow the game to be released on platforms where releasing the source code is not possible (This will be useful if a console port ever becomes a possibility). Contributions with no specified license will be assumed to be under the Artistic 2.0 License. 

## Building
### Windows
Note: Windows is currently untested. 
- Download Rust.
- Download the project source code through your git client of choice.
- Enter the directory containing the source files with your terminal, and run `cargo build --release`.

### Linux 
- Download Rust.
- Download the project source code through your git client of choice.
- Download `mold` and `clang`.
	- These are tools to improve compilation times
	- If you do not want to install them, delete `.cargo/config.toml` in the directory containing the source code.
- Enter the directory containing the source files with your terminal, and run `cargo build --release`.

### MacOS
- While MacOS is not supported, it should theoretically work with the steps provided for the Windows version. 
- Contribuitions providing fixes for MacOS may be accepted.
