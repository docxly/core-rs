# HWP Character Shape (글자 모양) Structure Reference

## 4.2.6. HWPTAG_CHAR_SHAPE (Table 33)

This document describes the binary structure of character shape data in HWP format.

Tag ID: `HWPTAG_CHAR_SHAPE`
Total Size: **72 bytes**

## Data Structure

| 자료형 (Data Type) | 길이(바이트) | 설명 (Description) |
|-------------------|------------|-------------------|
| WORD array[7] | 14 | 언어별 글꼴 ID(FaceID) 참조 값(표 34 참조)<br>Font ID per language reference (see Table 34) |
| UINT8 array[7] | 7 | 언어별 장평, 50%~200%(표 34 참조)<br>Character width per language, 50%~200% |
| INT8 array[7] | 7 | 언어별 자간, -50%~50%(표 34 참조)<br>**Character spacing per language, -50%~50%** ⭐ |
| UINT8 array[7] | 7 | 언어별 상대 크기, 10%~250%(표 34 참조)<br>Relative size per language, 10%~250% |
| INT8 array[7] | 7 | 언어별 글자 위치, -100%~100%(표 34 참조)<br>Character position per language, -100%~100% |
| INT32 | 4 | 기준 크기, 0pt~4096pt<br>**Base size, 0pt~4096pt** ⭐ |
| UINT32 | 4 | 속성(표 30 참조)<br>Attributes (see Table 30) |
| INT8 | 1 | 그림자 간격, -100%~100%<br>Shadow spacing X, -100%~100% |
| INT8 | 1 | 그림자 간격, -100%~100%<br>Shadow spacing Y, -100%~100% |
| COLORREF | 4 | 글자 색<br>Text color |
| COLORREF | 4 | 밑줄 색<br>Underline color |
| COLORREF | 4 | 음영 색<br>Shade color |
| COLORREF | 4 | 그림자 색<br>Shadow color |
| UINT16 | 2 | 글자 테두리/배경 ID(CharShapeBorderFill ID) 참조 값 (5.0.2.1 이상)<br>Border/Fill ID reference (version 5.0.2.1+) |
| COLORREF | 4 | 취소선 색 (5.0.3.0 이상)<br>Strikethrough color (version 5.0.3.0+) |
| **전체 길이** | **72** | **Total length** |

## Language Array Index (표 34 참조)

The 7-element arrays correspond to different language types:
1. HANGUL (한글)
2. LATIN (영문)
3. HANJA (한자)
4. JAPANESE (일문)
5. OTHER (기타)
6. SYMBOL (기호)
7. USER (사용자)

## Key Properties for Line Spacing

### 1. 자간 (Character Spacing) - INT8 array[7]
- Range: **-50% to 50%**
- **Directly affects horizontal spacing between characters**
- Currently NOT implemented in md2hwp
- Default in our implementation: 0 (no extra spacing)

### 2. 기준 크기 (Base Size) - INT32
- Range: **0pt to 4096pt**
- **This is the character height we're using!**
- In HWPUNIT: 1000 = 10pt (approximately)
- Our current implementation:
  - H1: height="1400" (14pt)
  - H2: height="1300" (13pt)
  - H3: height="1200" (12pt)
  - H4: height="1100" (11pt)
  - H5/H6: height="1000" (10pt)
  - Normal: height="1000" (10pt)

### 3. 상대 크기 (Relative Size) - UINT8 array[7]
- Range: **10% to 250%**
- Can be used to make certain languages larger/smaller
- Currently NOT implemented (defaults to 100%)

## Current md2hwp Implementation

In `src/hwpx-generator.ts`, we define character properties in XML format:

```xml
<hh:charPr id="0" height="1000" ...>
  <hh:spacing hangul="0" latin="0" hanja="0"
              japanese="0" other="0" symbol="0" user="0"/>
  <hh:relSz hangul="100" latin="100" hanja="100"
            japanese="100" other="100" symbol="100" user="100"/>
  ...
</hh:charPr>
```

### What We're Using:
- ✅ **height** attribute → Base size (기준 크기)
- ✅ **spacing** element → Character spacing (자간) - currently all set to "0"
- ✅ **relSz** element → Relative size (상대 크기) - currently all set to "100"
- ✅ **fontRef** element → Font ID reference (글꼴 ID)
- ✅ **ratio** element → Character width (장평) - currently all set to "100"
- ✅ **offset** element → Character position (글자 위치) - currently all set to "0"

### What We Could Improve:
1. **Character spacing (자간)**: Allow adjustable spacing between characters
2. **Relative size (상대 크기)**: Make Korean text slightly larger than Latin
3. **Character width (장평)**: Allow compressed or expanded text

## Relationship to Line Spacing (줄간격)

Character shape properties directly affect line spacing:

1. **Base size (height)** determines the minimum line height
2. **Character spacing (spacing)** affects how much horizontal space text takes
3. **Line spacing** in paragraph properties is calculated as a percentage of character height

Formula:
```
Actual line height = character height × line spacing percentage
Example: 1000 × 160% = 1600 HWPUNIT
```

## References

- Binary format: 72 bytes total
- XML format: Used in HWPX/OWPML (what md2hwp generates)
- Related tables: Table 30 (attributes), Table 34 (language mapping)
