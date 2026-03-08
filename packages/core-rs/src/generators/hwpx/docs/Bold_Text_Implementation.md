# Bold Text Implementation in md2hwp

## Overview

Implementing proper bold text support in HWP format required solving multiple technical challenges related to how HWP's HWPML format handles character properties and text styling.

## The Journey

### Initial Attempt (v1.2.1-v1.2.2)

**Approach:** Used `<hh:fontweight>` tag with value 700
- Created charPr with `fontweight="700"`
- Used non-sequential charPr IDs (0, 1, 6, 7, 8, 9, 10)
- **Result:** Bold text appeared at 14pt (wrong size) and wasn't actually bold

**Root Cause Discovered:**
HWP's `charPrIDRef` attribute references the **position index** in the charProperties array, NOT the `id` attribute value!

Example:
```xml
<hh:charProperties itemCnt="7">
  <hh:charPr id="0" height="1000" .../> <!-- Position 0 -->
  <hh:charPr id="1" height="2000" .../> <!-- Position 1 -->
  <hh:charPr id="6" height="1000" .../> <!-- Position 2 -->
  ...
</hh:charProperties>
```

When using `charPrIDRef="6"`, HWP looks at **position 6**, not id="6"!

### Second Attempt (v1.2.3)

**Fix:** Made charPr IDs sequential (0-5)
- Position 0: id="0" → 10pt normal
- Position 1: id="1" → 10pt bold
- Position 2: id="2" → 14pt bold (H1)
- etc.

**Result:** Size was now correct (10pt), but text still wasn't bold!

**Problem:** Using `<hh:fontweight>` tag didn't work. HWP was ignoring it.

### Final Solution (v1.2.4)

**Discovery:** Analyzed user-corrected HWP file to find the correct format.

**Key Findings:**
1. Use `<hh:bold/>` tag instead of `<hh:fontweight>`
2. Different `fontRef` values for bold text
3. Keep `borderFillIDRef="1"` (no borders)

## Correct Implementation

### Character Property for Bold Text

```xml
<hh:charPr id="1" height="1000" textColor="#000000" shadeColor="none"
           useFontSpace="0" useKerning="0" symMark="NONE" borderFillIDRef="1">
  <hh:fontRef hangul="0" latin="0" hanja="1" japanese="1"
              other="1" symbol="1" user="1"/>
  <hh:ratio hangul="100" latin="100" hanja="100" japanese="100"
            other="100" symbol="100" user="100"/>
  <hh:spacing hangul="0" latin="0" hanja="0" japanese="0"
              other="0" symbol="0" user="0"/>
  <hh:relSz hangul="100" latin="100" hanja="100" japanese="100"
            other="100" symbol="100" user="100"/>
  <hh:offset hangul="0" latin="0" hanja="0" japanese="0"
             other="0" symbol="0" user="0"/>
  <hh:bold/>  <!-- ← The key element! -->
  <hh:underline type="NONE" shape="SOLID" color="#000000"/>
  <hh:strikeout shape="NONE" color="#000000"/>
  <hh:outline type="NONE"/>
  <hh:shadow type="NONE" color="#C0C0C0" offsetX="10" offsetY="10"/>
</hh:charPr>
```

### Key Differences from Normal Text

| Aspect | Normal Text (id="0") | Bold Text (id="1") |
|--------|---------------------|-------------------|
| Height | 1000 (10pt) | 1000 (10pt) - same! |
| Bold Tag | None | `<hh:bold/>` |
| fontRef hanja | "0" | "1" |
| fontRef japanese | "0" | "1" |
| fontRef other | "0" | "1" |
| fontRef symbol | "0" | "1" |
| fontRef user | "0" | "1" |
| borderFillIDRef | "1" | "1" - same! |

### Why fontRef Values Matter

The different `fontRef` values for non-Latin scripts (hanja, japanese, etc.) tell HWP to use alternate font faces for those character sets when rendering bold text. This is important for CJK (Chinese-Japanese-Korean) text rendering.

### Common Pitfalls to Avoid

1. **Don't use `borderFillIDRef="2"`** for bold text
   - borderFill id="2" has visible borders (used for tables)
   - This causes outline boxes around bold text
   - Always use borderFillIDRef="1" (no borders)

2. **Don't use `<hh:fontweight>` tag**
   - HWP ignores this tag for bold rendering
   - Use `<hh:bold/>` instead

