# `rust-mos-hello-world`

Minimal hello-world example for mos-*-none targets, provided by https://github.com/mrk-its/rust-mos

## Running

The easiest way is to use provided `devcontainer.json` configuration for vscode:

1. Configure Visual Studio Code with `Remote - Containers` extension
2. Open this project inside devcontainer
3. In vscode terminal do:
    ```
      # build for mos-sim-none arch and run in `mos-sim` emulator 
      cargo run 

      # build for mos-atari8-none target
      cargo build --target mos-atari8-none

      # build for mos-nes-nrom-128-none target
      cargo build --target mos-nes-nrom-128-none
    ```

## License

All source code (including code snippets) is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.
