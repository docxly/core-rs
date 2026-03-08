# md2hwp Improvements Summary

## Overview

This document summarizes all improvements made to the md2hwp library for better HWP output quality.

## 1. Heading Hierarchy Implementation ✅

### What Was Added
- Proper H1-H6 heading support with different font sizes
- H1: 1400 HWPUNIT (14pt)
- H2: 1300 HWPUNIT (13pt)
- H3: 1200 HWPUNIT (12pt)
- H4: 1100 HWPUNIT (11pt)
- H5/H6: 1000 HWPUNIT (10pt - normal size)

### Character Properties
Created 6 different character property sets (charPr id="0" through "10") with:
- Bold weight (700) for all headings
- Graduated font sizes
- Proper baseline and spacing calculations

### Benefits
- Clear visual hierarchy in documents
- Professional document appearance
- Proper heading structure for navigation

---

## 2. Line Spacing (줄간격) Improvements ✅

### What Was Added
Multiple paragraph properties with different line spacing:

- **paraPr id="0"**: 140% line spacing (for headings)
- **paraPr id="1"**: 150% line spacing (for lists)
- **paraPr id="20"**: 160% line spacing (for normal paragraphs)
- **paraPr id="21"**: 160% line spacing + top margin (paragraphs after headings)

### Smart Spacing
- Extra gaps (300-400 HWPUNIT) between different content types
- Context-aware paragraph selection based on previous content
- Consistent vertical rhythm throughout document

### Benefits
- Better readability
- Professional document spacing
- Improved visual separation between sections

---

## 3. Line Wrapping Fix (자간 압축 해결) ✅

### The Problem
Long sentences were not wrapping naturally. Instead, HWP was compressing character spacing (자간) to force text onto one line.

### The Solution
**Removed `<hp:linesegarray>` from regular paragraphs and lists** while keeping it for headings and tables.

#### Before (Not Working)
```xml
<hp:p paraPrIDRef="20" ...>
  <hp:run charPrIDRef="0">
    <hp:t>Long sentence...</hp:t>
  </hp:run>
  <hp:linesegarray>
    <hp:lineseg textpos="0" ... flags="393216"/>
  </hp:linesegarray>
</hp:p>
```

#### After (Working!)
```xml
<hp:p paraPrIDRef="20" ...>
  <hp:run charPrIDRef="0">
    <hp:t>Long sentence...</hp:t>
  </hp:run>
</hp:p>
```

### Why It Works
- No layout hints to override
- HWP calculates line breaks from paragraph properties
- Uses `breakNonLatinWord="BREAK_WORD"` for natural wrapping
- Character spacing stays at 0 (no compression)

### Where Applied
**Removed linesegarray from:**
- Normal paragraphs
- List items
- Mixed content paragraphs
- Image placeholders
- Empty paragraphs

**Kept linesegarray for:**
- Headings (precise spacing needed)
- Tables (layout structure needed)
- Table cells (content layout needed)

### Benefits
- ✅ Natural line wrapping for long sentences
- ✅ Consistent character spacing
- ✅ No compression artifacts
- ✅ Proper text flow for Korean and English

---

## 4. Character Spacing Settings ✅

### What Was Changed
Set character spacing to **0** for all scripts:
```xml
<hh:spacing hangul="0" latin="0" hanja="0"
            japanese="0" other="0" symbol="0" user="0"/>
```

### Why
- Prevents automatic spacing that could trigger compression
- Allows HWP to use natural character metrics
- Works with line wrapping fix

### Benefits
- Natural character appearance
- No unwanted spacing adjustments
- Consistent rendering

---

## 5. Paragraph Break Settings ✅

### What Was Configured
```xml
<hh:breakSetting
  breakLatinWord="KEEP_WORD"       <!-- Don't break English words -->
  breakNonLatinWord="BREAK_WORD"   <!-- Allow Korean/CJK wrapping -->
  widowOrphan="0"
  keepWithNext="0"
  keepLines="0"
  pageBreakBefore="0"
  lineWrap="BREAK"/>               <!-- Enable line wrapping -->
```

### Benefits
- English words stay intact
- Korean text wraps naturally
- Proper line breaking behavior

---

## 6. Bold Text Support (v1.2.4) ✅

### The Problem
Bold text (`**text**`) was not rendering correctly:
- Initially appeared at 14pt instead of 10pt
- Then had correct size but no bold weight
- Attempted solutions with `<hh:fontweight>` didn't work