3. **Don't skip the fontRef values**
   - Bold text needs fontRef values of "1" for non-Latin scripts
   - This ensures proper rendering of Korean/CJK bold text

4. **Ensure sequential charPr IDs**
   - charPrIDRef uses position index, not id value
   - IDs must be 0, 1, 2, 3... without gaps

## Implementation Code

### Helper Method

```typescript
private generateCharPr(id: string, height: string, bold: boolean = false): string {
  const baseAttrs = `id="${id}" height="${height}" textColor="#000000" shadeColor="none" useFontSpace="0" useKerning="0" symMark="NONE" borderFillIDRef="1"`;

  // Different fontRef for bold text (CJK support)
  const fontRefVals = bold
    ? 'hangul="0" latin="0" hanja="1" japanese="1" other="1" symbol="1" user="1"'
    : 'hangul="0" latin="0" hanja="0" japanese="0" other="0" symbol="0" user="0"';

  // Use <hh:bold/> tag, NOT fontweight
  const boldTag = bold ? `<hh:bold/>` : '';

  return `<hh:charPr ${baseAttrs}>
    <hh:fontRef ${fontRefVals}/>
    <hh:ratio hangul="100" latin="100" hanja="100" japanese="100" other="100" symbol="100" user="100"/>
    <hh:spacing hangul="0" latin="0" hanja="0" japanese="0" other="0" symbol="0" user="0"/>
    <hh:relSz hangul="100" latin="100" hanja="100" japanese="100" other="100" symbol="100" user="100"/>
    <hh:offset hangul="0" latin="0" hanja="0" japanese="0" other="0" symbol="0" user="0"/>
    ${boldTag}
    <hh:underline type="NONE" shape="SOLID" color="#000000"/>
    <hh:strikeout shape="NONE" color="#000000"/>
    <hh:outline type="NONE"/>
    <hh:shadow type="NONE" color="#C0C0C0" offsetX="10" offsetY="10"/>
  </hh:charPr>`;
}
```

### Character Properties Definition

```typescript
// Sequential IDs: 0-5
const charProperties = `<hh:charProperties itemCnt="6">
  ${this.generateCharPr('0', '1000', false)}  <!-- Normal 10pt -->
  ${this.generateCharPr('1', '1000', true)}   <!-- Bold 10pt -->
  ${this.generateCharPr('2', '1400', true)}   <!-- H1: 14pt bold -->
  ${this.generateCharPr('3', '1300', true)}   <!-- H2: 13pt bold -->
  ${this.generateCharPr('4', '1200', true)}   <!-- H3: 12pt bold -->
  ${this.generateCharPr('5', '1100', true)}   <!-- H4: 11pt bold -->
</hh:charProperties>`;
```

### Using Bold in Paragraphs

```typescript
const runs = content.children.map(child => {
  const t = this.escapeXml(child.content || '');
  const cid = child.style?.bold ? '1' : '0';  // Position 1 = bold
  return `<hp:run charPrIDRef="${cid}"><hp:t>${t}</hp:t></hp:run>`;
}).join('');
```

## Testing

To verify bold text works correctly:

1. **Create test markdown:**
   ```markdown
   # Heading (should be 14pt bold)

   Normal text and **bold text** (same size, different weight).

   Korean: **진하게** should also be bold.
   ```

2. **Generate HWP file**

3. **Open in Hancom Office and verify:**
   - Bold text is 10pt (same size as normal text)
   - Bold text has heavier weight
   - No outline boxes around bold text
   - Heading is 14pt bold
   - Korean bold text works

## References

- HWP HWPML Format Specification
- Hancom Office 2020+ compatibility testing
- User-corrected sample files for format validation

## Lessons Learned

1. **Always verify assumptions about XML attributes**
   - `charPrIDRef` seemed like it would reference `id` attribute
   - Actually references position index - subtle but critical difference

2. **Test with actual application**
   - XML can be syntactically correct but semantically wrong
   - Only Hancom Office can verify true compatibility

3. **Learn from working examples**
   - Analyzing user-corrected file revealed `<hh:bold/>` tag
   - Would have been very difficult to discover through documentation alone

4. **Document the journey**
   - Future maintainers will benefit from understanding why code is structured this way
   - Prevents regression by documenting pitfalls
