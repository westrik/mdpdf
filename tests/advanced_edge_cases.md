# Advanced Edge Cases Test Suite

This document contains advanced edge cases designed to stress-test the markdown-to-typst conversion library.

## Unicode and Text Processing Edge Cases

### Control Characters and Null Bytes
```
Text with null byte: Hello\x00World
Text with control chars: \x01\x02\x03\x04\x05\x06\x07\x08\x0B\x0C\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F
Text with bell: \x07
Text with backspace: \x08
Text with vertical tab: \x0B
Text with form feed: \x0C
Text with shift out: \x0E
Text with shift in: \x0F
```

### Zero-Width Characters
```
Zero-width space: [​] (U+200B)
Zero-width non-joiner: [‌] (U+200C)
Zero-width joiner: [‍] (U+200D)
Left-to-right mark: [‎] (U+200E)
Right-to-left mark: [‏] (U+200F)
Word joiner: [⁠] (U+2060)
Invisible separator: [⁣] (U+2063)
Invisible plus: [⁤] (U+2064)
```

### Combining Characters and Diacritics
```
Multiple combining marks: a̋é̂ǒ̃ṻ̌ñ̈́
Zalgo text: Ḧ̴̰ë̷́ĺ̸̰l̴̈́ö̸̈ ̵̿W̴̋o̶̓r̸̈́l̸̊d̵̈!
Extreme combining: c̣̈̇̊̃̂̋̌̏̎̍̚
```

### Bidirectional Text
```
Mixed RTL/LTR: English עברית العربية Mixed
RTL override: ‏Right-to-left override
LTR override: ‎Left-to-right override
Bidirectional embedding: ‪LTR embedding‬ ‫RTL embedding‬
```

## HTML Parsing Edge Cases

### Malformed HTML Tags
```
<unclosed tag
<tag with spaces >
<tag/with/forward/slashes>
<tag with="unclosed attribute
<tag with='mixed quotes" and content>
<tag with="nested "quotes" inside">
<tag with='nested 'quotes' inside'>
<tag with="escaped \"quotes\" inside">
<tag with='escaped \'quotes\' inside'>
```

### Complex HTML Attributes
```
<div class="test" data-value="complex 'quoted' \"double\" content" style="color: red; background: url('data:image/svg+xml,<svg xmlns=\"http://www.w3.org/2000/svg\"><circle cx=\"50\" cy=\"50\" r=\"40\" stroke=\"black\" stroke-width=\"3\" fill=\"red\"/></svg>')" onclick="alert('xss')" onmouseover="alert('xss')" onload="alert('xss')">
  Content with <script>alert('xss')</script> and <style>body { color: red; }</style>
</div>
```

### Nested HTML with Unclosed Tags
```
<div>
  <p>Paragraph with <strong>bold <em>italic <code>code</code> with `backticks`</em> and <a href="javascript:alert('xss')">malicious link</a></strong>
  <ul>
    <li>List item with <strong>unclosed bold
    <li>Another item with <em>unclosed italic
  </ul>
  <table>
    <tr><th>Header with <script>alert('xss')</script></th><th>Another header</th></tr>
    <tr><td>Cell with <img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'><rect width='100' height='100' fill='red'/></svg>" alt="SVG data URL" onerror="alert('xss')"></td><td>Normal cell</td></tr>
  </table>
</div>
```

### HTML Entities and Escaping
```
&amp; &lt; &gt; &quot; &apos; &copy; &reg; &trade; &nbsp; &mdash; &ndash; &hellip;
&#x27; &#x2F; &#x3C; &#x3E; &#x22; &#x26;
&#39; &#47; &#60; &#62; &#34; &#38;
```

### Self-Closing Tags
```
<br>
<br/>
<br />
<hr>
<hr/>
<hr />
<img src="test.jpg" alt="test">
<img src="test.jpg" alt="test"/>
<img src="test.jpg" alt="test" />
```

## Markdown Syntax Edge Cases

