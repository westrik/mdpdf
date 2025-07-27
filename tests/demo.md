---
page_size: letter
---

# Markdown to PDF Test 😎 

This is a demonstration of [`mdpdf`](https://github.com/westrik/mdpdf).

## Markdown Elements

### Links
- [Link to Google](https://www.google.com)

<!-- section links are not yet supported -->
<!-- - [Relative link to PDF section](#math) -->
<!-- - [Relative link to section that does not exist](#does-not-exist) -->

### Inline Code
You can use inline code like `console.log("Hello World")` or `const x = 42` or ``` `hello` ``` within your text.

### Ordered Lists
1. First item in ordered list
2. Second item with **bold text**
3. Third item with *italic text*
4. Fourth item with `inline code`
5. Fifth item with **`bold inline code`**
5. 6<sup>th</sup> item with <sub>sub</sub>script and <sup>super</sup>script

### Task Lists
- [ ] incomplete task
- [x] complete task

### Blockquotes

> This is a blockquote. It can contain multiple lines.
> 
> You can have **bold** and *italic* text in blockquotes.
> 
> You can have `inline code` too.
>
> > Nested blockquotes work as well.
> >
> > > Second level of nesting.
> > > > Third level of nesting.

### Horizontal Rules

Above the rule.

---

Below the rule.

### Strikethrough Text
This text has ~~strikethrough~~ formatting applied to it.

### GitHub blockquote tags

> [!NOTE]
> note

> [!TIP]
> tip

> [!IMPORTANT]
> important

> [!WARNING]
> warning

> [!CAUTION]
> caution

## Enhanced Code Blocks

### JavaScript with Syntax Highlighting
```javascript
// Enhanced JavaScript example
class Calculator {
    constructor() {
        this.history = [];
    }
    
    add(a, b) {
        const result = a + b;
        this.history.push(`${a} + ${b} = ${result}`);
        return result;
    }
    
    getHistory() {
        return this.history;
    }
}

const calc = new Calculator();
console.log(calc.add(5, 3)); // 8
console.log(calc.getHistory());
```

### Rust with Syntax Highlighting
```rust
// Enhanced Rust example
use std::collections::HashMap;

#[derive(Debug)]
struct Cache<K, V> {
    data: HashMap<K, V>,
    max_size: usize,
}

impl<K, V> Cache<K, V> 
where 
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn new(max_size: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_size,
        }
    }
    
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.data.len() >= self.max_size {
            // Remove oldest entry (simple implementation)
            if let Some(old_key) = self.data.keys().next().cloned() {
                self.data.remove(&old_key);
            }
        }
        self.data.insert(key, value)
    }
    
    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }
}

fn main() {
    let mut cache = Cache::new(3);
    cache.insert("a", 1);
    cache.insert("b", 2);
    cache.insert("c", 3);
    cache.insert("d", 4); // This will evict "a"
    
    println!("Cache: {:?}", cache);
}
```

### Python with Syntax Highlighting
```python
# Enhanced Python example
from typing import List, Optional, Dict, Any
from dataclasses import dataclass
from datetime import datetime
import json

@dataclass
class User:
    id: int
    name: str
    email: str
    created_at: datetime
    preferences: Dict[str, Any]
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            'id': self.id,
            'name': self.name,
            'email': self.email,
            'created_at': self.created_at.isoformat(),
            'preferences': self.preferences
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'User':
        return cls(
            id=data['id'],
            name=data['name'],
            email=data['email'],
            created_at=datetime.fromisoformat(data['created_at']),
            preferences=data['preferences']
        )

class UserManager:
    def __init__(self):
        self.users: List[User] = []
    
    def add_user(self, user: User) -> None:
        self.users.append(user)
    
    def find_by_email(self, email: str) -> Optional[User]:
        return next((u for u in self.users if u.email == email), None)
    
    def export_to_json(self, filename: str) -> None:
        with open(filename, 'w') as f:
            json.dump([u.to_dict() for u in self.users], f, indent=2)

# Usage example
manager = UserManager()
user = User(
    id=1,
    name="John Doe",
    email="john@example.com",
    created_at=datetime.now(),
    preferences={"theme": "dark", "notifications": True}
)
manager.add_user(user)
print(f"User: {user}")
```

## Mixed Content Examples

### Lists with Various Elements
- Regular list item
- Item with **bold**ed text
- Item with *italic*ized text
- Item with `inline code`blocks
- Item with a [link to GitHub](https://github.com)[with URLs displayed]
- Item with a [**bold link and _italic_ link to GitHub**](https://github.com)
- En--dash & em---dash (`--` becomes --, `---` becomes ---)

### Nested Lists
1. First level
2. Another first level that wraps all the way around to the next line because it is very long.
    1. Second level
        1. Third level
            1. Fourth level
            5. Another fourth level
                1. Fifth level
                    1. Sixth level
    2. Back to second level
        1. Another third level
            - Unordered fourth level
    3. Back to second level
3. Back to first level
    - Unordered second level
    - Another unordered second level. This item also wraps around to the next line because it is very long.

### Code with Comments
```javascript
// This is a comment
const greeting = "Hello, World!"; // Inline comment
console.log(greeting);

/* 
   Multi-line comment
   explaining complex logic
*/
function complexFunction() {
    // TODO: Implement this function
    return null;
}
```

## Image Support

### Data URL Image


PNG: ![Markdown logo](data:image/png;base64,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)

SVG: ![Markdown logo](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyMDgiIGhlaWdodD0iMTI4IiB2aWV3Qm94PSIwIDAgMjA4IDEyOCI+PHJlY3Qgd2lkdGg9IjE5OCIgaGVpZ2h0PSIxMTgiIHg9IjUiIHk9IjUiIHJ5PSIxMCIgc3Ryb2tlPSIjMDAwIiBzdHJva2Utd2lkdGg9IjEwIiBmaWxsPSJub25lIi8+PHBhdGggZD0iTTMwIDk4VjMwaDIwbDIwIDI1IDIwLTI1aDIwdjY4SDkwVjU5TDcwIDg0IDUwIDU5djM5em0xMjUgMGwtMzAtMzNoMjBWMzBoMjB2MzVoMjB6Ii8+PC9zdmc+)


### External Image

SVG: ![Markdown logo](https://upload.wikimedia.org/wikipedia/commons/4/48/Markdown-mark.svg)

JPG (also is a link): [![Example Image](https://picsum.photos/200)](https://google.com)

## Emoji

😀 😂 🥰 😊 😍 🤣 ❤️ 😭 😘 👍 

😅 😁 🔥 ✨ 🥺 😩 🙏 ✅ 💕 😌 

🎉 💜 😔 💪 🙄 😎 ✌️ 💫 😤 💖 

🤦 😉 🎂 💗 🤷 😏 👀 😳 🌹 🔫

😒 💙 😢 🤔 ☺️ 😆 🌟 😄 💝 💀 

🖤 😃 💯 🥳 ⭐ 👏 ✍️ 🎈 💓 🤗 

😡 👉 💛 💚 😋 😑 🌸 🤪 ✊ 🎊 

💥 ✝️ 🙂 😕 💭 🤨 🌺 ♥️ 🤝 🌈 

🙈 💞 ⚡ 🌙 ☀️ 🎵 ☹️ 👌 🎶 ☝️ 

💐 🌷 🦋 🎀 ⚘ 🌞 🌊 🍀 🌿 🌱


### Math

Inline math: $\sqrt{3x-1}+(1+x)^2$

Block math:

$$\left( \sum_{k=1}^n a_k b_k \right)^2 \leq \left( \sum_{k=1}^n a_k^2 \right) \left( \sum_{k=1}^n b_k^2 \right)$$

Math code blocks:

```math
\left( \sum_{k=1}^n a_k b_k \right)^2 \leq \left( \sum_{k=1}^n a_k^2 \right) \left( \sum_{k=1}^n b_k^2 \right)
```


### Tables

| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Cell 1A  | Cell 1B  | Cell 1C  |
| Cell 2A  | Cell 2B  | Cell 2C  |
| Cell 3A  | Cell 3B  | Cell 3C  |

| Left-aligned | Center-aligned | Right-aligned |
|:-------------|:-------------:|-------------:|
| Left         | Center        | Right        |
| Text         | **Bold**          | _Italic_         |



### Definition list

Term 1
:   Definition 1

Term 2
:   Definition 2



### Footnotes

Footnote referenced [^1].

[^1]: footnote defined

-----


## RTL

זוהי דוגמה לטקסט מימין לשמאל

هذا بعض الأمثلة على النص من اليمين إلى اليسار

مرحبا بالعالم hello world שלום עולם

مرحبا بالعالم 

 שלום עולם

[דוגמה דוט קום](http://example.com)

![דוגמה דוט קום](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyMDgiIGhlaWdodD0iMTI4IiB2aWV3Qm94PSIwIDAgMjA4IDEyOCI+PHJlY3Qgd2lkdGg9IjE5OCIgaGVpZ2h0PSIxMTgiIHg9IjUiIHk9IjUiIHJ5PSIxMCIgc3Ryb2tlPSIjMDAwIiBzdHJva2Utd2lkdGg9IjEwIiBmaWxsPSJub25lIi8+PHBhdGggZD0iTTMwIDk4VjMwaDIwbDIwIDI1IDIwLTI1aDIwdjY4SDkwVjU5TDcwIDg0IDUwIDU5djM5em0xMjUgMGwtMzAtMzNoMjBWMzBoMjB2MzVoMjB6Ii8+PC9zdmc+)


<!-- 

## CJK

Fonts not included for CJK. Adding them would be easy but would significantly increase binary size.

你好世界

こんにちは世界 

안녕하세요 세상
-->

-----

# HTML Rendering

This document tests various HTML elements and their conversion to Typst.

## Basic Formatting

<p>This is a <strong>bold paragraph</strong> with <em>italic text</em> and <u>underlined content</u>.</p>

<p>Here's some <strike>strikethrough text</strike> and <s>another strikethrough</s>.</p>

## Headings

<h1>HTML Heading 1</h1>
<h2>HTML Heading 2</h2>
<h3>HTML Heading 3</h3>
<h4>HTML Heading 4</h4>
<h5>HTML Heading 5</h5>
<h6>HTML Heading 6</h6>

## Links and Images

<p>Here's a <a href="https://example.com">simple link</a> and a <a href="https://example.com" target="_blank">link with target</a>.</p>

<p>And an image: <a href="https://google.com"><img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAANAAAACACAMAAABN9BexAAABRFBMVEUAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD///8NAl4NAAAAanRSTlMAAQIDBQcICQsMDg8SGBsfJSo2ODk6Ozw9P0FCREpQUVJTVFpfYWZnbHN0dXZ3eHl7gIKDhYaIiYqLjJiZnqCkp6ipr7CxsrO0vr/BwsPGx8nR1dfY2eDh4uPk5+jp7PHy8/T19vj5+v3+PVg6RwAAAAFiS0dEa1JlpZgAAAPiSURBVHja7d1rWxJBGAbgB3ABK+mgIZqVaUHhqXMQlNnJU6FFWYJWaB6Y//8D+iDsidnd2WXXeOea9xN+4IL7kn3mmUEB6E4iV96sHzByc1DfLI8nYJ/Uwj4jPPvzKatnZpcRn90ZEyf2qM3IT/t5vOuJv2VSzEpX9IxJMk86109bFlB7GgCSP5k000gBWGQSzRyQ2JcJtBdHzvTj+4k0yE164qOJkMVL44dFEJ0lw1DGhvH7AdlZ1RHr+K7fnqALmtQR39DSb6fpgoZ1RAvGyw+Ex6RQIAVSIAVSIAVSIAVSIAVSoP8D4h1E5MUeo8C7r6/n5uNufYFOciKesSMyINbMeD+tkQajA2JbmtcDDH1ilEDstdcDVBktkFcwFBg1kHsw8ANhoEGuweAQCIMNcgkGp0AYcJBzMFQZTZBTMBQYVRA/GJwDYeBB3GBwCYTBB3GCwS0QCIB6g6HKaIPswVBg1EHWYHAPBBIgSzB4BAINkCkYvAKBCMgIhiqTA9QNhgKTBXQWDN6BQAbEmhmhQKADYluaSCAQArFqlckFEh4FUiAFYkwwACpkQGIRXdPIgIQW0WYGdEAY9aw5JzlQAuGuSGGlBPLaKlRADeQeDDVN8JmVgy8ApXBBrsHQ2ZYLgGKvgnqWYyGDXIKhe3Ai8tpJvAvmWRtC2CDnYMj7uRhSn4N4ttMIH+QUDBV/V/fFL/49Xy8hChA/GGqaz7jK7Pj1NC4jEhA3GEzndKL5e8Xn/8f8uoqIQJxgMJ+kCi8o11p+PH+ziAzUGwz5QCvk1LG45+QmIgTZg6EScMmfPRX1tO8hUpA1GGpa0A5zXxRURLQgSzDY3tHzVcoES1AJUYNMwWB/z9UXSKwE2QpPJCAjGPJ91WaREmQvPNGAusFQ6XMf4F2CttM4F9BZMNS0fjc2XiWot/BEBMJIg/sWv++dmnsJ4hSeqEAYPeL9EYb/radbCeIVnshAyOfD2UuPHTp5DsdwnqDQDgecShC/8BAAOZQgh8JDAcQvQUXQBfFKUAmUQb0lyLHw0AD1lCDnwkMEZCtB22lQB1lKkFvhIQMylSDXwkMHpJcg98JDCNQ5CeKe8NAEYerYu/CQAmH21LPw0AKhWET4ICqjQAqkQAqkQAqkQAqkQAqkQOcA+qPfHKbruaAjfsv3oZMb+u0PdEFrOmLdfDa+RNXz0DC8wLhpx786SfA6Gr6xZiJcl+zDj5txYF4m0AMAyR/yeHZTAHBHno94v312WT2VBfS4kxPxFTk8b/QvfogtyPVFFgCmySfDzi3r8pSc2yO9/hSTPStuPFvaqLfoWVr19VLWeLX9A7BB7+nmPT+tAAAAAElFTkSuQmCC" alt="Test image" width="200" height="100"></a></p>

## Code Elements

<p>Inline code: <code>println!("Hello, world!");</code></p>

<pre><code>fn main() {
    println!("This is a code block");
    let x = 42;
    println!("x = {}", x);
}</code></pre>

## Lists

<h3>Unordered List</h3>
<ul>
<li>First item with <strong>bold text</strong></li>
<li>Second item with <em>italic text</em></li>
<li>Third item with <a href="https://example.com">a link</a></li>
</ul>

<h3>Ordered List</h3>
<ol>
<li>First ordered item</li>
<li>Second ordered item</li>
<li>Third ordered item</li>
</ol>

## Blockquotes

<blockquote>This is a simple blockquote with some text.</blockquote>

<blockquote cite="https://example.com">This is a blockquote with a citation attribute.</blockquote>
<blockquote cite="#asdf{}">This is a blockquote with a citation attribute.</blockquote>

## Tables

<table>
<tr>
<th>Header 1</th>
<th>Header 2</th>
<th>Header 3</th>
</tr>
<tr>
<td>Cell 1</td>
<td>Cell 2</td>
<td>Cell 3</td>
</tr>
<tr>
<td>Cell 4</td>
<td>Cell 5</td>
<td>Cell 6</td>
</tr>
</table>

## Definition Lists

<dl>
<dt>Term 1</dt>
<dd>Definition for term 1</dd>
<dt>Term 2</dt>
<dd>Definition for term 2 with <strong>bold text</strong></dd>
</dl>

## Subscript and Superscript

<p>Here's some <sub>subscript text</sub> and <sup>superscript text</sup>.</p>

## Line Breaks and Spacing

<!-- TODO: <br>s aren't working here? -->
<p>First line<br>Second line</p>
<p>Third line<br/>Fourth line</p>
<p>Fifth line<br />Sixth line</p>

## Horizontal Rules

<hr>
<hr/>
<hr />

## HTML Entities

<p>HTML entities: &amp; &lt; &gt; &quot; &apos; &copy; &reg; &trade;</p>

## Nested Structures

<div>
<p>This is a paragraph inside a div with <strong>bold <em>and italic</em> text</strong>.</p>
<ul>
<li>List item with <a href="https://example.com">link</a></li>
</ul>
</div>

## Elements That Should Be Stripped

<video src="video.mp4">Video content</video>
<audio src="audio.mp3">Audio content</audio>
<canvas>Canvas content</canvas>
<script>alert('hello');</script>
<style>body { color: red; }</style>

## Malformed HTML (Should Handle Gracefully)

<p>Unclosed paragraph
<strong>Unclosed bold
<em>Nested unclosed italic</em>
<a href="https://example.com">Unclosed link

## Special Characters in Attributes

<a href="https://example.com/path?param=value&other=123">Link with query params</a>
<img src="image.jpg" alt="Image with &quot;quotes&quot; and &amp; symbols">

## Mixed Content

<p>This paragraph has <strong>bold</strong>ed text, <em>italic</em>ized text, <code>code</code>-blocks, and <a href="https://example.com">links</a> all mixed together.</p>

<blockquote>
<p>This is a blockquote with a paragraph inside it.</p>
<ul>
<li>And a list item</li>
</ul>
</blockquote>

This should have quotes:
<q>I am a quote</q>

## Comments

<!-- This is an HTML comment that should be stripped -->

<p>Text after comment</p>

## Centered Text

<center>
This is centered
<img src="https://upload.wikimedia.org/wikipedia/commons/4/48/Markdown-mark.svg">
</center>

## Complex Nested Structure

<div>
<h2>Section Title</h2>
<p>Introduction paragraph with <em>emphasis</em>.</p>
<blockquote>
<p>Quote with <strong>bold text</strong> and <a href="https://example.com">a link</a>.</p>
</blockquote>
<table>
<tr><th>Header</th><th>Another Header</th></tr>
<tr><td>Data</td><td>More data</td></tr>
</table>
</div>


----

## Correctly escapes Typst syntax

@reference

[@reference](http://example.com)

should be a plus + right there

#set

<label>

= this should not be a heading

this should not = be a heading

+ item

/ Term: description

there should be backslash right here: \

~ there should be a tilde at the start of this line

#rect(width: 1cm)

\#ad

/* block comment */

// line comment

