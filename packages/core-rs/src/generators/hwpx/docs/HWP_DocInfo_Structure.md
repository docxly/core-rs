# HWP DocInfo Stream Structure Reference

This document contains reference information about the DocInfo stream structure in HWP binary format.

## Table 4: Document Information (표 4 문서 정보)

The following table shows the data stored in the DocInfo stream:

| Tag ID | 길이(바이트) | 레벨 | 설명 |
|--------|------------|------|------|
| HWPTAG_DOCUMENT_PROPERTIES | 30 | 0 | 문서 속성(표 14 참조) |
| HWPTAG_ID_MAPPINGS | 32 | 0 | 아이디 매핑 헤더(표 15 참조) |
| HWPTAG_BIN_DATA | 가변 | 1 | 바이너리 데이터(표 17 참조) |
| HWPTAG_FACE_NAME | 가변 | 1 | 글꼴(표 19 참조) |
| HWPTAG_BORDER_FILL | 가변 | 1 | 테두리/배경(표 23 참조) |
| HWPTAG_CHAR_SHAPE | 72 | 1 | 글자 모양(표 33 참조) |
| HWPTAG_TAB_DEF | 14 | 1 | 탭 정의(표 36 참조) |
| HWPTAG_NUMBERING | 가변 | 1 | 문단 번호(표 38 참조) |
| HWPTAG_BULLET | 10 | 1 | 글머리표(표 42 참조) |
| HWPTAG_PARA_SHAPE | 54 | 1 | 문단 모양(표 43 참조) |
| HWPTAG_STYLE | 가변 | 1 | 스타일(표 47 참조) |
| HWPTAG_MEMO_SHAPE | 22 | 1 | 메모 모양 |
| HWPTAG_TRACK_CHANGE_AUTHOR | 가변 | 1 | 변경 추적 작성자 |
| HWPTAG_TRACK_CHANGE | 가변 | 1 | 변경 추적 내용 및 모양 |
| HWPTAG_DOC_DATA | 가변 | 0 | 문서 임의의 데이터(표 49 참조) |
| HWPTAG_FORBIDDEN_CHAR | 가변 | 0 | 금지처리 문자 |
| HWPTAG_COMPATIBLE_DOCUMENT | 4 | 0 | 호환 문서(표 54 참조) |
| HWPTAG_LAYOUT_COMPATIBILITY | 20 | 1 | 레이아웃 호환성(표 56 참조) |
| HWPTAG_DISTRIBUTE_DOC_DATA | 256 | 0 | 배포용 문서 |
| HWPTAG_TRACKCHANGE | 1032 | 1 | 변경 추적 정보 |
| 전체 길이 | 가변 | | |

## Notes

### Tag Length
- **Fixed size**: Specific byte count (e.g., 30, 32, 72)
- **가변 (Variable)**: Variable length depending on content

### Level
- **0**: Top-level tags
- **1**: Nested/child tags

### Key Tags for md2hwp Implementation

The following tags are particularly relevant for our current implementation:

1. **HWPTAG_FACE_NAME** (표 19) - Font faces
   - Currently implemented in `generateHeaderXml()`
   - Defines fonts like "맑은 고딕" and "Arial"

2. **HWPTAG_BORDER_FILL** (표 23) - Borders and backgrounds
   - Currently implemented for table borders
   - borderFill id="1" (invisible) and id="2" (visible 0.12mm)

3. **HWPTAG_CHAR_SHAPE** (표 33) - Character properties
   - **Recently implemented** for heading hierarchy
   - charPr id="0" through id="10" for different font sizes

4. **HWPTAG_PARA_SHAPE** (표 43) - Paragraph properties
   - Currently implemented as paraPr id="0" and id="20"

5. **HWPTAG_STYLE** (표 47) - Styles
   - Currently only "바탕글" (Normal) style implemented
   - Could be extended for outline styles (개요 1-7)

## Future Implementation Opportunities

Based on this structure, we could implement:
- **HWPTAG_NUMBERING**: For numbered lists
- **HWPTAG_BULLET**: For enhanced bullet formatting
- **HWPTAG_TAB_DEF**: For custom tab stops
- **HWPTAG_STYLE**: For proper heading outline styles

## References

This information is based on the official HWP binary format specification.
Related specifications can be found at:
- HWPX/OWPML XML format (currently used in md2hwp)
- HWP 5.0+ binary format specification
