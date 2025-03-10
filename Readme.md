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

## Air Traffic Controller / Yoke Airway

I created a folder `rust-airway` that defines a test [Airway](https://yokecd.github.io/docs/airtrafficcontroller/atc/) defining a CRD of SuperService. The logic is basic, but it generates 2 wasm files if you build with `wasm32-wasip1`:

- `rust-airway.wasm` the actual airway flight to be sent to `yoke takeoff` to create the CRD.
- `superservice-flight.wasm` the flight that triggers when a manifest for `SuperService` is sent to the cluster with `kubectl apply` or any other method of creating new resources (Helm, Client Libraries, etc).

If you read through the source of the first, there is a URL defined in the WasmUrls section of the AirwaySpec that tells Yoke ATC where to find `superservice-flight.wasm` and for local testing I just ran a quick node server that hosts the static wasm file on it. In the future you will be able to consume that flight URL via a OCI image and then it can be pushed to somewhere like GitHub Container Registry or Docker Hub. At the time of writing this there is an open PR to do this: https://github.com/yokecd/yoke/pull/87

Also some caveats, I use the CustomResource derive macro from the kube crate, and to make the program compile to WASM, you need to disable the default features of the crate to make it actually compile.
