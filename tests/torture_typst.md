# Typst Document Breaking Test Data

This document contains various types of data that could potentially break Typst documents, including special characters, syntax conflicts, and edge cases.

## Typst Special Characters and Syntax Conflicts

### Reserved Characters and Symbols
```
# @ $ \ { } [ ] ( ) < > = + - * / % ^ _ ~ ` | & ! ? : ; , . " '
```

### Hash/Heading Conflicts
```
#heading #function #variable
##double-hash ###triple-hash
# spaced hash
#no-space-hash
#123numeric
#with-dashes-and_underscores
#UPPERCASE
#миксed-SCRIPT
```

### Math Mode Breakers
```
$broken math mode
$ unclosed math
$$ double dollar
$$$ triple dollar
$ # hash in math
$ \ backslash in math
$ { } braces in math
$ @variable in math
$ #function in math
```

### String and Escape Conflicts
```
"unclosed string
'unclosed single quote
\"escaped quote in string\"
'escaped quote in string\'
"string with #variable"
"string with @reference"
"string with $math$"
\\double backslash
\n\t\r newlines and tabs
\u{1234} unicode escape
\x41 hex escape
```

### Function Call Syntax Breakers
```
#function(
#function(broken
#function(arg1, arg2,
#function(key: value,
#function(nested: #other())
#function("string", key: value
#function(..args)
#function(content)[more content]
```

### Variable and Reference Conflicts
```
@unclosed-ref
@ref-with-123numbers
@ref_with_underscores
@ref-with-special-chars!
@миксed-script-ref
@@double-at
@
@ spaced at
```

## Unicode and International Characters

### Non-Latin Scripts
```
العربية 한국어 中文 日本語 हिन्दी עברית русский ελληνικά
𝕌𝕟𝕚𝕔𝕠𝕕𝕖 𝔪𝔞𝔱𝔥 𝓈𝓎𝓂𝒷𝑜𝓁𝓈
🌍🔥💻📊🎯🚀⚡🎨🔧📱
```

### Directional Text (RTL/LTR Conflicts)
```
English עברית Arabic العربية Mixed
‏Right-to-left override
‎Left-to-right override
```

### Zero-Width and Invisible Characters
```
Zero-width space: [​]
Zero-width non-joiner: [‌]
Zero-width joiner: [‍]
Soft hyphen: [­]
```

### Combining Characters and Diacritics
```
a̋ é̂ ǒ̃ ṻ̌ ñ̈́
c̣̈̇̊̃̂̋̌̏̎̍̚
Zalgo text: Ḧ̴̰ë̷́ĺ̸̰l̴̈́ö̸̈ ̵̿W̴̋o̶̓r̸̈́l̸̊d̵̈!
```

### Mathematical Unicode
```
∑∏∫∮∯∰∱∲∳∴∵∶∷∸∹∺∻∼∽∾∿≀≁≂≃≄≅≆≇≈≉≊≋≌≍≎≏≐≑≒≓≔≕≖≗≘≙≚≛≜≝≞≟≠≡≢≣≤≥≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊂⊃⊄⊅⊆⊇⊈⊉⊊⊋⊌⊍⊎⊏⊐⊑⊒⊓⊔⊕⊖⊗⊘⊙⊚⊛⊜⊝⊞⊟⊠⊡⊢⊣⊤⊥⊦⊧⊨⊩⊪⊫⊬⊭⊮⊯⊰⊱⊲⊳⊴⊵⊶⊷⊸⊹⊺⊻⊼⊽⊾⊿⋀⋁⋂⋃⋄⋅⋆⋇⋈⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋔⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋮⋯⋰⋱⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿
```

## Special Formatting Characters

### Control Characters
```
Character	Unicode	Description
NULL	U+0000	Null character
SOH	U+0001	Start of Heading
STX	U+0002	Start of Text
ETX	U+0003	End of Text
EOT	U+0004	End of Transmission
ENQ	U+0005	Enquiry
```

### Whitespace Variations
```
Regular space: [ ]
Non-breaking space: [ ]
Em space: [ ]
En space: [ ]
Thin space: [ ]
Hair space: [ ]
Figure space: [ ]
Punctuation space: [ ]
```

### Quotation Mark Variations
```
"Regular quotes"
"Smart quotes"
'Single smart quotes'
«Guillemets»
‚German quotes'
„German quotes"
「Japanese quotes」
『Japanese quotes』
```

## Potential Parser Breaking Sequences

### Nested Markup Conflicts
```
#heading with @ref and $math$ and "strings"
@ref[#heading with more @refs]
$math with #function() calls$
"string with nested "quotes" inside"
```

### Comment-like Sequences
```
// This looks like a comment but isn't in Typst
/* This looks like a block comment */
<!-- HTML comment style -->
% LaTeX comment style
```

### Code Block Conflicts
```
```typst
This is supposed to be a code block
But contains #functions
And @references
And $math$
```

```python
# This might conflict with Typst syntax
def function():
    return "#hash @at $dollar"
```
```

### Table Breaking Characters
```
| Column 1 | Column 2 | Column 3 |
|----------|----------|----------|
| Data with | pipes | inside |
| "Quotes" | @refs | #funcs |
| $math$ | \escapes | #[content] |
```

## File Path and URL Characters

### Problematic File Paths
```
/path/with/@symbols
C:\Windows\Path\With\#Hash
./relative/path/with spaces
../parent/path/with'quotes
~/home/path/with$dollar
```

### URL Breaking Characters
```
https://example.com/path?query=value&other=@symbol
http://user:pass@domain.com/path#fragment
ftp://[::1]:8080/path/with spaces
file:///C:/Windows/System32/drivers
```

## Large Data Sequences

### Long Strings
```
ThisIsAnExtremelyLongStringWithoutAnySpacesOrBreaksThatCouldPotentiallyBreakTextWrappingOrParsingAlgorithmsInTypstDocumentsAndMightCauseMemoryOrPerformanceIssuesWhenProcessingVeryLargeDocumentsWithSimilarContent
```

### Repeated Characters
```
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

## Binary and Non-Text Data

### Binary-like Sequences
```
00000000 00000001 11111111 10101010
\x00\x01\x02\x03\xFF\xFE\xFD\xFC
```

### Base64-like Data
```
SGVsbG8gV29ybGQhIFRoaXMgaXMgYSBsb25nIGJhc2U2NCBlbmNvZGVkIHN0cmluZyB0aGF0IG1pZ2h0IGNhdXNlIGlzc3VlcyBpbiBUeXBzdCBkb2N1bWVudHMgaWYgbm90IGhhbmRsZWQgY29ycmVjdGx5
```

## Context-Dependent Breaking Cases

### Math Context Breakers
```
$1 + 2 = 3 # but this hash might break$
$\text{text with @ref inside}$
$f(x) = #function-call$
$"string in math mode"$
```

### Content Block Breakers
```
#[
  Content block with @refs
  And $math$ 
  And "strings"
  And #nested-functions()
  And // fake comments
  And /* more fake comments */
]
```

### Show Rules Breakers
```
#show heading: it => [
  #it.body with @ref and $math$
]

#show "@": [BROKEN REFERENCE]
#show "$": [BROKEN MATH]
#show "#": [BROKEN FUNCTION]
```

## Language-Specific Edge Cases

### Programming Language Keywords
```
if else for while function class def return import from
true false null undefined NaN Infinity
int float double char string bool void
public private protected static final
```

### SQL Injection Style
```
'; DROP TABLE users; --
" OR 1=1; --
UNION SELECT * FROM passwords
```

### Script Injection Style
```
<script>alert('XSS')</script>
javascript:alert('XSS')
data:text/html,<script>alert('XSS')</script>
```

## Memory and Performance Stressors

### Large Numbers
```
123456789012345678901234567890123456789012345678901234567890
999999999999999999999999999999999999999999999999999999999999
0.123456789012345678901234567890123456789012345678901234567890
```

### Scientific Notation
```
1.23e+308
-1.79e+308
1.23e-308
NaN
+Infinity
-Infinity
```

### Complex Fractions
```
$frac(frac(frac(1, 2), frac(3, 4)), frac(frac(5, 6), frac(7, 8)))$
```

## Final Stress Test

### Ultimate Breaking String
```
#function(@ref["string with #nested(@ref) and $math$ and \\escapes"], key: value, ..args)[Content with @more-refs and #more-functions($nested-math$) and "more strings" and /* fake comments */ and // more fake comments and 中文 and 🚀 and ∑∏∫ and \\\\\\]
```

### Mixed Everything
```
This document contains: #functions, @references, $math$, "strings", \escapes, @miксed-scripts, 🌍emojis, ∑math-symbols, /* comments */, // more comments, nested #[content with @refs and $x^2$], URLs like https://example.com/@user, file paths like C:\Path\With\#Hash, and various unicode: العربية 한국어 中文 ñ̈́ ḧ̃̂ë̊̌ḷ̸̍l̴̾ö̵̈ and binary \x00\xFF and long strings ThisIsVeryLongWithoutSpaces and repeated ###################### characters.
```

---

*Note: This test data is designed to stress-test Typst parsers and renderers. Use with caution in production environments.*