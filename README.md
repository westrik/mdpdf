# mdpdf

Convert Markdown to PDF with a self-contained Rust binary. Available for CLI and Node.js.

## Features

- Convert Markdown to PDF using [Typst](https://github.com/typst/typst)
- Self-contained (no large external dependencies)
- Cross-platform support (x86_64 and aarch64)
- Node.js bindings via napi-rs
- Syntax highlighting for code blocks

## Usage

### Basic Usage

#### CLI

```sh
mdpdf README.md -o README.pdf
```

#### Node.js

```javascript
import { markdownToPdf, markdownToTypstCode } from "mdpdf";
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
