# Line Wrapping Fix - Complete Analysis

## Problem

Long sentences were not wrapping to the next line naturally. Instead, HWP was **compressing character spacing (자간)** to force text onto one line.

## Root Cause

The presence of `<hp:linesegarray>` with a single line segment prevented HWP from recalculating line breaks, even with correct settings.

## Investigation Process

### 1. Initial Discovery - Wrong Flags Value

First discovered that `flags` attribute was incorrect:

```xml
<!-- WRONG (bit 21 set) -->
<hp:lineseg ... flags="2490368"/>

<!-- CORRECT -->
<hp:lineseg ... flags="393216"/>
```

**Bit 21 = 1** tells HWP: "Don't recalculate line breaks, use the provided linesegarray as-is"

### 2. Fixing Flags Didn't Work

After changing all `flags="2490368"` to `flags="393216"`, wrapping still failed.

### 3. Comparison with Manually Corrected File

Compared our generated file with user's manually corrected file:

**Manually Corrected** (after fixing in Hancom Office):
- Multiple `<hp:lineseg>` elements per paragraph showing actual line breaks
- Example: textpos="0", "81", "168" (3 lines)
- flags="393216"

**Our Generated File** (with correct flags):
- Only ONE `<hp:lineseg>` per paragraph
- flags="393216" (correct value)
- **Result: Still not wrapping!**

### 4. The Real Issue

Even with correct `flags="393216"`, providing a single `<hp:lineseg>` seems to tell HWP "this paragraph has one line". HWP respects this layout hint instead of recalculating based on `breakNonLatinWord` settings.

## Solution

**Remove `<hp:linesegarray>` entirely** from regular paragraphs and lists, allowing HWP to calculate line breaks from scratch using paragraph properties.

### Files Modified

**src/hwpx-generator.ts:**

**Removed `<hp:linesegarray>` from:**
- Line 209: `generateParagraph()` - paragraphs with children (mixed content)
- Line 216: `generateParagraph()` - simple paragraphs
- Line 237: `generateList()` - list items
- Line 280: `generateTable()` - empty paragraph after table
- Line 293: `generateImage()` - image placeholders

**Kept `<hp:linesegarray>` for:**
- Line 188: `generateHeading()` - headings (working correctly)
- Line 268: Table cells (working correctly)
- Line 277: Table paragraph container (working correctly)

### Before and After

**BEFORE (not working):**
```xml
<hp:p paraPrIDRef="20" ...>
  <hp:run charPrIDRef="0">
    <hp:t>Long sentence that should wrap...</hp:t>
  </hp:run>
  <hp:linesegarray>
    <hp:lineseg textpos="0" vertpos="0" vertsize="1000"
                textheight="1000" baseline="850" spacing="600"
                horzpos="0" horzsize="42520" flags="393216"/>
  </hp:linesegarray>
</hp:p>
```

**AFTER (working!):**
```xml
<hp:p paraPrIDRef="20" ...>
  <hp:run charPrIDRef="0">
    <hp:t>Long sentence that should wrap...</hp:t>
  </hp:run>
</hp:p>
```

## Why This Works

Without `<hp:linesegarray>`, HWP has:
- No layout hints to override
- No pre-calculated line breaks to respect
- **Must calculate layout from scratch** using:
  - `breakNonLatinWord="BREAK_WORD"` (allows wrapping)
  - `lineSpacing` values (140%, 150%, 160%)
  - `horzsize` from page margins (42520 HWPUNIT)
  - Character properties (height, spacing, etc.)

## Complementary Settings

The fix works in conjunction with these paragraph property settings:

1. **breakNonLatinWord="BREAK_WORD"** - Allows Korean/CJK text to wrap naturally
2. **Character spacing = 0** - No forced spacing that could trigger compression
3. **snapToGrid="1"** - Keeps grid alignment for consistent layout
4. **lineWrap="BREAK"** - Enables line wrapping
5. **horizontal="LEFT"** - Left alignment (not JUSTIFY for most content)

## Testing

```bash
npm run build
node test-wrapping-convert.js
```

Open `test-wrapping-output.hwp` in Hancom Office to verify:
- ✅ Long sentences wrap to multiple lines
- ✅ Character spacing remains consistent
- ✅ No automatic compression
- ✅ Natural line breaks

## Impact

This fix affects how we generate:
- ✅ Normal paragraphs - NO linesegarray (HWP calculates)
- ✅ List items - NO linesegarray (HWP calculates)
- ✅ Mixed content paragraphs - NO linesegarray (HWP calculates)
- ✅ Image placeholders - NO linesegarray (HWP calculates)
- ✅ Headings - KEEP linesegarray (for precise spacing)
- ✅ Tables - KEEP linesegarray (for table layout)
- ✅ Table cells - KEEP linesegarray (for cell content)

## Key Insights

1. **Flags value matters**: `flags="2490368"` (bit 21) explicitly prevents recalculation
2. **But flags alone aren't enough**: Even with correct flags, a single lineseg acts as a layout hint
3. **Absence is better than presence**: No linesegarray = HWP must calculate from scratch
4. **Context matters**: Headings and tables benefit from explicit linesegarray, paragraphs don't
5. **Trust the paragraph properties**: HWP does proper line wrapping when given no layout hints

## Related Documentation

- `docs/HWP_Document_Data_Records.md` - HWPTAG_LIST_HEADER documentation
- `docs/HWP_CharShape_Structure.md` - Character spacing reference
- `test-wrapping-output-correct.hwpx` - Manually corrected reference file

## Credits

Issue discovered and resolved through:
1. Initial symptom: Character spacing compression on long lines
2. First attempt: Fixed flags from 2490368 to 393216 (helped but not enough)
3. Systematic comparison with manually corrected file
4. Final solution: Remove linesegarray entirely from paragraphs/lists

**Final result: ✅ Wrapping works correctly!**
