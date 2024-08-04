# daff-wasm

A [wasm](https://webassembly.org/) module for the `daff-rs` library.

# Usage

```ts
import init, { Text, type TextChanges } from 'TODO';

await init(); // Initializes WASM module

const text = new Text('a,b,c\n1,2,3')

const changes: TextChanges = text.compare(Text('a,b,c\n1,2,4'));
```
