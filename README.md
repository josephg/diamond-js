# Diamond JS wrapper library

This is a javascript / WASM wrapper around [diamond types](https://github.com/josephg/diamond-types).

This code is currently experimental WIP. Do not trust this for anything important. The API can and will change without notice.

### Building

Since diamond-types is not yet published to cargo, you'll need to:

1. Check out diamond-types locally
2. Check out diamond-js (this package) locally in an adjacent directory
3. Build:

```
$ wasm-pack build --target nodejs
```