### The Root Causes Discovered

#### 1. Character Property Reference Issue
HWP's `charPrIDRef` uses **position index**, not the `id` attribute:
```xml
<!-- charPrIDRef="6" looks at position 6, not id="6"! -->
<hh:charProperties itemCnt="7">
  <hh:charPr id="0" .../> <!-- Position 0 -->
  <hh:charPr id="1" .../> <!-- Position 1 -->
  <hh:charPr id="6" .../> <!-- Position 2 ← charPrIDRef="6" goes here! -->
</hh:charProperties>
```

**Solution:** Made all charPr IDs sequential (0-5)

#### 2. Wrong Bold Tag
Using `<hh:fontweight hangul="700" .../>` didn't work.

**Solution:** Analyzed user-corrected HWP file and discovered HWP requires `<hh:bold/>` tag

#### 3. Font References
Bold text needs different `fontRef` values for CJK scripts.

### The Correct Implementation

```xml
<!-- Bold text: charPr id="1" at position 1 -->
<hh:charPr id="1" height="1000" borderFillIDRef="1" ...>
  <hh:fontRef hangul="0" latin="0" hanja="1" japanese="1"
              other="1" symbol="1" user="1"/>  <!-- ← Different! -->
  ...
  <hh:bold/>  <!-- ← The key! -->
  ...
</hh:charPr>
```

### Character Property Mapping

| Position | ID | Height | Purpose |
|----------|-----|--------|---------|
| 0 | "0" | 1000 (10pt) | Normal text |
| 1 | "1" | 1000 (10pt) | **Bold text** |
| 2 | "2" | 1400 (14pt) | H1 heading |
| 3 | "3" | 1300 (13pt) | H2 heading |
| 4 | "4" | 1200 (12pt) | H3 heading |
| 5 | "5" | 1100 (11pt) | H4 heading |

### Usage in Paragraphs

```typescript
const cid = child.style?.bold ? '1' : '0';  // Position 1 = bold
return `<hp:run charPrIDRef="${cid}"><hp:t>${text}</hp:t></hp:run>`;
```

### Benefits
- ✅ Bold text renders at correct size (10pt, same as normal)
- ✅ Bold weight properly applied
- ✅ Works for English and Korean text
- ✅ No outline boxes or unwanted borders
- ✅ Headings remain properly sized and bold

### Testing
```bash
npm run build
node test-bold-clear-convert.js
# Open test-bold-clear-output.hwp in Hancom Office
```

---

## 7. Nested Lists with Indentation (v1.2.5) ✅

### The Problem
Markdown supports nested lists, but md2hwp wasn't handling them correctly:
- Nested list items weren't being parsed
- No visual indentation to show hierarchy
- Bold text in list items wasn't working
- Mixed content (bold + normal text) in same list item not supported

### Example Input
```markdown
- **총 예산**: 35,000,000원
- **주요 항목**:
  - 해외 연사 항공료 및 숙박: ~10,440,000원
  - 연사비: 3,400,000원
```

### The Root Causes Discovered

#### 1. Missing Recursive Parsing
The `marked.js` library puts nested lists in the parent item's `tokens` array:
```javascript
{
  type: 'list_item',
  text: '주요 항목',
  tokens: [
    { type: 'text', text: '주요 항목' },
    { type: 'list', items: [...] }  // ← Nested list here!
  ]
}
```

**Solution:** Recursively parse the `item.tokens` array in `parseList()`

#### 2. Bold Text Not Parsed in Lists
The `parseList()` method was using `extractText()` which strips formatting.

**Solution:** Use `parseInlineTokens()` to preserve bold/italic formatting

#### 3. No Visual Indentation
Nested lists had no indentation to show hierarchy levels.

**Solution:** Add `level` parameter and generate inline paragraph properties with left margin

### The Implementation

#### Parser Changes (markdown-parser.ts)
```typescript
private parseList(token: Tokens.List): HwpContent {
  const children: HwpContent[] = [];

  for (const item of token.items) {
    // Check for nested tokens (including nested lists)
    if ('tokens' in item && Array.isArray(item.tokens)) {
      for (const subToken of item.tokens) {
        if (subToken.type === 'text') {
          // Parse inline elements (bold, italic, etc.)
          const inlineElements = this.parseInlineTokens(subToken.text);
          // ...
        } else {
          // Recursively handle nested lists
          const parsed = this.tokenToContent(subToken);
          if (parsed) children.push(parsed);
        }
      }
    } else {
      // Handle inline formatting in simple list items
      const inlineElements = this.parseInlineTokens(item.text);
      // ...
    }
  }

  return { type: 'list', children };
}
```

