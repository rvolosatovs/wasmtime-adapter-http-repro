use std::io::{self, Read, Write};

use anyhow::Context;

fn encode_component(module: impl AsRef<[u8]>, adapter: &[u8]) -> anyhow::Result<Vec<u8>> {
    wit_component::ComponentEncoder::default()
        .validate(true)
        .module(module.as_ref())
        .context("failed to set core component module")?
        .adapter("wasi_snapshot_preview1", adapter)
        .context("failed to add WASI adapter")?
        .encode()
        .context("failed to encode a component")
}

fn main() -> anyhow::Result<()> {
    let mut adapter = vec![];
    eprintln!("reading adapter from STDIN...");
    let n = io::stdin()
        .read_to_end(&mut adapter)
        .context("failed to read STDIN")?;
    eprintln!("read {n} byte-long adapter from STDIN");
    let component = encode_component(
        include_bytes!(concat!(env!("OUT_DIR"), "/component.wasm")),
        &adapter,
    )?;
    eprintln!("writing component to STDOUT...");
    io::stdout()
        .lock()
        .write_all(&component)
        .context("failed to write component to STDOUT")
}
