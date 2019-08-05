# rustvis

#### Data Visualisation with Rust

rustvis is a high-performance Rust data visualisation library, allowing for the creation of responsive, flexible charts. 

rustvis allows you to create:
- Bar charts
- Histograms 
- Gradient-based barcharts/histograms
- Linecharts 
- Scatterplots

#### Output
Charts are outputted as PNG/JPEG images, but other options such as SVG and HTML/CSS will be available soon. 

### View Example Graphics
![Sample graphics generated with rustvis](https://i.imgur.com/vfZHyU4.png)

### Documentation
Documentation can be found [here](https://silvia-odwyer.github.io/rustvis/docs/rustvis/index.html).

## Run The Examples

<!-- ## Cargo Status -->
<!-- `GDL` can be installed via Cargo by declaring the following dependency in your Cargo.toml file:
```toml
[dependencies]
GDL-rs = "*"
``` -->

Clone this repo:
```sh
git clone https://github.com/silvia-odwyer/rustvis
```

Run the binary, which will create a sample barchart:
```sh
cd crate
cargo run --release 
```

See [/examples](https://github.com/silvia-odwyer/gdl/tree/master/crate/examples) for more examples.

#### WebAssembly
Coming soon:tm:

The WebAssembly version of this library will make use of the browser's canvas, for graphic creation in the browser.