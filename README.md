# mdpdf

[![CI](https://github.com/westrik/mdpdf/actions/workflows/CI.yml/badge.svg)](https://github.com/westrik/mdpdf/actions/workflows/CI.yml)
[![NPM Version](https://img.shields.io/npm/v/%40mdpdf%2Fmdpdf)](https://www.npmjs.com/package/@mdpdf/mdpdf)

Convert Markdown to PDF with a self-contained Rust binary via CLI or Node.js.

## Features

- Convert Markdown to PDF using [Typst](https://github.com/typst/typst)
- Self-contained (no large external dependencies)
- Cross-platform support (x86_64 and aarch64)
- Node.js bindings via napi-rs
- Syntax highlighting for code blocks
- GitHub Flavored Markdown features, including alerts, footnotes, task lists, tables, and strikethrough
- Math rendering, internal links, and optional tables of contents
- Custom fonts, page sizes, margins, and Typst configuration

### Example Output

- [Example PDF](https://github.com/westrik/mdpdf/blob/main/tests/demo.pdf) ([Source](https://raw.githubusercontent.com/westrik/mdpdf/refs/heads/main/tests/demo.md))

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
import { markdownToPdf, markdownToPdfWithStats } from "@mdpdf/mdpdf";

const options = {
  pageSize: "a4",
  margin: "20mm",
  fontFamily: "Liberation Serif",
  fontPaths: ["./fonts"],
  fontSize: 11,
  toc: true,
  typstConfig: "#set text(fill: navy)",
};

const pdfBytes = await markdownToPdf("# This is Markdown", options);
const { pdf, stats } = await markdownToPdfWithStats(
  "# This is Markdown",
  options,
);
```

`markdownToPdfWithStats` returns the generated PDF and character, line, conversion-time,
and rendering-time statistics. `markdownToTypstCode` accepts the same options. Existing
positional Node.js arguments remain supported but are deprecated; pass an options object
for new code.

The CLI accepts equivalent document options:

```sh
mdpdf document.md --output document.pdf --page-size a4 --margin 20mm \
  --font-size 11 --font-path ./fonts --toc
```

## Development

### CLI

```sh
cargo build --release --features cli
cargo test
```

### Node.js

```sh
corepack yarn install --immutable
corepack yarn build
CI=1 corepack yarn test
```

This project requires the Yarn version pinned in `package.json`. Use
`corepack yarn` rather than a globally installed `yarn`; this avoids falling
back to Yarn 1 and ensures failed N-API builds cannot leave `index.d.ts` empty.
