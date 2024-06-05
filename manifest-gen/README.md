# manifest-gen

Generates manifest JSON file from partition binaries.

## Build

Install the latest Rust toolchain and run cargo.

```
cargo build
```

## Usage

e.g. Generate `test` application version `1.0` manifest for ESP32-S3. 

```
cargo run -- --name test --version 1.0 --chip-family esp32s3 --part-data bootloader.bin --part-offset 0 --part-data ota_data.bin --part-offset 0x10000 --part-data application.bin --part-offset 0x14000 --output test_manifest.json
```