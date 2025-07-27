# mdpdf

[![CI](https://github.com/westrik/mdpdf/actions/workflows/CI.yml/badge.svg)](https://github.com/westrik/mdpdf/actions/workflows/CI.yml)
![NPM Version](https://img.shields.io/npm/v/%40mdpdf%2Fmdpdf)

Convert Markdown to PDF with a self-contained Rust binary via CLI or Node.js.

## Features

- Convert Markdown to PDF using [Typst](https://github.com/typst/typst)
- Self-contained (no large external dependencies)
- Cross-platform support (x86_64 and aarch64)
- Node.js bindings via napi-rs
- Syntax highlighting for code blocks

### Example Output

- [demo.pdf](/tests/demo.pdf) ([demo.md](/tests/demo.md))

## Usage

### Basic Usage

#### CLI

```sh
mdpdf README.md -o README.pdf
```

#### Node.js

```
npm install @mdpdf/mdpdf
# or
yarn add @mdpdf/mdpdf
# or
pnpm add @mdpdf/mdpdf
```

```javascript
import mdpdf from "@mdpdf/mdpdf";
const pdfBytes = await mdpdf("# this is markdown");
```

## Development

### CLI

```sh
cargo build --release --features cli
cargo test
```

### Node.js

```sh
npm run build
npm run test
```
