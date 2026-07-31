# Changelog

## 0.1.7

- Allow Node.js callers to supply a custom Typst preamble. ([#10](https://github.com/westrik/mdpdf/pull/10))
- Add custom font support and document configuration for page size, margins, font size, font paths, and a table of contents. ([#1](https://github.com/westrik/mdpdf/pull/1), [#3](https://github.com/westrik/mdpdf/pull/3), [#4](https://github.com/westrik/mdpdf/pull/4))
- Add advanced math rendering, internal links, Markdown footnotes, and GitHub-style alerts. ([#2](https://github.com/westrik/mdpdf/pull/2), [#3](https://github.com/westrik/mdpdf/pull/3), [#11](https://github.com/westrik/mdpdf/pull/11), [#12](https://github.com/westrik/mdpdf/pull/12))
- Expand the Node.js API with an options object and `markdownToPdfWithStats()` for conversion and rendering statistics. The previous positional arguments remain available but are deprecated. ([#4](https://github.com/westrik/mdpdf/pull/4))
- Add `aarch64-unknown-linux-musl` binaries. ([#6](https://github.com/westrik/mdpdf/pull/6))
- Fix Markdown conversion for duplicate or formatted heading links and footnotes with extended Markdown content or labels containing spaces. ([0eb9a5e](https://github.com/westrik/mdpdf/commit/0eb9a5e5a543dbb1618fe85dc487277f6199e260))
- Make Node.js builds use the package-pinned Yarn version and preserve generated TypeScript declarations when a build fails. ([203313a](https://github.com/westrik/mdpdf/commit/203313a1af83bba3a3b170095dff3354a9af0d8a))

Contributors: [@varunsharma27](https://github.com/varunsharma27) and [@sabraman](https://github.com/sabraman).

## 0.1.6

- Expose `evict()` to let callers reclaim native memory. ([#8](https://github.com/westrik/mdpdf/pull/8))

Contributors: [@mako-taco](https://github.com/mako-taco)

## 0.1.4

- Build binaries for `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu` with glibc 2.34 to support, e.g., Amazon Linux 2023.

## 0.1.3

- Disable YAML metadata support.

## 0.1.2

- Improve header spacing.

## 0.1.1

- Make exports `async`.

## 0.1.0

- Initial release.
