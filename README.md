`bevy-2d-template`
==================
[`cargo-generate`] template for jumpstarting small 2D [`bevy projects`]

## Usage
If you don't have them already, install [`cargo-generate`]:

```bash
cargo install cargo-generate
```

Then instanciate this template:
```bash
cargo generate --git https://github.com/weidingerhp/bevy-2d-template
```

### Local binary

Just go ahead and start your newly created Game with 

```bash
cargo run
```

and you would have a simple window with your title!

[`cargo-generate`]: https://github.com/cargo-generate/cargo-generate
[`bevy projects`]: https://bevyengine.org/

### WASM

Setup and build:

```bash
rustup target add wasm32-unknown-unknown

cargo install wasm-pack
```

if installing wasm-pack did not work there is also a downloader for a pre-built binary (windows) at https://rustwasm.github.io/wasm-pack/installer/ .

Building can be done with 
```bash
wasm-pack build --target web --release
```

this builds needed files into a folder named `pkg`.

After that - please copy the `index.html` from the `web` folder and the whole `assets`-folder into `pkg` and deploy the whole thing on a webserver.

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
