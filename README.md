# pacreason

If you're like me, you don't hesitate to install packages when you're trying something new.
`pacreason` helps you remember why you installed packages, by prompting you to explain it at installation time.

## Usage

- `pacman -S` will automatically prompt you for each new package.
- `pacreason get package...` will display the provided reason for any package.
- `pacreason set package reason...` will set the reason for an already-installed package.

## Disclaimer

The hook functionality of this program explicitly works around intentional features of pacman.
While it works for me, it may break your installation.

## Installation

This program can be installed with `makepkg`.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.