### Unclosed Markdown Elements
```
**Unclosed bold text
*Unclosed italic text
`Unclosed code
~~Unclosed strikethrough
[Unclosed link](http://example.com
![Unclosed image](http://example.com/image.jpg
> Unclosed blockquote
- Unclosed list item
1. Unclosed ordered list item
```

### Nested Markdown Elements
```
**Bold with *italic inside* and `code` and ~~strikethrough~~**
*Italic with **bold inside** and `code` and ~~strikethrough~~*
`Code with **bold** and *italic* and ~~strikethrough~~`
~~Strikethrough with **bold** and *italic* and `code`~~
```

### Complex Escaping
```
\*escaped asterisk\* \`escaped backtick\` \[escaped bracket\] \#escaped hash \@escaped at \=escaped equals \_escaped underscore \+escaped plus \!escaped exclamation \"escaped quote\" \'escaped single quote\' \<escaped less than\> \>escaped greater than\> \~escaped tilde \$escaped dollar \|escaped pipe \&escaped ampersand \?escaped question \:escaped colon \;escaped semicolon \,escaped comma \.escaped period
```

### Code Block Edge Cases
```
````
```
Code block with unclosed backticks
````

````markdown
```rust
fn main() {
    println!("Hello, world!");
}
```
````

```
Code block ending with backtick: `
```

```
Code block with backticks inside: `code`
```
```

### Link and Image Edge Cases
```
[Link with **bold** and *italic*](http://example.com)
[Link with `code` and ~~strikethrough~~](http://example.com)
[Link with [nested link](http://example.com)](http://example.com)
[Link with ![image](http://example.com/image.jpg)](http://example.com)
[Link with unclosed bracket](http://example.com
[Link with unclosed parenthesis](http://example.com
[Link with unclosed both](http://example.com

![Image with **bold** and *italic*](http://example.com/image.jpg)
![Image with `code` and ~~strikethrough~~](http://example.com/image.jpg)
![Image with [link](http://example.com)](http://example.com/image.jpg)
![Image with unclosed bracket](http://example.com/image.jpg
![Image with unclosed parenthesis](http://example.com/image.jpg
```

## Table Edge Cases

### Malformed Tables
```
| Header 1 | Header 2 | Header 3
| Cell 1 | Cell 2 | Cell 3 |
| Cell 4 | Cell 5 | Cell 6

| Header 1 | Header 2 | Header 3 |
| Cell 1 | Cell 2
| Cell 4 | Cell 5 | Cell 6 |

| Header 1 | Header 2 | Header 3 |
|:----------|:----------:|----------:|
| Cell 1 | Cell 2 | Cell 3 |
| Cell 4 | Cell 5 | Cell 6 |

| Header 1 | Header 2 | Header 3 |
|:----------|:----------:|----------:|
| Cell 1 with `code` | Cell 2 with **bold** | Cell 3 with _italic_ |
| Cell 4 with [link](http://example.com) | Cell 5 with ~~strikethrough~~ | Cell 6 with #hash |
```

### Tables with Special Characters
```
| Column with #hash | Column with @at | Column with $dollar |
|:------------------|:----------------:|-------------------:|
| Cell with \backslash | Cell with "quotes" | Cell with 'single quotes' |
| Cell with <tags> | Cell with [brackets] | Cell with {braces} |
| Cell with (parentheses) | Cell with |pipe| | Cell with &ampersand |
```

## List Edge Cases

### Deeply Nested Lists
```
- Level 1
  - Level 2
    - Level 3
      - Level 4
        - Level 5
          - Level 6
            - Level 7
              - Level 8
                - Level 9
                  - Level 10
                    - Level 11
                      - Level 12
                        - Level 13
                          - Level 14
                            - Level 15
                              - Level 16
                                - Level 17
                                  - Level 18
                                    - Level 19
                                      - Level 20
                                        - Level 21
                                          - Level 22
                                            - Level 23
                                              - Level 24
                                                - Level 25
                                                  - Level 26
                                                    - Level 27
                                                      - Level 28
                                                        - Level 29
                                                          - Level 30
                                                            - Level 31
                                                              - Level 32
                                                                - Level 33
                                                                  - Level 34
                                                                    - Level 35
                                                                      - Level 36
                                                                        - Level 37
                                                                          - Level 38
                                                                            - Level 39
                                                                              - Level 40
                                                                                - Level 41
                                                                                  - Level 42
                                                                                    - Level 43
                                                                                      - Level 44
                                                                                        - Level 45
                                                                                          - Level 46
                                                                                            - Level 47
                                                                                              - Level 48
                                                                                                - Level 49
                                                                                                  - Level 50
```

### Mixed List Types
```
1. Ordered item 1
   - Unordered subitem 1
   - Unordered subitem 2
2. Ordered item 2
   - Unordered subitem 3
   - Unordered subitem 4
     1. Nested ordered item 1
     2. Nested ordered item 2
        - Deep nested unordered item
```

### Lists with Complex Content
```
- List item with **bold** and *italic* and `code` and ~~strikethrough~~
- List item with [link](http://example.com) and ![image](http://example.com/image.jpg)
- List item with #hash and @at and $dollar
- List item with <html> tags and &amp; entities
- List item with unclosed **bold
- List item with unclosed *italic
- List item with unclosed `code
- List item with unclosed ~~strikethrough
```

## Blockquote Edge Cases

### Deeply Nested Blockquotes
```
> Level 1
> > Level 2
> > > Level 3
> > > > Level 4
> > > > > Level 5
> > > > > > Level 6
> > > > > > > Level 7
> > > > > > > > Level 8
> > > > > > > > > Level 9
> > > > > > > > > > Level 10
```

### Blockquotes with Complex Content
```
> Blockquote with **bold** and *italic* and `code` and ~~strikethrough~~
> 
> Blockquote with [link](http://example.com) and ![image](http://example.com/image.jpg)
> 
> Blockquote with #hash and @at and $dollar
> 
> Blockquote with <html> tags and &amp; entities
> 
> Blockquote with unclosed **bold
> 
> Blockquote with unclosed *italic
> 
> Blockquote with unclosed `code
> 
> Blockquote with unclosed ~~strikethrough
```

## Heading Edge Cases

### Headings with Special Characters
```
# Heading with #hash and @at and $dollar
## Heading with **bold** and *italic* and `code`
### Heading with [link](http://example.com) and ![image](http://example.com/image.jpg)
#### Heading with <html> tags and &amp; entities
##### Heading with unclosed **bold
###### Heading with unclosed *italic
```

### Headings with Unicode
```
# Heading with 🚀🌟🎉💯🔥💪😎🤯⚡️💥🎊🎈
## Heading with العربية 한국어 中文 日本語 हिन्दी עברית русский
### Heading with 𝕌𝕟𝕚𝕔𝕠𝕕𝕖 𝔪𝔞𝔱𝔥 𝓈𝓎𝓂𝒷𝑜𝓁𝓈
#### Heading with 🌍🔥💻📊🎯🚀⚡🎨🔧📱
##### Heading with ∑∏∫∮∯∰∱∲∳∴∵∶∷∸∹∺∻∼∽∾∿
###### Heading with a̋é̂ǒ̃ṻ̌ñ̈́
```

## Math Expression Edge Cases

### Inline Math
```
Inline math: $E = mc^2$ and $\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}$
Math with special chars: $f(x) = #function(x)$ and $g(x) = @variable(x)$
Math with unicode: $\sum_{i=1}^{n} x_i = \prod_{j=1}^{m} y_j$
Unclosed math: $E = mc^2
Math with nested: $f(x) = \frac{1}{1 + e^{-x}}$
```

### Display Math
```
$$
\frac{\partial f}{\partial x} = \lim_{h \to 0} \frac{f(x + h) - f(x)}{h}
$$

$$
\begin{align}
E &= mc^2 \\
F &= ma \\
P &= mv
\end{align}
$$

$$
Unclosed display math
$$

$$
Math with #function and @variable
$$
```

## Typst Syntax Conflicts

### Typst Function Calls
```
#set page("a4")
#show link: underline
#let hrule = line(length: 100%)
#set text(font: "Libertinus Serif")
#set text(size: 12pt)
#let my_function(x) = x * 2
#show heading: it => [*#it.body*]
```

### Typst Variables and References
```
@my-variable
@my_function
@my-function
@my_function(x, y)
@my-function(x, y)
```

### Typst Content Blocks
```
#[
  This is a content block
  With multiple lines
  And #nested functions
  And @references
  And $math$
]
```

### Typst Show Rules
```
#show heading: it => [
  #it.body with @ref and $math$
]

#show "@": [BROKEN REFERENCE]
#show "$": [BROKEN MATH]
#show "#": [BROKEN FUNCTION]
```

## Performance and Memory Edge Cases

### Large Content
```
# Very Long String Without Spaces

ThisIsAnExtremelyLongStringWithoutAnySpacesOrBreaksThatCouldPotentiallyBreakTextWrappingOrParsingAlgorithmsInTypstDocumentsAndMightCauseMemoryOrPerformanceIssuesWhenProcessingVeryLargeDocumentsWithSimilarContentThisIsAnExtremelyLongStringWithoutAnySpacesOrBreaksThatCouldPotentiallyBreakTextWrappingOrParsingAlgorithmsInTypstDocumentsAndMightCauseMemoryOrPerformanceIssuesWhenProcessingVeryLargeDocumentsWithSimilarContentThisIsAnExtremelyLongStringWithoutAnySpacesOrBreaksThatCouldPotentiallyBreakTextWrappingOrParsingAlgorithmsInTypstDocumentsAndMightCauseMemoryOrPerformanceIssuesWhenProcessingVeryLargeDocumentsWithSimilarContent

# Repeated Characters

################################################################################
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
```

### Deep Nesting
```
#function(#function(#function(#function(#function(#function(#function(#function(#function(#function())))))))))
@ref[@ref[@ref[@ref[@ref[@ref[@ref[@ref[@ref[@ref[content]]]]]]]]]]
$x^{x^{x^{x^{x^{x^{x^{x^{x^{x}}}}}}}}}$
```

## Security and Injection Edge Cases

### XSS Attempts
```
<script>alert('XSS')</script>
javascript:alert('XSS')
data:text/html,<script>alert('XSS')</script>
<img src="x" onerror="alert('XSS')">
<a href="javascript:alert('XSS')">Click me</a>
<svg onload="alert('XSS')">
<iframe src="javascript:alert('XSS')">
```

### SQL Injection Style
```
'; DROP TABLE users; --
" OR 1=1; --
UNION SELECT * FROM passwords
```

### Command Injection Style
```
$(rm -rf /)
`rm -rf /`
| rm -rf /
; rm -rf /
&& rm -rf /
|| rm -rf /
```

## File Path and URL Edge Cases

### Problematic File Paths
```
/path/with/@symbols
C:\Windows\Path\With\#Hash
./relative/path/with spaces
../parent/path/with'quotes
~/home/path/with$dollar
/path/with/unicode/🚀/🌟/🎉
```

### URL Breaking Characters
```
https://example.com/path?query=value&other=@symbol
http://user:pass@domain.com/path#fragment
ftp://[::1]:8080/path/with spaces
file:///C:/Windows/System32/drivers
data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg"><script>alert('XSS')</script></svg>
```

## Mixed Content Edge Cases

### Complex Nested Structures
```
<div>
  <h1>Title with **bold** and *italic*</h1>
  <p>Paragraph with <strong>HTML bold</strong> and <em>HTML italic</em> and `code` and ~~strikethrough~~</p>
  <blockquote>
    <p>Blockquote with [link](http://example.com) and ![image](http://example.com/image.jpg)</p>
    <ul>
      <li>List item with #hash and @at and $dollar</li>
      <li>List item with <html> tags and &amp; entities</li>
    </ul>
  </blockquote>
  <table>
    <tr><th>Header with **bold**</th><th>Header with *italic*</th></tr>
    <tr><td>Cell with `code`</td><td>Cell with ~~strikethrough~~</td></tr>
  </table>
</div>
```

### Markdown Inside HTML
```
<div>
  # Markdown heading inside HTML
  **Bold text** and *italic text* and `code`
  - List item 1
  - List item 2
  > Blockquote inside HTML
  [Link](http://example.com) and ![Image](http://example.com/image.jpg)
</div>
```

### HTML Inside Markdown
```
# Markdown heading

**Bold text** with <strong>HTML bold</strong> and *italic text* with <em>HTML italic</em>

- List item with <code>HTML code</code>
- List item with <a href="http://example.com">HTML link</a>

> Blockquote with <blockquote>nested HTML blockquote</blockquote>

| Column 1 | Column 2 |
|----------|----------|
| Cell with <strong>HTML</strong> | Cell with <em>HTML</em> |
```

## Final Stress Test

### Ultimate Breaking String
```
#function(@ref["string with #nested(@ref) and $math$ and \\escapes"], key: value, ..args)[Content with @more-refs and #more-functions($nested-math$) and "more strings" and /* fake comments */ and // more fake comments and 中文 and 🚀 and ∑∏∫ and \\\\\\]
```

### Mixed Everything
```
This document contains: #functions, @references, $math$, "strings", \escapes, @miксed-scripts, 🌍emojis, ∑math-symbols, /* comments */, // more comments, nested #[content with @refs and $x^2$], URLs like https://example.com/@user, file paths like C:\Path\With\#Hash, and various unicode: العربية 한국어 中文 ñ̈́ ḧ̃̂ë̊̌ḷ̸̍l̴̾ö̵̈ and binary \x00\xFF and long strings ThisIsVeryLongWithoutSpaces and repeated ###################### characters.

<div>
  <h1>HTML with **Markdown** and <strong>HTML</strong></h1>
  <p>Mixed content with `code`, <code>HTML code</code>, ~~strikethrough~~, and <strike>HTML strikethrough</strike></p>
  <ul>
    <li>List with [link](http://example.com) and <a href="http://example.com">HTML link</a></li>
    <li>List with ![image](http://example.com/image.jpg) and <img src="http://example.com/image.jpg" alt="HTML image"></li>
  </ul>
  <blockquote>
    <p>Blockquote with #hash, @at, $dollar, and &amp; entities</p>
  </blockquote>
</div>

| Column with `code` | Column with **bold** | Column with _italic_ | Column with [link](url) |
|:-------------------|:---------------------|:---------------------|:----------------------:|
| Cell with #set page("a4") | Cell with #show link: underline` | Cell with `#let hrule = line(length: 100%)` | Cell with `[brackets]` |
| Cell with `*bold\*` | Cell with ``code`` | Cell with `#heading` | Cell with `=title` |
| Cell with `+list` | Cell with `"quote"` | Cell with `'single'` | Cell with `<tag>` |
| Cell with `>close>` | Cell with `@symbol` | Cell with `=equals` | Cell with `#hash` |

```rust
fn main() {
    println!("Hello, world!");
    // This contains Typst syntax: #set page("a4")
    let x = 42;
    // More Typst syntax: #show link: underline
    // And some math: $E = mc^2$
    // And some references: @my_function
}
```

> Blockquote with **bold** and *italic* and `code` and ~~strikethrough~~
> 
> And [links](http://example.com) and ![images](http://example.com/image.jpg)
> 
> And #hash and @at and $dollar
> 
> And <html> tags and &amp; entities

- List item with **bold** and *italic* and `code` and ~~strikethrough~~
- List item with [link](http://example.com) and ![image](http://example.com/image.jpg)
- List item with #hash and @at and $dollar
- List item with <html> tags and &amp; entities
- List item with unclosed **bold
- List item with unclosed *italic
- List item with unclosed `code
- List item with unclosed ~~strikethrough

1. Ordered item with **bold** and *italic* and `code` and ~~strikethrough~~
2. Ordered item with [link](http://example.com) and ![image](http://example.com/image.jpg)
3. Ordered item with #hash and @at and $dollar
4. Ordered item with <html> tags and &amp; entities
5. Ordered item with unclosed **bold
6. Ordered item with unclosed *italic
7. Ordered item with unclosed `code
8. Ordered item with unclosed ~~strikethrough

---

*Note: This test data is designed to stress-test the markdown-to-typst conversion library. Use with caution in production environments.* 