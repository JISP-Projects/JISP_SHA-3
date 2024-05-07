# JISP SHA-3

This crate is my pure-rust implementation of SHA-3, this implementation is just my personal project and has not been officially verified or audited and should therefore not be used in any real-world applications, it is only meant for small personal projects such as mine. 

# Running the simple GUI
For demonstration purposes I have also made a simple User Interface in this [project](https://github.com/JISP-Projects/JISP_SHA-3)
<img src="https://raw.githubusercontent.com/JISP-Projects/JISP_SHA-3/master/images/SHA-3.png" width="100%">

If you want to use this interface yourself you need to have the rust compiler [Cargo](https://www.rust-lang.org/tools/install) installed.

To execute this program you need to then clone this repository and run
```
cargo run -p jisp_sha3_gui
```
in the directory. Alternatively you can also use `cargo build` and then run the executable found in the `/target` folder

# Installing Crate
In order to use this crate in your own rust projects you need to add the following to your `Cargo.toml` file
```
[dependencies]
jisp_sha2 = "0.1.0"
```

# Documentation
Look at the [docs.rs](https://docs.rs/jisp_sha3/0.2.0) to learn more about how to use this project!