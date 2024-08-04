# daff-wasm

A [wasm](https://webassembly.org/) module for the `daff-rs` library.

# Usage

```ts
import init, { Csv, type Changes } from 'TODO';

await init(); // Initializes WASM module

const csv = new Csv('a,b,c\n1,1,1')

const changes: Changes = csv.compare('a,b,c\n2,1,1');
```
