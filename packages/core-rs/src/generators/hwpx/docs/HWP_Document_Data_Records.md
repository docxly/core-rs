# HWP Document Data Records Reference

## 4.3. 본문의 데이터 레코드 (Document Data Records)

본문에서 사용되는 데이터 레코드는 다음과 같다.

## Table 57: Document Data Record Tags

| Tag ID | Value | 설명 (Description) |
|--------|-------|-------------------|
| HWPTAG_PARA_HEADER | HWPTAG_BEGIN+50 | 문단 헤더 (Paragraph header) |
| HWPTAG_PARA_TEXT | HWPTAG_BEGIN+51 | 문단의 텍스트 (Paragraph text) |
| HWPTAG_PARA_CHAR_SHAPE | HWPTAG_BEGIN+52 | 문단의 글자 모양 (Paragraph character shape) |
| HWPTAG_PARA_LINE_SEG | HWPTAG_BEGIN+53 | 문단의 레이아웃 (Paragraph layout) |
| HWPTAG_PARA_RANGE_TAG | HWPTAG_BEGIN+54 | 문단의 영역 태그 (Paragraph range tag) |
| HWPTAG_CTRL_HEADER | HWPTAG_BEGIN+55 | 컨트롤 헤더 (Control header) |
| **HWPTAG_LIST_HEADER** | **HWPTAG_BEGIN+56** | **문단 리스트 헤더 (Paragraph list header)** ⭐ |
| HWPTAG_PAGE_DEF | HWPTAG_BEGIN+57 | 용지 설정 (Page definition) |
| HWPTAG_FOOTNOTE_SHAPE | HWPTAG_BEGIN+58 | 각주/미주 모양 (Footnote/Endnote shape) |
| HWPTAG_PAGE_BORDER_FILL | HWPTAG_BEGIN+59 | 쪽 테두리/배경 (Page border/fill) |
| HWPTAG_SHAPE_COMPONENT | HWPTAG_BEGIN+60 | 개체 (Object) |
| HWPTAG_TABLE | HWPTAG_BEGIN+61 | 표 개체 (Table object) |
| HWPTAG_SHAPE_COMPONENT_LINE | HWPTAG_BEGIN+62 | 직선 개체 (Line object) |
| HWPTAG_SHAPE_COMPONENT_RECTANGLE | HWPTAG_BEGIN+63 | 사각형 개체 (Rectangle object) |
| HWPTAG_SHAPE_COMPONENT_ELLIPSE | HWPTAG_BEGIN+64 | 타원 개체 (Ellipse object) |
| HWPTAG_SHAPE_COMPONENT_ARC | HWPTAG_BEGIN+65 | 호 개체 (Arc object) |
| HWPTAG_SHAPE_COMPONENT_POLYGON | HWPTAG_BEGIN+66 | 다각형 개체 (Polygon object) |
| HWPTAG_SHAPE_COMPONENT_CURVE | HWPTAG_BEGIN+67 | 곡선 개체 (Curve object) |
| HWPTAG_SHAPE_COMPONENT_OLE | HWPTAG_BEGIN+68 | OLE 개체 (OLE object) |
| HWPTAG_SHAPE_COMPONENT_PICTURE | HWPTAG_BEGIN+69 | 그림 개체 (Picture object) |
| HWPTAG_SHAPE_COMPONENT_CONTAINER | HWPTAG_BEGIN+70 | 컨테이너 개체 (Container object) |
| HWPTAG_CTRL_DATA | HWPTAG_BEGIN+71 | 컨트롤 임의의 데이터 (Control arbitrary data) |
| HWPTAG_EQEDIT | HWPTAG_BEGIN+72 | 수식 개체 (Equation object) |
| RESERVED | HWPTAG_BEGIN+73 | 예약 (Reserved) |
| HWPTAG_SHAPE_COMPONENT_TEXTART | HWPTAG_BEGIN+74 | 글맵시 (Text art) |
| HWPTAG_FORM_OBJECT | HWPTAG_BEGIN+75 | 양식 개체 (Form object) |
| HWPTAG_MEMO_SHAPE | HWPTAG_BEGIN+76 | 메모 모양 (Memo shape) |
| HWPTAG_MEMO_LIST | HWPTAG_BEGIN+77 | 메모 리스트 헤더 (Memo list header) |
| HWPTAG_CHART_DATA | HWPTAG_BEGIN+79 | 차트 데이터 (Chart data) |
| HWPTAG_VIDEO_DATA | HWPTAG_BEGIN+82 | 비디오 데이터 (Video data) |
| HWPTAG_SHAPE_COMPONENT_UNKNOWN | HWPTAG_BEGIN+99 | Unknown |

