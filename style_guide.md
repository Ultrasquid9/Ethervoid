# Style Guide (WIP)

## Textures
### Colors
The Ethervoid Pallete can be found in `/assets/evoid-pallete.ase`. This pallete is a combination of a few palletes, most notably "Ressurect 64" and "Pear36", and is specifically designed for use in the game. 

### Resolution
Sprites should have a minimum resolution of 16:16. All resolutions should be multiples of 16 (E.G. 32:32 is allowed, 64:256 is allowed, 18:20 is not allowed).

## Code
Generally, follow what `cargo fmt`, `cargo clippy`, and `stylua` do (you can run them all at once with `just lint`). In the rare case that the code is more readable without them, use `#[rustfmt::skip]` or `#[allow(/* clippy lint here*/)]`. 
