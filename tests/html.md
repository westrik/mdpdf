# HTML Parsing Test

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

<p>And an image: <img src="data:image/png;base64,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" alt="Test image" width="200" height="100"></p>

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

<p>This paragraph has <strong>bold</strong>, <em>italic</em>, <code>code</code>, and <a href="https://example.com">links</a> all mixed together.</p>

<blockquote>
<p>This is a blockquote with a paragraph inside it.</p>
<ul>
<li>And a list item</li>
</ul>
</blockquote>

## Comments

<!-- This is an HTML comment that should be stripped -->

<p>Text after comment</p>

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
