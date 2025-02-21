# `matrix-sdk-crypto-js`

Welcome to the [WebAssembly] + JavaScript binding for the Rust
[`matrix-sdk-crypto`] library! WebAssembly can run anywhere, but these
bindings are designed to run on a JavaScript host. These bindings are
part of the [`matrix-rust-sdk`] project, which is a library
implementation of a [Matrix] client-server.

`matrix-sdk-crypto` is a no-network-IO implementation of a state
machine, named `OlmMachine`, that handles E2EE ([End-to-End
Encryption](https://en.wikipedia.org/wiki/End-to-end_encryption)) for
[Matrix] clients.

## Usage

These WebAssembly bindings are written in [Rust]. To build them, you
need to install the Rust compiler, see [the Install Rust
Page](https://www.rust-lang.org/tools/install). Then, the workflow is
pretty classical by using [npm], see [the Downloading and installing
Node.js and npm
Page](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm).

Once the Rust compiler, Node.js and npm are installed, you can run the
following commands:

```sh
$ npm install
$ npm run build
$ npm run test
```

A `matrix_sdk_crypto.js`, `matrix_sdk_crypto.d.ts` and a
`matrix_sdk_crypto_bg.wasm` files should be generated in the `pkg/`
directory.

TBD

## Documentation

To generate the documentation, please run the following command:

```sh
$ npm run doc
```

The documentation is generated in the `./docs` directory.



[WebAssembly]: https://webassembly.org/
[`matrix-sdk-crypto`]: https://github.com/matrix-org/matrix-rust-sdk/tree/main/crates/matrix-sdk-crypto
[`matrix-rust-sdk`]: https://github.com/matrix-org/matrix-rust-sdk
[Matrix]: https://matrix.org/
[Rust]: https://www.rust-lang.org/
[npm]: https://www.npmjs.com/
