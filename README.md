# snippy
a simple command line tool for [sniplink](https://beta.sniplink.net/).

## usage
```console
snippy get [sniplink url id]
```
```console
snippy shorten [url] --nocopy --duration=30
```

## disclaimer
this project was made simply as a short foray into rust, i will likely not continue development on it.

## installation
> **presumptions**: you have git and rust/cargo properly installed and added to your `path`.   


you can either **build from source**, or download the binary from the [releases page](https://github.com/ibra/snippy/releases). 

To build from source, clone the repository:
```
git clone https://github.com/ibra/snippy
```
then run:
```
cargo build --release
```
this should then create an executable in `./target/release/snippy.exe`. you can add this exe to your `path`.

## contributing
find or make an issue you would like to work on, and fork the project and make a separate branch of which the naming conventions depend on the type of fix or patch: `feature-custom-duration`, `bug-bad-formatting`   
etc.