## Key Tags for md2hwp

### HWPTAG_LIST_HEADER (HWPTAG_BEGIN+56) ⭐

This is the **paragraph list header** tag that controls line wrapping behavior (see Table 65):

**Structure:**
- INT16 (2 bytes): 문단 수 (paragraph count)
- UINT32 (4 bytes): 속성 (properties)
  - **bit 3~4**: 문단의 줄바꿈 (line wrapping mode)
    - 0 = 일반적인 줄바꿈 (normal wrapping) ✅
    - 1 = 자간을 조종하여 한 줄을 유지 (compress spacing) ❌
    - 2 = 내용에 따라 폭이 늘어남 (expand width)

**IMPORTANT:** This binary format tag doesn't have a direct equivalent in HWPX (XML) format!
- In binary HWP: Explicit control via bit 3~4
- In HWPX XML: Behavior determined implicitly by other settings

**Workaround for HWPX:**
- Set character spacing to 0 to avoid auto-compression
- Use `snapToGrid="0"` to prevent grid-based adjustments
- Rely on `lineWrap="BREAK"` in paragraph properties

### Other Relevant Tags

1. **HWPTAG_PARA_HEADER** (HWPTAG_BEGIN+50)
   - Paragraph header information
   - In HWPX: `<hp:p>` element attributes

2. **HWPTAG_PARA_CHAR_SHAPE** (HWPTAG_BEGIN+52)
   - Character shape for paragraph
   - In HWPX: `<hp:run charPrIDRef="...">` references

3. **HWPTAG_PARA_LINE_SEG** (HWPTAG_BEGIN+53)
   - Paragraph layout/line segments
   - In HWPX: `<hp:linesegarray><hp:lineseg>` elements

4. **HWPTAG_TABLE** (HWPTAG_BEGIN+61)
   - Table objects
   - In HWPX: `<hp:tbl>` elements

## Binary vs XML Format

| Binary (HWP 3.0) | XML (HWPX 5.0+) | Notes |
|------------------|-----------------|-------|
| HWPTAG records | XML elements | Different structure |
| Bit-level control | Attribute-based | Less granular control |
| Explicit flags | Implicit behavior | May have limitations |

## Impact on md2hwp Implementation

Since we generate HWPX (XML) format, we cannot directly control all the binary format settings like HWPTAG_LIST_HEADER bit 3~4. This means:

✅ **What we CAN control:**
- Character properties (height, font, weight)
- Paragraph properties (spacing, alignment, margins)
- Line spacing percentages
- Grid snapping on/off

❌ **What we CANNOT directly control:**
- Line wrapping compression mode (bit 3~4)
- Some low-level formatting behaviors

**Solution:** Use XML-compatible approaches:
- Avoid positive character spacing that triggers compression
- Use `snapToGrid="0"` to prevent adjustments
- Rely on paragraph margins and line spacing for readability

## References

- Table 57: Document Data Records (본문의 데이터 레코드)
- Table 65: Paragraph List Header (문단 리스트 헤더)
- Table 33: Character Shape (글자 모양)
- HWP Binary Format Specification
- HWPX/OWPML XML Format Specification
