# Singularity

POC for LLM Client

## Using [`mistral.rs`](https://github.com/EricLBuehler/mistral.rs)

Rust framework for llm inference which looks promising and works, but has weird API(`String` where you need `Path` etc) especially for usage of local models or better to say lack of thereof. Metal backend works, but may be in conflict with some Gguf models as `Qwen3-4B-GGUF` fails to start.

Better wait for `wgpu` support in `candle` and improvements in `mistral.rs` and this will be great cross-platform solution.

## Developers

Developers should install [rustup][rustup] and configure their editor to use [rust-analyzer][rust-analyzer].

[fluent]: https://projectfluent.org/
[fluent-guide]: https://projectfluent.org/fluent/guide/hello.html
[iso-codes]: https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
[rustup]: https://rustup.rs/
[rust-analyzer]: https://rust-analyzer.github.io/