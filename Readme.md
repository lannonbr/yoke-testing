# Yoke Exploration

A repo testing out [yoke](https://github.com/yokecd/yoke) in various languages

Languages tested so far:

- Golang
- Rust

## Setup

- Install yoke
- Install your language of choice
  - For Rust, make sure that the target `wasm32-wasip1` is installed. you can do such with rustup as follows: `rustup target add wasm32-wasip1`
  - Go will automatically cross-compile if you define the proper GOOS and GOARCH env variables.
- Build the main program for each language
  - Rust: `cd rust`, `cargo build --target=wasm32-wasip1`
  - Go: `cd go`, `GOOS=wasip1 GOARCH=wasm go build -o example.wasm main.go`
- Run `yoke takeoff <release_name> <wasm_file>`

## Things noticed during development

- Go is going to be the best supported due to the Kubernetes APIs being native to Go and the packages maintained by the Kubernetes Team directly.
- Rust had some hiccups when trying to collapse a bunch of different resources into a single Vec to be deployed due to Rust's strict type safety. The workaround I did was used `serde_json` early to convert a given resource into a `Value` type and then made a `Vec<Value>` which then was finally converted to a JSON string at the end.