#### Generator Changes (hwpx-generator.ts)
```typescript
private generateList(
  content: HwpContent,
  vertPos: number,
  isFirst: boolean,
  previousType: string | null,
  level: number = 0  // ← New parameter for nesting level
): { xml: string; nextVertPos: number } {
  // Calculate indentation (800 HWPUNIT per level ≈ 8mm)
  const indent = level * 800;

  for (const item of content.children) {
    // Handle nested lists recursively with level + 1
    if (item.type === 'list') {
      const nestedResult = this.generateList(
        item, currentVertPos, first, 'list', level + 1
      );
      // ...
    }

    // Handle bold text in list items
    if (item.children && item.children.length > 0) {
      const textRuns = item.children.map(child => {
        const t = this.escapeXml(child.content || '');
        const cid = child.style?.bold ? '1' : '0';
        return `<hp:run charPrIDRef="${cid}"><hp:t>${t}</hp:t></hp:run>`;
      }).join('');
      runs = `<hp:run charPrIDRef="0"><hp:t>• </hp:t></hp:run>${textRuns}`;
    }

    // Generate inline paragraph properties with indentation
    if (indent > 0) {
      paraPrXml = `<hp:paraPr>
        <hp:margin>
          <hc:left value="${indent}" unit="HWPUNIT"/>
        </hp:margin>
        <hp:lineSpacing type="PERCENT" value="150" unit="HWPUNIT"/>
      </hp:paraPr>`;
    }
  }
}
```

### Indentation Calculation

| Level | Indent (HWPUNIT) | Approximate (mm) |
|-------|------------------|------------------|
| 0 | 0 | 0 |
| 1 | 800 | ~8mm |
| 2 | 1600 | ~16mm |
| 3 | 2400 | ~24mm |

Each nesting level adds **800 HWPUNIT** of left margin.

### Benefits
- ✅ **Nested list parsing** - recursively handles multi-level structures
- ✅ **Visual indentation** - clear hierarchy with proper margins
- ✅ **Bold text in lists** - supports `**label**: value` pattern
- ✅ **Mixed content** - handles bold + normal text in same item
- ✅ **Arbitrary depth** - supports any level of nesting
- ✅ **Natural appearance** - indentation matches typical document formatting

### Testing
```bash
npm run build
node test-nested-list-convert.js
# Open test-nested-list-output.hwp in Hancom Office
```

Test files:
- `test-nested-list.md` - Markdown with nested lists and bold text
- `test-nested-list-convert.js` - Conversion script
- `test-nested-list-output.hwp` - Generated HWP file

---

## Testing

All improvements have been tested with:
```bash
npm run build
node test-headings-convert.js      # Test heading hierarchy
node test-spacing-convert.js       # Test line spacing
node test-wrapping-convert.js      # Test line wrapping
node test-bold-clear-convert.js    # Test bold text
node test-nested-list-convert.js   # Test nested lists with indentation
```

Test files included:
- `test-headings.md` / `test-headings-output.hwp`
- `test-spacing.md` / `test-spacing-output.hwp`
- `test-wrapping.md` / `test-wrapping-output.hwp`
- `test-bold-clear.md` / `test-bold-clear-output.hwp`
- `test-nested-list.md` / `test-nested-list-output.hwp`

---

## Documentation

Complete documentation available in:
- `docs/HWP_Document_Data_Records.md` - HWPTAG reference
- `docs/HWP_CharShape_Structure.md` - Character properties
- `docs/Line_Wrapping_Fix.md` - Detailed wrapping fix analysis
- `docs/Bold_Text_Implementation.md` - Bold text implementation journey

---

## Summary

### Before
- Flat heading structure (all same size)
- Tight line spacing
- Line wrapping issues (character compression)
- Basic paragraph formatting

### After
- ✅ Professional heading hierarchy (H1-H6)
- ✅ Comfortable line spacing (140%-160%)
- ✅ Natural line wrapping (no compression)
- ✅ Smart spacing between content types
- ✅ Context-aware paragraph selection
- ✅ Proper character spacing settings
- ✅ **Working bold text support** (`**text**`)

### Result
**Professional, readable HWP documents with natural text flow and proper text formatting!** 🎉
