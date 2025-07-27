- [ ] section links
- [ ] latex math to typst math syntax (for `${}$` / `$${}$$` / \`\`\`math) using https://crates.io/crates/tex2typst-rs (?)

- [ ] aarch64 musl build
- [ ] use correct left border color for github blockquotes

- [ ] improve node api:

```ts
interface Options {
    pageSize: string;
    margin: {left: string, right: string, top: string, bottom: string}; 
}
interface MarkdownToPdfStatistics {
    totalCharacters: number;
    totalLines: number;
    totalImages: number;
    totalLinks: number;
    // etc.
    conversionTimeMs: number;
    renderingTimeMs: number;
}
interface MarkdownToPdfResult {
    pdf: Buffer;
    statistics: MarkdownToPdfStatistics;
    typstCode: string;
}
export declare function markdownToPdf(markdown: string, options?: Options): Promise<MarkdownToPdfResult>
// make this the default export
```