# This is a heading with <span style="color: red">inline HTML that's not closed

## Second heading with **bold and <em>mixed emphasis** but em not closed

This is a paragraph with [a link that has **bold inside](http://example.com) but the bold is not closed.

<div class="unclosed-div">
This div is never closed and contains:

### A heading inside a div
- List item with `inline code that's not closed
- Another item with *emphasis not closed

```javascript
// Code block inside an unclosed div
function broken() {
    console.log("this is in a code block");
```

More text after the code block but still inside the unclosed div.

</div>

Wait, I closed the div but opened another one:
<div id="another-div">

> This is a blockquote with **bold text
> And another line with *italic text
> 
> > Nested blockquote with `code that never closes

1. Ordered list item with <strong>HTML strong tag
2. Second item with **markdown bold and <em>HTML em** but em not closed
3. Third item with [link with **bold inside](http://test.com) and *italic not closed

<table>
<tr><td>Table cell with **bold not closed
<td>Another cell with *italic not closed
</tr>
<tr>
<td colspan="2">Cell with `code not closed and [link not closed](
</table>

More text with ***three asterisks but only two closing**

<script>
alert("This script tag in markdown");
</script>

## Final heading with <img src="test.jpg" alt="Image with **bold in alt text not closed">

Text with HTML entities that might confuse: &lt;div&gt; &amp; &quot;quotes&quot; &#39;apostrophe&#39;

<div class="nested">
<p>HTML paragraph inside div with **markdown bold not closed
<div class="inner">
More nesting with *italic not closed
</div>
</p>

Final text with mixed: **bold *italic* bold** *italic **bold*** chaos

---

Horizontal rule followed by:
<br>
Line break tag followed by **bold not closed and `code not closed

| Table | With | Headers |
|-------|------|---------|
| Cell with **bold | Cell with *italic not closed | Cell with `code not closed |
| <td>HTML td in markdown table</td> | More text | [Link not closed]( |

<style>
body { color: red; }
</style>

The end with ***unmatched emphasis marks** and *more unmatched* and **even more unmatched