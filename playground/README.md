# playground

An interactive playground for the `daff-rs` project. This project uses the `daff-wasm` API which provides an interface to the `daff-rs` implementations.

# Getting started

Install the [Rust toolchain](https://rustup.rs/).

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install [`pnpm`](https://pnpm.io/installation).

```
curl -fsSL https://get.pnpm.io/install.sh | sh -
```

Install LTS Node.

```
pnpm env use --global lts
```

Install [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/).

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Install the dependencies.

```
pnpm install
```

Build the Wasm package.

```
pnpm run dev:wasm
```

Run the development server.

```
pnpm dev
```
