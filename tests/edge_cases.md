# Extreme Edge Case Test

## Unicode and Special Characters
This contains extreme Unicode: 🚀🌟🎉💯🔥💪😎🤯⚡️💥🎊🎈🎁🎂🎄🎃🎅🎆🎇🎈🎉🎊🎋🎌🎍🎎🎏🎐🎑🎒🎓🎔🎕🎖🎗🎘🎙🎚🎛🎜🎝🎞🎟🎠🎡🎢🎣🎤🎥🎦🎧🎨🎩🎪🎫🎬🎭🎮🎯🎰🎱🎲🎳🎴🎵🎶🎷🎸🎹🎺🎻🎼🎽🎾🎿🏀🏁🏂🏃🏄🏅🏆🏇🏈🏉🏊🏋🏌🏍🏎🏏🏐🏑🏒🏓🏔🏕🏖🏗🏘🏙🏚🏛🏜🏝🏞🏟🏠🏡🏢🏣🏤🏥🏦🏧🏨🏩🏪🏫🏬🏭🏮🏯🏰🏱🏲🏳🏴🏵🏶🏷🏸🏹🏺🏻🏼🏽��🏿

## Typst Syntax Conflicts
This should break Typst: `#set page("a4")` and `#show link: underline` and `#let hrule = line(length: 100%)`

## Complex Escaping
Test escaping: `\*bold\*` `\`code\`` `\[link\]` `\#heading` `\=title` `\+list` `\"quote\"` `\'single\'` `\<tag\>` `\>close\>`

## Malformed HTML with Complex Attributes
<div class="test" data-value="complex 'quoted' \"double\" content" style="color: red; background: url('data:image/svg+xml,<svg xmlns=\"http://www.w3.org/2000/svg\"><circle cx=\"50\" cy=\"50\" r=\"40\" stroke=\"black\" stroke-width=\"3\" fill=\"red\"/></svg>')">
  <p>This has <strong>bold <em>italic <code>code</code> with `backticks`</em> and <a href="javascript:alert('xss')">malicious link</a></strong></p>
  <table border="1" cellpadding="0" cellspacing="0" style="width: 100%; border-collapse: collapse;">
    <tr><th>Header with <script>alert('xss')</script></th><th>Another header</th></tr>
    <tr><td>Cell with <img src="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'><rect width='100' height='100' fill='red'/></svg>" alt="SVG data URL" onerror="alert('xss')"></td><td>Normal cell</td></tr>
  </table>
</div>

## Extremely Nested Structures
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

## Complex Tables with Special Characters
| Column 1 with `code` | Column 2 with **bold** | Column 3 with _italic_ | Column 4 with [link](url) |
|:---------------------|:----------------------:|----------------------:|----:|
| Cell with #set page("a4") | Cell with #show link: underline` | Cell with `#let hrule = line(length: 100%)` | Cell with `[brackets]` |
| Cell with `*bold\*` | Cell with ``code`` | Cell with `#heading` | Cell with `=title` |
| Cell with `+list` | Cell with `"quote"` | Cell with `'single'` | Cell with `<tag>` |
| Cell with `>close>` | Cell with `@symbol` | Cell with `=equals` | Cell with `#hash` |

## Math Expressions (Incomplete Implementation)
Inline math: $E = mc^2$ and $\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}$

Display math:
$$
\frac{\partial f}{\partial x} = \lim_{h \to 0} \frac{f(x + h) - f(x)}{h}
$$

## Code Blocks with Typst Syntax
```typst
#set page("a4")
#show link: underline
#let hrule = line(length: 100%)
#set text(font: "Libertinus Serif")
#set text(size: 12pt)
```

```rust
fn main() {
    println!("Hello, world!");
    // This contains Typst syntax: #set page("a4")
    let x = 42;
    // More Typst syntax: #show link: underline
}
```

## multiple formats in link


- [**hello** _world_](http://example.com)
