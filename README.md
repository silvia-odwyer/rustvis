# rustvis

#### Data Visualisation with Rust

rustvis is a high-performance Rust data visualisation library, allowing for the creation of responsive, flexible charts. 

rustvis allows you to create:
- Bar charts
- Histograms 
- Gradient-based barcharts/histograms
- Linecharts 

#### Output
Charts are outputted as PNG/JPEG images, but other options such as SVG and HTML/CSS will be available soon. 

### View Example Graphics
![Sample graphics generated with rustvis](https://i.imgur.com/vfZHyU4.png)

### Documentation
Documentation can be found [here](https://silvia-odwyer.github.io/rustvis/docs/index.html).

`rustvis` allows you to either generate a graphic with a size of your choice, or draw charts onto existing graphics by passing
a mutable reference to the drawing function. 

### Appearance/Customization
`rustvis` provides flexible customisation options for the appearance of charts. These include:

- Preset gradients for visual effect
- Changing the background colors of the graphs
- Whether to include a grid or not
- The chart's line colors and x/y axes
- Height/width of the chart. 
- Replacing the bars of charts with images 
- Bar/line colors 


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

#### Notice
While the core functionality is implemented, this is a work-in-progress, and is in heavy development.

#### WebAssembly
Coming soon:tm:

The WebAssembly version of this library will make use of the browser's canvas, for graphic creation in the browser.