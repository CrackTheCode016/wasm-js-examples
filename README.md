# Blob - Learn WebAssembly x Rust

> **Note:** This is mostly just slightly elaborated examples from the [wasm_bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) docs.

WebAssembly is a portable, sandboxed binary format, originally designed to compliment Javascript in the browser.  As of today, many other languages have native support for compiling to WebAssembly, such as Rust, C, C++, or Go.  It is generally high performance, running at near-native speeds, making it ideal for offloading heavier computation in.  

It makes sense that one also would want to use WebAssembly **outside** of the browser as well.  Because of this, various WebAssembly runtimes have been developed, which allow for WebAssembly "blobs", or programs, to be executed beyond the browser.  Below is a list of noteworthy projects, both in and outside the browser, which use the portability and performance benefits of WebAssembly to improve the overall experience: 

- **Google Earth**, uses WebAssembly to run their otherwise C++-based codebase within the browser.
- **Substrate**, a framework for building blockchains, uses WebAssembly as the core runtime aspect, allowing for portable representations of entire blockchains' business logic as a single binary.
- **CosmWasm**, Cosmos blockchain smart contract plugins compile to WebAssembly.
- **Figma**, sped up their load times by 3x by compiling their C++ codebase to WebAssembly. 

This repository serves as a small example project of why, and how WebAssembly can be brought out of the browser context.  It includes an example project, written in Rust, where both the host and the WebAssembly library can be written and interact in the same language.

## Different Flavors of WebAssembly - Breaking out of the browser

It's important to remember that WebAssembly is just that - a binary format.  At its core, it is really a binary format that is supposed to be ran in a virtual machine that knows how to interpret WebAssembly.

## Necessary Components of WebAssembly

- Imports
- Exports
- Locals
- Modules
- Memory
- Host functions

## Building

```
npm install
npm run serve
```

## Running

After serving, head to http://localhost:8080.