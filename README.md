# Description

This is a reproducer for `wasi:http` dependency of Wasmtime leaking into the preview2 adapter.

The produced binary will attempt to encode a component using an compiled-in reactor Rust module depending on `wasi-http` and reactor adapter passed on STDIN.

For example, with `dev` tag of https://github.com/bytecodealliance/wasmtime pointing to https://github.com/bytecodealliance/wasmtime/commit/9e4d44626a008f8a1b1b3b4a48f0d5693185844b

```
$ curl -sL https://github.com/bytecodealliance/wasmtime/releases/download/dev/wasi_snapshot_preview1.reactor.wasm | cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/wasmtime-adapter-http`
reading adapter from STDIN...
read 117379 byte-long adapter from STDIN
Error: failed to add WASI adapter

Caused by:
    0: failed to merge WIT packages of adapter `wasi_snapshot_preview1` into main packages
    1: failed to merge package `wasi:http` into existing copy
    2: failed to merge interface `types`
    3: expected type `incoming-body` to be present
```

Note, `component/wit/deps/http` points to https://github.com/WebAssembly/wasi-http/commit/c33d6f09f88aceb48123b8c5778ee864f4524079, which does not define a `incoming-body` type: https://github.com/WebAssembly/wasi-http/blob/c33d6f09f88aceb48123b8c5778ee864f4524079/wit/types.wit.

`incoming-body` *is*, however, defined in the `wasi-http` dependency used by Wasmtime repository used to build the adapter: https://github.com/bytecodealliance/wasmtime/blob/9e4d44626a008f8a1b1b3b4a48f0d5693185844b/crates/wasi-http/wit/deps/http/types.wit#L154.

The encoding error suggests that the `incoming-body` resource type has "bled" into the adapter causing a conflict.
