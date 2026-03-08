# 한글 문서 파일 구조 3.0 / HWPML - HWPML XML structure

- Source document: `한글문서파일형식3.0_HWPML_revision1.2.pdf`
- Source URL: https://cdn.hancom.com/link/docs/%ED%95%9C%EA%B8%80%EB%AC%B8%EC%84%9C%ED%8C%8C%EC%9D%BC%ED%98%95%EC%8B%9D3.0_HWPML_revision1.2.pdf
- Physical PDF pages: 55-120
- Extraction method: `pdftotext -layout`
- Notes: Printed section: `II. HWPML 구조` (printed pages 47-112). Includes HWPML root, header, body, and XML element descriptions. This is the closest free official text to OWPML/HWPX XML semantics.

## Extracted text

```text
II. HWPML 구조

--- page break ---

HWPML

48

--- page break ---

                                                              HWPML

1. 개요
  HWPML은 한글 워드 프로세서 문서를 기술하기 위한 W3C XML 기반의 개방형 마크업 언어이다.

2. 형식 설명
  HWPML 엘리먼트에 대한 설명은 다음 표 형식을 기본으로 해서 설명한다.

                             엘리먼트 명
      설명           엘리먼트에 대한 설명
   부모 엘리먼트
   자식 엘리먼트/
    엘리먼트 값
  속   속성명1         속성1에 대한 설명                   값의 범위   기본값
  성   속성명2         속성2에 대한 설명                   값의 범위   기본값

                      표 1 각 엘리먼트 설명을 위한 테이블 예

   값을 가지지 않는 경우에는 공백으로 남겨둔다.

   속성들이 가질 수 있는 값들의 경우

      - 열거형은 값들을 '|'으로 구분해서 표기한다.
      - 범위를 가지는 경우에는 ‘최소값 ~ 최대값’과 같은 형식으로 표기한다.
      - 일반적인 값의 범위를 가질 경우에는 해당 범위를 서술식으로 표기한다.

   속성값에 대해서 부가적인 설명이 필요한 경우 ‘속성값 (설명)’과 같은 형식으로 표기한다.

2.1. 기본 속성 값 형식 설명
  2.2에서 설명할 기본 속성 값은 다음 표 형식을 기본으로 해서 설명한다.

                                  속성 이름
   축약어     축약어
              값1          값1에 대한 설명
      설명      값2          값2에 대한 설명
              값3          값3에 대한 설명

   제시된 축약어는 본문 내에서 [축약어]와 같은 형식으로 사용된다.

2.2. 기본 속성 값

                                  hwpunit
      설명   10 pt = 1000 hwpunit

                                  글꼴 유형
            rep    대표 글꼴
      설명     ttf   트루타입 글꼴
            hft    한글 전용 글꼴

                                                                 49

--- page break ---

HWPML

                                          선 종류 1
        축약어   LineType1
                    Solid     실선
                    Dash      긴 점선
                     Dot      점선
                  DashDot     -.-.-.-.
               DashDotDot     -..-..-..
                LongDash      Dash보다 긴 선분의 반복
        설명
                   Circle     Dot보다 큰 동그라미의 반복
               DoubleSlim     2중선
                 SlimThick    가는 선 + 굵은 선 2중선
                 ThickSlim    굵은 선 + 가는 선 2중선
              SlimThickSlim   가는 선 + 굵은 선 + 가는 선 3중선
                    None      선 없음

                                          선 종류 2
        축약어   LineType2
                    Solid     실선
                    Dash      긴 점선
                     Dot      점선
                  DashDot     -.-.-.-.
               DashDotDot     -..-..-..
        설명      LongDash      Dash보다 긴 선분의 반복
                   Circle     Dot보다 큰 동그라미의 반복
               DoubleSlim     2중선
                 SlimThick    가는 선 + 굵은 선 2중선
                 ThickSlim    굵은 선 + 가는 선 2중선
              SlimThickSlim   가는 선 + 굵은 선 + 가는 선 3중선

                                          선 종류 3
        축약어   LineType3
                  Solid       실선
                   Dot        점선
                  Thick       두꺼운 선
        설명
                  Dash        긴 점선
                 DashDot      -.-.-.-.
               DashDotDot     -..-..-..

50

--- page break ---

                                                HWPML

                                선 두께
축약어   LineWidth
          0.1mm
         0.12mm
         0.15mm
          0.2mm
         0.25mm
          0.3mm
          0.4mm
          0.5mm
설명
          0.6mm
          0.7mm
          1.0mm
          1.5mm
          2.0mm
          3.0mm
          4.0mm
          5.0mm

                                색
축약어   RGB-Color
      RGB 값 (0x00bbggrr)을 십진수로 표현한 값
           rr       red 1 byte
설명
          gg        green 1 byte
          bb        blue 1 byte

                              번호 모양 1
축약어   NumberType1
               Digit          1, 2, 3
           CircledDigit       동그라미 쳐진 1, 2, 3
          RomanCapital        I, II, III
           RomanSmall         i, ii, iii
           LatinCapital       A, B, C
            LatinSmall        a, b, c
       CircledLatinCapital    동그라미 쳐진 A, B, C
설명      CircledLatinSmall     동그라미 쳐진 a, b, c
          HangulSyllable      가, 나, 다
      CircledHangulSyllable   동그라미 쳐진 가, 나, 다
           HangulJamo         ㄱ, ㄴ, ㄷ
       CircledHangulJamo      동그라미 쳐진 ㄱ, ㄴ, ㄷ
         HangulPhonetic       일, 이, 삼
            Ideograph         一, 二, 三
        CircledIdeograph      동그라미 쳐진 一, 二, 三

                                                   51

--- page break ---

HWPML

                                        번호 모양 2
        축약어   NumberType2
                       Digit           1, 2, 3
                   CircledDigit        동그라미 쳐진 1, 2, 3
                  RomanCapital         I, II, III
                   RomanSmall          i, ii, iii
                   LatinCapital        A, B, C
                    LatinSmall         a, b, c
               CircledLatinCapital     동그라미 쳐진 A, B, C
                CircledLatinSmall      동그라미 쳐진 a, b, c
                  HangulSyllable       가, 나, 다
        설명    CircledHangulSyllable    동그라미 쳐진 가, 나, 다
                   HangulJamo          ㄱ, ㄴ, ㄷ
               CircledHangulJamo       동그라미 쳐진 ㄱ, ㄴ, ㄷ
                 HangulPhonetic        일, 이, 삼
                    Ideograph          一, 二, 三
                CircledIdeograph       동그라미 쳐진 一, 二, 三
                  DecagonCircle        갑, 을, 병, 정, 무, 기, 경, 신, 임, 계
               DecagonCircleHanja      甲, 乙, 丙, 丁, 戊, 己, 庚, 辛, 壬, 癸
                      Symbol           4가지 문자가 차례로 반복
                     UserChar          사용자 지정 문자 반복

                                        정렬 방식 1
        축약어   AlignmentType1
                   Justify      양쪽 정렬
                    Left        왼쪽 정렬
                    Right       오른쪽 정렬
        설명
                   Center       가운데 정렬
                 Distribute     배분 정렬
              DistributeSpace   나눔 정렬(공백에만 배분)

                                 정렬 방식 2
        축약어   AlignmentType2
                   Left      왼쪽 정렬
        설명        Center     가운데 정렬
                   Right     오른쪽 정렬

                                      화살표 시작/끝 모양
        축약어   ArrowType
                  Normal        모양 없음
                   Arrow        화살 모양
                   Spear        작살 모양
              ConcaveArrow      오목한 화살모양
              EmptyDiamond      속이 빈 다이아몬드 모양
        설명
               EmptyCircle      속이 빈 원 모양
                 EmptyBox       속이 빈 사각 모양
              FilledDiamond     속이 채워진 다이아몬드 모양
                FilledCircle    속이 채워진 원 모양
                 FilledBox      속이 채워진 사각 모양

52

--- page break ---

                                          HWPML

                           화살표 시작/끝 크기
축약어   ArrowSize
        SmallSmall      작은-작은
       SmallMedium      작은-중간
        SmallLarge      작은-큰
       MediumSmall      중간-작은
설명    MediumMedium      중간-중간
       MediumLarge      중간-큰
        LargeSmall      큰-작은
       LargeMedium      큰-중간
        LargeLarge      큰-큰

                                  언어 종류
축약어   LangType
         Hangul         한글
          Latin         영어
          Hanja         한자
설명      Japanese        일본어
          Other         기타
         Symbol         심볼
          User          사용자

                                  무늬 종류
축약어   HatchStyle
        Horizontal      - - - -
          Vertical      |||||
        BackSlash       \\\\\
설명
           Slash        /////
           Cross        +++++
      CrossDiagonal     xxxxx

                              채우기 유형
축약어   InfillMode
              Tile      바둑판식으로-모두
         TileHorzTop    바둑판식으로-가로/위
       TileHorzBottom   바둑판식으로-가로/아래
         TileVertLeft   바둑판식으로-세로/왼쪽
        TileVertRight   바둑판식으로-세로/오른쪽
             Total      크기에 맞추어
            Center      가운데로
          CenterTop     가운데 위로
설명
        CenterBottom    가운데 아래로
          LeftCenter    왼쪽 가운데로
            LeftTop     왼쪽 위로
          LeftBottom    왼쪽 아래로
         RightCenter    오른쪽 가운데로
           RightTop     오른쪽 위로
         RightBottom    오른쪽 아래로
             Zoom       확대

                                             53

--- page break ---

HWPML

                                      Line Wrap
        축약어   LineWrapType
                  Break        일반적인 줄바꿈
        설명       Squeeze       자간을 조정하여 한 줄을 유지
                  Keep         내용에 따라 폭이 늘어남

                                      Text Wrap
        축약어   TextWrapType
                   Square      bound rect를 따라
                    Tight      오브젝트의 outline을 따라
                  Through      오브젝트 내부의 빈 공간까지
        설명
              TopAndBottom     좌/우에는 텍스트를 배치하지 않음
                BehindText     글과 겹치게 하여 글 뒤로
               InFrontOfText   글과 겹치게 하여 글 앞으로

                                      필드의 종류
        축약어   FieldType
                     Clickhere        누름틀
                     Hyperlink        하이퍼링크
                     Bookmark         블록 책갈피
                      Formula         표계산식
                     Summery          문서요약
                      UserInfo        사용자정보
                        Date          현재 날짜/시간
                      DocDate         문서 날짜/시간
                        Path          파일 경로
                      Crossref        상호참조
                     Mailmerge        메일머지
                       Memo           메모
                  RevisionChange
                    RevisionSign
                  RevisionDelete
        설명
                  RevisionAttach
                  RevisionClipping
                 RevisionSawtooth
                 RevisionThinking
                   RevisionPraise
                    RevisionLine
              RevisionSimpleChange
                 RevisionHyperlink
                RevisionLineAttach
                 RevisionLineLink
               RevisionLineTransfer
                RevisionRightmove
                 RevisionLeftmove
                 RevisionTransfer
                    RevisionSplit

54

--- page break ---

                                                                         HWPML

3. 루트 엘리먼트
                              HWPML
     설명          HWPML의 시작을 알리는 루트 엘리먼트.
   부모 엘리먼트
   자식 엘리먼트       HEAD, BODY, TAIL
  속   Version    HWPML의 버전.                        2.8           2.8
    SubVersion                                   8.0.0.0       8.0.0.0
  성    Style2                                 embed | export   embed

                             표 2 HWPML 엘리먼트

                                                                            55

--- page break ---

HWPML

     4. 헤더 엘리먼트
                                     HEAD
                    문서요약정보 외에 글꼴, 글자속성, 문단속성에 대한 테이블 등 문서에
           설명
                    대한 전반적인 정보를 담고 있다.
        부모 엘리먼트     HWPML
        자식 엘리먼트     DOCSUMMARY, DOCSETTING, MAPPINGTABLE
        속                                               0 이상의
           SecCnt   구역의 개수
        성                                                정수형

                                 표 3 HEAD 엘리먼트

     4.1. 문서 요약 정보 엘리먼트

                                     DOCSUMMARY
          설명        문서요약정보를 담는다.
        부모 엘리먼트     HEAD
                    TITLE, SUBJECT, AUTHOR, DATE, KEYWORDS, COMMENTS,
        자식 엘리먼트
                    FORBIDDENSTRING
                              표 4 DOCSUMMARY 엘리먼트

                                      TITLE
           설명       문서 제목
        부모 엘리먼트     DOCSUMMARY
         엘리먼트 값     문자열 형태의 문서 제목
                                 표 5 TITLE 엘리먼트

                                     SUBJECT
           설명       문서 주제
        부모 엘리먼트     DOCSUMMARY
         엘리먼트 값     문자열 형태의 문서 주제
                                표 6 SUBJECT 엘리먼트

                                 AUTHOR
           설명       문서 저자
        부모 엘리먼트     DOCSUMMARY
         엘리먼트 값     문자열 형태의 문서 저자
                                표 7 AUTHOR 엘리먼트

                                  DATE
           설명       작성 날짜
        부모 엘리먼트     DOCSUMMARY
         엘리먼트 값     문자열 형태의 문서 작성 날짜
                                 표 8 DATE 엘리먼트

56

--- page break ---

                                                    HWPML

                      KEYWORDS
   설명     키워드
부모 엘리먼트   DOCSUMMARY
 엘리먼트 값   문자열 형태의 문서 키워드
                     표 9 KEYWORDS 엘리먼트

                      COMMENTS
   설명     기타 설명
부모 엘리먼트   DOCSUMMARY
 엘리먼트 값   문자열 형태의 기타 설명
                    표 10 COMMENTS 엘리먼트

                       FORBIDDENSTRING
  설명      금칙 문자
부모 엘리먼트   DOCSUMMARY
자식 엘리먼트   FORBIDDEN
                  표 11 FORBIDDENSTRING 엘리먼트

                            FORBIDDEN
   설명     금지 문자열
부모 엘리먼트   FORBIDDENSTRING
 엘리먼트 값   문자열
속
    id    한정자                                 문자열
성
                       표 12 HEAD 엘리먼트

                                                       57

--- page break ---

HWPML

     4.2. 문서 설정 정보 엘리먼트

                                      DOCSETTING
          설명          각종 설정 정보를 담는다.
        부모 엘리먼트       HEAD
        자식 엘리먼트       BEGINNUMBER, CARETPOS
                                표 13 DOCSETTING 엘리먼트

                                 BEGINNUMBER
           설명         문서 내 각종 시작번호에 대한 정보
        부모 엘리먼트       DOCSETTING
         엘리먼트 값
             Page     페이지 시작 번호                        1 이상의 정수형   1
           Footnote   각주 시작 번호                         1 이상의 정수형   1
        속  Endnote    미주 시작 번호                         1 이상의 정수형   1
            Picture   그림 시작 번호                         1 이상의 정수형   1
        성    Table    표 시작 번호                          1 이상의 정수형   1
           Equation   수식 시작 번호                         1 이상의 정수형   1
          TotalPage   전체 페이지 수                         1 이상의 정수형   1

                               표 14 BEGINNUMBER 엘리먼트

                                  CARETPOS
           설명         문서 내 캐럿의 위치 정보
        부모 엘리먼트       DOCSETTING
         엘리먼트 값
        속   List      리스트 아이디                            문자열
            Para      문단 아이디                             문자열
        성   Pos       문단 내에서의 글자단위 위치                    문자열

                                 표 15 CARETPOS 엘리먼트

58

--- page break ---

                                                                                HWPML

4.3. 문서 글꼴/스타일 정보

                                 MAPPINGTABLE
                 본문에 사용 중인 글꼴, 글자속성, 문단속성, 탭, 스타일 등등에 대한
       설명
                 세부정보를 담고 있다.
    부모 엘리먼트      HEAD
                 BINDATALIST, FACENAMELIST,
                 BORDERFILLLIST, CHARSHAPELIST,
    자식 엘리먼트
                 TABDEFLIST, NUMBERINGLIST, BULLETLIST,
                 PARASHAPELIST, STYLELIST, MEMOSHAPELIST
                          표 16 MAPPINGTABLE 엘리먼트

4.3.1. 문서 내 그림/OLE 정보

                              BINDATALIST
     설명          그림, OLE등의 바이너리 데이터 리스트.
   부모 엘리먼트       MAPPINGTABLE
   자식 엘리먼트       BINITEM
   속
      Count      BINITEM의 갯수                         0 이상의 정수형              0
   성
                           표 17 BINDATALIST 엘리먼트

                                   BINITEM
                 그림, OLE등의 바이너리 데이터 아이템에 대한 정보.
                 DTD상으로는 Type을 제외한 속성들이 #IMPLIED로 선언되어 있지만,
       설명
                 실제로는 Type="Link"일때 APath와 RPath가 필수이며,
                 Type="Embedding"일때는 BinData와 Format이 필수이다.
    부모 엘리먼트      BINDATALIST
     엘리먼트 값
                                                         Link (외부 파일) |
                 그림의 경우 "Link"와 "Embedding"만 가능.
        Type                                           Embedding (문서 포함)
                 OLE의 경우 "Storage"만 가능.                  | Storage (OLE)
       APath     Type이 "Link"일때, 연결 파일의 절대 경로
       RPath     Type이 "Link"일때, 연결 파일의 상대 경로
   속             Type이 "Embedding"이거나 "Storage"일때,
   성   BinData   BINDATASTORAGE에 저장된 바이너리
                 데이터의 아이디
                 Type이 "Embedding"일때, 바이너리
                 데이터의 포맷종류                              jpg | bmp | gif |
       Format
                 그림의 경우 "jpg", "bmp", "gif" 중 선택.              ole
                 OLE의 경우 "ole"만 가능.
                             표 18 BINITEM 엘리먼트

                                                                                   59

--- page break ---

HWPML

     4.3.2. 글꼴 정보

                                       FACENAMELIST
           설명          글꼴 리스트
         부모 엘리먼트       MAPPINGTABLE
         자식 엘리먼트       FONTFACE
                                표 19 FACENAMELIST 엘리먼트

                                    FONTFACE
          설명           언어별 글꼴 그룹
        부모 엘리먼트        FACENAMELIST
        자식 엘리먼트        FONT
        속   Lang       글꼴에 대한 언어 종류.                                 [LangType]
        성  Count       글꼴의 갯수                                      0 이상의 정수형

                                  표 20 FONTFACE 엘리먼트

                                          FONT
          설명           각각의 글꼴
        부모 엘리먼트        FONTFACE
        자식 엘리먼트        SUBSTFONT, TYPEINFO
        속    Id        글꼴 아이디                          0 이상의 정수형
            Type       글꼴의 유형.                          rep | ttf | hft
        성  Name        글꼴 이름                               문자열

                                      표 21 FONT 엘리먼트

                                        SUBSTFONT
           설명          대체 글꼴
        부모 엘리먼트        FONT
         엘리먼트 값
        속   Type       대체 글꼴의 유형.                        rep | ttf | hft
        성   Name       글꼴 이름                                 문자열

                                 표 22 SUBSTFONT 엘리먼트

                                     TYPEINFO
                            글꼴속성에 따라 글꼴을 대체하는 PANOSE시스템의 폰트 분류
              설명
                            속성들에 대한 정보
          부모 엘리먼트           FONT
           엘리먼트 값
            FamilyType      글꼴 계열
             SerifStyle     세리프 유형
              Weight        굵기
            Proportion      비례
        속    Contrast       대조
        성 StrokeVariation   스트로크 편차
             ArmStyle       자획유형
            Letterform      글자형
              Midline       중간선
              XHeight       X-높이
                                  표 23 TYPEINFO 엘리먼트

60

--- page break ---

                                                                                      HWPML

4.3.3. 테두리/배경/채우기 정보

                                   BORDERFILLLIST
     설명               테두리/배경/채우기 정보 리스트
   부모 엘리먼트            MAPPINGTABLE
   자식 엘리먼트            BORDERFILL
   속
      Count           테두리/배경 항목의 갯수                     0 이상의 정수형
   성
                                표 24 BORDERFILLLIST 엘리먼트

                                      BORDERFILL
                                문단, 표의 셀, 그림 및 그리기 개체에서 사용하는
              설명
                                테두리/배경/채우기 에 대한 각각의 세부 정보
          부모 엘리먼트               BORDERFILLLIST
                                LEFTBORDER, RIGHTBORDER, TOPBORDER,
          자식 엘리먼트
                                BOTTOMBORDER, DIAGONAL, FILLBRUSH
                                                                  1 이상의
                 Id             테두리/채우기 항목 아이디
                                                                    정수
              ThreeD            3D효과 on/off (미구현)               true | false    false
              Shadow            그림자 효과 on/off (미구현)             true | false    false
                                테두리/배경 대화상자의 Slash 대각선
                                                                0 | 2 | 3 |
               Slash            모양 중 왼쪽부터 차례대로 "0", "2",                          0
                                                                   6 | 7
                                "3", "6", "7"
                                테두리/배경 대화상자의 BackSlash
                                                                0 | 2 | 3 |
   속         BackSlash          대각선 모양 중 왼쪽부터 차례대로                                0
                                                                   6 | 7
                                "0", "2", "3", "6", "7"
   성                            꺽어진 대각선.
                                Slash, BackSlash의 가운데 대각선이
                                꺽어진 대각선임을 나타냄.
            CrookedSlash                                                          0
                                테두리/배경 대화상자의 Slash 또는
                                BackSlash 대각선 모양 중 마지막
                                6번째 모양을 표시한다.
            CounterSlash                                                          0
          CounterBackSlash                                                        0
        BreakCellSeparateLine                                                     0

                                 표 25 BORDERFILL 엘리먼트

                                      LEFTBORDER
                                      RIGHTBORDER
                                       TOPBORDER
                                     BOTTOMBORDER
                                  DIAGONAL
      설명              왼쪽/오른쪽/위/아래/대각선 테두리 정보
   부모 엘리먼트            BORDERFILL
    엘리먼트 값
       Type           테두리선 종류                                [LineType1]        Sold
   속                                                                           0.12m
       Width          테두리선 굵기.                               [LineWidth]
   성                                                                             m
       Color          테두리선 색상.                               [RGB-Color]          0

       표 26 LEFTBORDER, RIGHTBORDER, TOPBORDER, BOTTOMBORDER, DIAGONAL 엘리먼트

                                                                                         61

--- page break ---

HWPML

                                        FILLBRUSH
          설명             채우기 정보
        부모 엘리먼트          BORDERFILL
        자식 엘리먼트          WINDOWBRUSH, GRADATION, IMAGEBRUSH
                                     표 27 FILLBRUSH 엘리먼트

                                         WINDOWBRUSH
            설명           면 채우기
         부모 엘리먼트         FILLBRUSH
         엘리먼트 값
           FaceColor     면색                                             [RGB-Color]
        속 HatchColor     무늬색                                            [RGB-Color]

        성   HatchStyle   무늬종류                                           [HatchStyle]

              Alpha
                                  표 28 WINDOWBRUSH 엘리먼트

                                          GRADATION
           설명            그라데이션 효과
         부모 엘리먼트         FILLBRUSH
         자식 엘리먼트         COLOR
                                                         Linear (줄무늬형) | Radial (원형) |
              Type       그라데이션 유형.
                                                         Conical (원뿔형) | Square (사각형)
              Angle      그러데이션의 기울임(시작각)                                                  90
             CenterX     그러데이션의 가로중심(중심 X 좌표)                                              0
        속    CenterY     그러데이션의 세로중심(중심 Y 좌표)                                              0
        성     Step       그러데이션 번짐 정도                           0 ~ 100                    50
                                                          워디안/한글2002/SE에서는
            ColorNum     그러데이션의 색수                                                        2
                                                              항상 2이다.
            StepCenter   그러데이션 번짐 정도의 중심                      0 ～ 100                     50
               Alpha
                                     표 29 GRADATION 엘리먼트

                                            COLOR
           설명            그라데이션 색
        부모 엘리먼트          GRADATION
         엘리먼트 값
        속
            Value        색                                  [RGB-Color]
        성
                                       표 30 COLOR 엘리먼트

                                          IMAGEBRUSH
          설명             그림으로 채우기
        부모 엘리먼트          FILLBRUSH
        자식 엘리먼트          IMAGE
        속
           Mode          채우기 유형                                           [InfillMode]   Tile
        성
                                     표 31 IMAGEBRUSH 엘리먼트

62

--- page break ---

                                                                         HWPML

                                IMAGE
   설명         그림 정보
부모 엘리먼트       IMAGEBRUSH
 엘리먼트 값
    Bright    밝기                                                     0
   Contrast   명암                                                     0
                                              RealPic (원래 그림에서) |
속   Effect    그림 효과                          GrayScale (그레이스케일로) |
성                                              BlackWhite (흑백으로)
              BINDATALIST의 BINITEM엘리먼트의
    BinItem
              아이디 참조값
     Alpha
                           표 32 IMAGE 엘리먼트

                                                                            63

--- page break ---

HWPML

     4.3.4. 글자 모양 정보

                                       CHARSHAPELIST
          설명           글자 모양 리스트
        부모 엘리먼트        MAPPINGTABLE
        자식 엘리먼트        CHARSHAPE
        속
           Count       글자 모양 항목 갯수                         0 이상의 정수
        성
                                 표 33 CHARSHAPELIST 엘리먼트

                                          CHARSHAPE
              설명            글자 모양 정보
            부모 엘리먼트         CHARSHAPELIST
                            FONTID, RATIO, CHARSPACING, RELSIZE, CHAROFFSET,
            자식 엘리먼트         ITALIC, BOLD, UNDERLINE, OUTLINE, SHADOW, EMBOSS,
                            ENGRAVE, SUPERSCRIPT, SUBSCRIPT
                 Id         글자 모양 아이디                      0 이상의 정수
               Height       글자 크기.                           [hwpunit]        1000
              TextColor     글자색                             [RGB-Color]         0
        속    ShadeColor     음영색                             [RGB-Color]    4294967295
        성   UseFontSpace    글꼴에 어울리는 빈칸                     true | false      false
             UseKerning     커닝                              true | false      false
              SymMark       강조점 종류                                              0
             BorderFillId   글자테두리 기능
                                   표 34 CHARSHAPE 엘리먼트

                                          FONTID
            설명         언어별 글꼴
         부모 엘리먼트       CHARSHAPE
          엘리먼트 값
                                                  <FONTFACE Lang="Hangul">의
             Hangul    한글글꼴 아이디 참조값.             자식엘리먼트인 FONT엘리먼트들 중
                                                참조하고자 하는 엘리먼트의 Id 속성값
                                                   <FONTFACE Lang="Latin">의
              Latin    영문글꼴 아이디 참조값.             자식엘리먼트인 FONT엘리먼트들 중
                                                참조하고자 하는 엘리먼트의 Id 속성값
                                                   <FONTFACE Lang="Hanja">의
             Hanja     한자글꼴 아이디 참조값.             자식엘리먼트인 FONT엘리먼트들 중
                                                참조하고자 하는 엘리먼트의 Id 속성값
        속                                        <FONTFACE Lang="Japanese">의
            Japanese   일본어글꼴 아이디 참조값.            자식엘리먼트인 FONT엘리먼트들 중
        성                                       참조하고자 하는 엘리먼트의 Id 속성값
                                                   <FONTFACE Lang="Other">의
             Other     외국어글꼴 아이디 참조값.            자식엘리먼트인 FONT엘리먼트들 중
                                                참조하고자 하는 엘리먼트의 Id 속성값
                                                  <FONTFACE Lang="Symbol">의
             Symbol    기호글꼴 아이디 참조값.             자식엘리먼트인 FONT엘리먼트들 중
                                                참조하고자 하는 엘리먼트의 Id 속성값
                                                   <FONTFACE Lang="User">의
              User     사용자글꼴 아이디 참조값.            자식엘리먼트인 FONT엘리먼트들 중
                                                참조하고자 하는 엘리먼트의 Id 속성값

                                     표 35 FONTID 엘리먼트

64

--- page break ---

                                                              HWPML

                               RATIO
   설명        언어별 장평
부모 엘리먼트      CHARSHAPE
 엘리먼트 값
   Hangul    한글글꼴에서의 장평                      50% ~ 200%   100
    Latin    영문글꼴에서의 장평                      50% ~ 200%   100

속   Hanja    한자글꼴에서의 장평                      50% ~ 200%   100
  Japanese   일본어글꼴에서의 장평                     50% ~ 200%   100
성   Other    외국어글꼴에서의 장평                     50% ~ 200%   100
   Symbol    기호글꼴에서의 장평                      50% ~ 200%   100
    User     사용자글꼴에서의 장평                     50% ~ 200%   100

                         표 36 RATIO 엘리먼트

                            CHARSPACING
   설명        언어별 자간
부모 엘리먼트      CHARSHAPE
 엘리먼트 값
   Hangul    한글글꼴에서의 자간                      -50% ~ 50%   0
    Latin    영문글꼴에서의 자간                      -50% ~ 50%   0
속   Hanja    한자글꼴에서의 자간                      -50% ~ 50%   0
  Japanese   일본어글꼴에서의 자간                     -50% ~ 50%   0
성   Other    외국어글꼴에서의 자간                     -50% ~ 50%   0
   Symbol    기호글꼴에서의 자간                      -50% ~ 50%   0
    User     사용자글꼴에서의 자간                     -50% ~ 50%   0

                         표 37 RATIO 엘리먼트

                         RELSIZE
   설명        언어별 글자의 상대크기
부모 엘리먼트      CHARSHAPE
 엘리먼트 값
   Hangul    한글글꼴에서의 상대크기                    10% ~ 250%   100
    Latin    영문글꼴에서의 상대크기                    10% ~ 250%   100
속   Hanja    한자글꼴에서의 상대크기                    10% ~ 250%   100
  Japanese   일본어글꼴에서의 상대크기                   10% ~ 250%   100
성   Other    외국어글꼴에서의 상대크기                   10% ~ 250%   100
   Symbol    기호글꼴에서의 상대크기                    10% ~ 250%   100
    User     사용자글꼴에서의 상대크기                   10% ~ 250%   100

                         표 38 RELSIZE 엘리먼트

                                                                 65

--- page break ---

HWPML

                                 CHAROFFSET
                     언어별 글자위치(상하위치)
            설명       0%을 기준으로 하여 100%에 가까울수록 글자위치가 아래로 내려가고
                     -100%에 가까울수록 위로 올라간다.
        부모 엘리먼트      CHARSHAPE
         엘리먼트 값
           Hangul    한글글꼴에서의 글자위치                               -100% ~ 100%         0
            Latin    영문글꼴에서의 글자위치                               -100% ~ 100%         0
        속   Hanja    한자글꼴에서의 글자위치                               -100% ~ 100%         0
          Japanese   일본어글꼴에서의 글자위치                              -100% ~ 100%         0
        성   Other    외국어글꼴에서의 글자위치                              -100% ~ 100%         0
           Symbol    기호글꼴에서의 글자위치                               -100% ~ 100%         0
            User     사용자글꼴에서의 글자위치                              -100% ~ 100%         0

                              표 39 CHAROFFSET 엘리먼트

                                       ITALIC
           설명        글자 속성 : 기울임
        부모 엘리먼트      CHARSHAPE
         엘리먼트 값
                                  표 40 ITALIC 엘리먼트

                                       BOLD
           설명        글자 속성 : 진하게
        부모 엘리먼트      CHARSHAPE
         엘리먼트 값
                                  표 41 BOLD 엘리먼트

                                     UNDERLINE
           설명        글자 속성 : 밑줄
        부모 엘리먼트      CHARSHAPE
         엘리먼트 값
                                           Bottom (글자 아래) | Center (글자 중간) |      Botto
        속    Type    밑줄 종류
                                                      Top (글자 위)                   m
        성   Shape    밑줄 모양                           [LineType2]                  Solid
            Color    밑줄 색                            [RGB-Color]                    0

                              표 42 UNDERLINE 엘리먼트

                                     STRIKEOUT
           설명        글자 속성 : 취소선
        부모 엘리먼트      CHARSHAPE
         엘리먼트 값
                                                            None (없음) |        Continuou
        속    Type    취소선 종류
                                                         Continuous (연속선)          s
        성   Shape                                          [LineType2]           Solid
            Color    취소선 색                                 [RGB-Color]             0

                              표 43 STRIKEOUT 엘리먼트

66

--- page break ---

                                                                             HWPML

                              OUTLINE
   설명        글자 속성 : 외곽선
부모 엘리먼트      CHARSHAPE
 엘리먼트 값
속
    Type     외곽선 종류                                    [LineType3]     Solid
성
                        표 44 OUTLINE 엘리먼트

                              SHADOW
   설명        글자 속성 : 그림자
부모 엘리먼트      CHARSHAPE, DRAWINGOBJECT, TEXTARTSHAPE
 엘리먼트 값
    Type     그림자 종류                           Drop (비연속) | Cont (연속)
속   Color    그림자 색                                [RGB-Color]
   OffsetX   그림자 간격 X                            -100% - 100%           10
성  OffsetY   그림자 간격 Y                            -100% - 100%           10
    Alpha
                        표 45 SHADOW 엘리먼트

                              EMBOSS
   설명        글자 속성 : 양각
부모 엘리먼트      CHARSHAPE
 엘리먼트 값
                         표 46 EMBOSS 엘리먼트

                             ENGRAVE
   설명        글자 속성 : 음각
부모 엘리먼트      CHARSHAPE
 엘리먼트 값
                        표 47 ENGRAVE 엘리먼트

                            SUPERSCRIPT
   설명        글자 속성 : 위 첨자
부모 엘리먼트      CHARSHAPE
 엘리먼트 값
                      표 48 SUPERSCRIPT 엘리먼트

                          SUBSCRIPT
   설명        글자 속성 : 아래 첨자
부모 엘리먼트      CHARSHAPE
 엘리먼트 값
                        표 49 SUBSCRIPT 엘리먼트

                                                                                67

--- page break ---

HWPML

     4.3.5. 탭 정보

                                     TABDEFLIST
          설명          탭 정의 리스트
        부모 엘리먼트       MAPPINGTABLE
        자식 엘리먼트       TABDEF
        속
           Count      탭 정의 개수                            0 이상의 정수
        성
                                표 50 TABDEFLIST 엘리먼트

                                    TABDEF
            설명           탭 정의 정보
          부모 엘리먼트        TABDEFLIST
          자식 엘리먼트        TABITEM
        속      Id        탭 정의 아이디                                 0 이상의 정수
           AutoTabLeft   문단 왼쪽 끝 자동탭 (내어 쓰기용 자동탭)                  true | false       false
        성 AutoTabRight   문단 오른쪽 끝 자동탭                              true | false       false

                                  표 51 TABDEF 엘리먼트

                                       TABITEM
            설명        탭 항목
         부모 엘리먼트      TABDEF
          엘리먼트 값
             Pos      탭의 위치                                 [hwpunit]
        속                                            Left (왼쪽) | Right (오른쪽) |
              Type    탭의 종류                                                           Left
                                                   Center (가운데) | Decimal (소수점)
        성                                                                             Soli
             Leader   채움 종류                                [LineType2]
                                                                                       d

                                 표 52 TABITEM 엘리먼트

                                   NUMBERINGLIST
          설명          번호 문단 모양 리스트
        부모 엘리먼트       MAPPINGTABLE
        자식 엘리먼트       NUMBERING
        속
           Count      번호 문단 모양의 갯수                       0 이상의 정수
        성
                               표 53 NUMBERINGLIST 엘리먼트

                                    NUMBERING
          설명          번호 문단 모양 정보
        부모 엘리먼트       NUMBERINGLIST
        자식 엘리먼트       PARAHEAD
        속    Id       번호 문단 모양 아이디                       1 이상의 정수
        성   Start     시작 번호                                                       1

                                표 54 NUMBERING 엘리먼트

68

--- page break ---

                                                                                    HWPML

                                  PARAHEAD
      설명             각 번호/글머리표 문단 머리의 정보
    부모 엘리먼트          NUMBERING, BULLET
                     문단 머리 문자열 포맷이다.
                     문자열 내 특정 문자에 제어코드(^)를 붙임으로써 한글에서 표시되는 번호
                     문단 머리의 포맷을 제어한다.
                      ^n : 레벨 경로를 표시한다. (예: 1.1.1.1.1.1.1)
    엘리먼트 값
                      ^N : 레벨 경로를 표시하며 마지막에 마침표를 하나 더 찍는다. (예:
                     1.1.1.1.1.1.1.)
                      ^레벨번호(1~7) : 해당 레벨에 해당하는 숫자 또는 문자 또는 기호를
                     표시한다.
        Level        수준                                         1 ~ 7
                                                           Left | Center |
      Alignment      문단의 정렬 종류                                               Left
                                                                Right
                     번호 너비를 실제 인스턴스 문자열의
     UseInstWidth                                           true | false    true
                     너비에 따를지 여부
     AutoIndent      자동 내어쓰기 여부                             true | false    true
속    WidthAdjust     번호 너비 보정값                                [hwpunit]        0
성                                                             percent |
    TextOffsetType   수준별 본문과의 거리 단위 종류                                     percent
                                                               hwpunit
      TextOffset     본문과의 거리                                                  50

                     번호 포맷
     NumFormat                                            [NumberType1]     Digit
                     (불릿 문단의 경우에는 사용되지 않는다.)

      CharShape      글자 모양 아이디 참조

                             표 55 PARAHEAD 엘리먼트

                                                                                       69

--- page break ---

HWPML

     4.3.6. 글머리표 정보

                                   BULLETLIST
          설명          글머리표 문단 모양 리스트
        부모 엘리먼트       MAPPINGTABLE
        자식 엘리먼트       BULLET
        속
           Count      글머리표 문단 모양의 개수                  0 이상의 정수
        성
                               표 56 BULLETLIST 엘리먼트

                                   BULLET
           설명         글머리표 문단 모양 정보
        부모 엘리먼트       BULLETLIST
         엘리먼트 값       PARAHEAD
        속     Id      글머리표 문단 모양 아이디                      1 이상의 정수
             Char     글머리표 문자
        성   Image                                          true | false   false

                                 표 57 BULLET 엘리먼트

70

--- page break ---

                                                                                        HWPML

4.3.7. 문단 모양 정보

                                    PARASHAPELIST
      설명             문단 모양 리스트
   부모 엘리먼트           MAPPINGTABLE
    엘리먼트 값           PARASHAPE
   속
       Count         문단 모양의 개수                         0 이상의 정수
   성
                             표 58 PARASHAPELIST 엘리먼트

                                    PARASHAPE
          설명                문단 모양
        부모 엘리먼트             PARASHAPELIST
        자식 엘리먼트             PARAMARGIN, PARABORDER
              Id            문단 모양 아이디                  0 이상의 정수
             Align          정렬 방식                    [AlignmentType1]         Justify
                                                     Baseline (글꼴기준) |
                                                         Top (위쪽) |
            VerAlign        세로 정렬                                            Baseline
                                                       Center (가운데) |
                                                        Bottom (아래)
                                                         None (없음) |
                                                       Outline (개요) |
          HeadingType       문단 머리 모양 종류                                       None
                                                       Number (번호) |
                                                        Bullet (글머리표)
                            번호 문단 또는 글머리표 문단
            Heading
                            모양 아이디 참조값
              Level         단계                              0 ~ 6               0
   속         TabDef         탭정의 아이디 참조값
                                                      KeepWord (단어) |
   성
         BreakLatinWord     줄 나눔 단위 (라틴 문자)         Hyphenation (하이픈) |      KeepWord
                                                       BreakWord (글자)
        BreakNonLatinWord   줄 나눔 단위 (비라틴 문자)        true (글자) | false (어절)     true
             Condense       공백 최소값                        0% ~ 75%               0
           WidowOrphan      외톨이줄 보호                      true | false         false
           KeepWithNext     다음 문단과 함께                    true | false         false
            KeepLines       문단 보호                        true | false         false
         PageBreakBefore    문단 앞에서 항상 쪽나눔                true | false         false
          FontLineHeight    글꼴에 어울리는 줄높이                 true | false         false
            SnapToGrid      편집 용지의 줄격자 사용                true | false          true
             LineWrap       한줄로 입력                    [LineWrapType]          Break
       AutoSpaceEAsianEng   한글과 영어 간격을 자동 조절             true | false          true
       AutoSpaceEAsianNum   한글과 숫자 간격을 자동 조절             true | false          true

                              표 59 PARASHAPE 엘리먼트

                                                                                           71

--- page break ---

HWPML

                                            PARAMARGIN
               설명               문단 여백
            부모 엘리먼트             PARASHAPE
             엘리먼트 값
                                들여쓰기/내어쓰기.                 hwpunit 또는 글자수.
                                숫자 다음에 "ch"가 붙어
                 Indent         있으면 글자수로 표시된                 n > 0 : 들여쓰기 n              0
                                것이고 그 외 숫자만으로 된                n == 0 : 보통
                                경우는 hwpunit단위이다.             n < 0 : 내어쓰기 n
                  Left          왼쪽 여백                      [hwpunit] 또는 글자수              0
                  Right         오른쪽 여백                     [hwpunit] 또는 글자수              0
                  Prev          문단 간격 위                    [hwpunit] 또는 글자수              0
        속         Next          문단 간격 아래                   [hwpunit] 또는 글자수              0
        성                                                    Percent (글자에 따라) |
                                                                Fixed (고정값) |
             LineSpacingType    줄 간격 종류                                                Percent
                                                         BetweenLines (여백만 지정) |
                                                                 AtLeast (최소)
                                                     0% ~ 500% (Type이 “Percent”일 때),
                                                     [hwpunit] 또는 글자수 (Type이
               LineSpacing      줄 간격 값               “Fixed”일 때),                       160
                                                     [hwpunit] 또는 글자수 (Type이
                                                     “BetweenLines”일 때)

                                     표 60 PARAMARGIN 엘리먼트

                                         PARABORDER
               설명              문단 테두리/배경
            부모 엘리먼트            PARASHAPE
             엘리먼트 값
                                                                 BORDERFILL엘리먼트의
              BorderFill       테두리/배경 모양 아이디 참조값
                                                                       Id속성 값
               OffsetLeft      문단 테두리 왼쪽 간격                           [hwpunit]
        속     OffsetRight      문단 테두리 오른쪽 간격                          [hwpunit]
        성      OffsetTop       문단 테두리 위쪽 간격                           [hwpunit]
             OffsetBottom      문단 테두리 아래쪽 간격                          [hwpunit]
                Connect        문단 테두리 연결 여부                          true | false        false
             IgnoreMargin      문단 테두리 여백 무시 여부                       true | false        false

                                     표 61 PARABORDER 엘리먼트

72

--- page break ---

                                                                            HWPML

4.3.8. 스타일 정보

                                     STYLELIST
     설명            스타일 리스트
   부모 엘리먼트         MAPPINGTABLE
   자식 엘리먼트         STYLE
   속
      Count        스타일 갯수                            0 이상의 정수
   성
                               표 62 STYLELIST 엘리먼트

                                       STYLE
      설명           스타일 정보
    부모 엘리먼트        STYLELIST
    엘리먼트 값
        Id         스타일 아이디
                                                        Para (문단 스타일) |
         Type      스타일 종류                                                 Para
                                                         Char (글자 스타일)
                   로컬 스타일 이름.
         Name
                   한글윈도우에서는 한글 스타일 이름.
       EngName     영문 스타일 이름.
                   문단 모양 아이디 참조값.
                                                        PARASHAPE엘리
       ParaShape   스타일의 종류가 문단인 경우 반드시 지정해야
   속                                                     먼트의 Id속성값
                   한다.
   성               글자 모양 아이디 참조값.
                                                        CHARSHAPE엘리
       CharShape   스타일의 종류가 글자인 경우 반드시 지정해야
                                                         먼트의 Id속성값
                   한다.
                   다음 스타일 아이디 참조값.
                   문단 스타일에서 사용자가 리턴키를 입력하여 다음
       NextStyle
                   문단으로 이동하였을때 적용될 문단 스타일을
                   지정한다.
        LangId     언어 아이디
       LockForm    양식모드에서 Style 보호하기

                                  표 63 STYLE 엘리먼트

                                                                                 73

--- page break ---

HWPML

     4.3.9. 메모 정보

                                         MEMOSHAPELIST
          설명             메모 리스트
        부모 엘리먼트          MAPPINGTABLE
        자식 엘리먼트          MEMO
        속
           Count         메모 개수                             0 이상의 정수
        성
                                 표 64 MEMOSHAPELIST 엘리먼트

                                             MEMO
            설명           메모
         부모 엘리먼트         MEMOSHAPELIST
          엘리먼트 값
                 Id      메모 아이디
              Width      메모의 선 두께                                        0
        속   LineType     메모의 선 종류
            LineColor    메모의 선의 색                          [RGB-Color]
        성    FillColor   메모의 색                             [RGB-Color]
           ActiveColor   메모가 활성화 되었을 때 색                   [RGB-Color]
           MemoType
                                        표 65 MEMO 엘리먼트

74

--- page break ---

                                                                            HWPML

5. 본문 엘리먼트
                                      BODY
    설명              본문
  부모 엘리먼트           HWPML
  자식 엘리먼트           SECTION
                                 표 66 BODY 엘리먼트

                                     SECTION
     설명             구역
  부모 엘리먼트           BODY
   엘리먼트 값           P
  속
       Id           구역 아이디
  성
                                표 67 SECTION 엘리먼트

                                        P
         설명           문단
      부모 엘리먼트         SECTION
       엘리먼트 값         TEXT
                                                         PARASHAPE
        ParaShape     문단 모양 아이디 참조값
                                                        엘리먼트의 Id 값
                                                           STYLE
          Style       문단 스타일 아이디 참조값
                                                        엘리먼트의 Id 값
  속                   개요 문단일 경우 문서내 유일한 아이디.
          InstId
  성                   개요 문단이 아닐 때는 사용되지 않는다.
                      현재 문단에서 쪽 나눔(CTRL-ENTER)이
        PageBreak                                         true | false   false
                      되었는지 여부
                      현재 문단에서 단 나눔(CTRL-SHIFT-ENTER)이
       ColumnBreak                                        true | false   false
                      되었는지 여부

                                  표 68 P 엘리먼트

                                         TEXT
    설명               컨트롤을 포함한 텍스트 문자열
  부모 엘리먼트            P
                     SECDEF, COLDEF, TABLE, PICTURE, CONTAINER, OLE,
                     EQUATION, TEXTART, LINE, RECTANGLE, ELLIPSE, ARC,
                     POLYGON, CURVE, CONNECTLINE, UNKNOWNOBJECT,
                     FIELDBEGIN, FIELDEND, BOOKMARK, HEADER, FOOTER,
  자식 엘리먼트
                     FOOTNOTE, ENDNOTE, AUTONUM, NEWNUM, PAGENUMCTRL,
                     PAGEHIDING, PAGENUM, INDEXMARK, COMPOSE, DUTMAL,
                     HIDDENCOMMENT, BUTTON, RADIOBUTTON, CHECKBUTTON,
                     COMBOBOX, EDIT, LISTBOX, SCROLLBAR
  속                                                 CHARSHAPE엘
       CharShape     글자 모양 아이디 참조값
  성                                                  리먼트의 Id값

                                 표 69 TEXT 엘리먼트

                                                                                 75

--- page break ---

HWPML

     5.1. 글자 엘리먼트

                                       CHAR
          설명        글자
        부모 엘리먼트     TEXT
                    문자열, TAB, LINEBREAK, HYPEN, NBSPACE, FWSPACE,
        자식 엘리먼트
                    TITLEMARK, MARKPENBEGIN, MARKPENEND
        속
            Style   스타일 아이디 참조값
        성
                                 표 70 CHAR 엘리먼트

                                  MARKPENBEGIN
           설명       형광펜 시작
        부모 엘리먼트     CHAR
         엘리먼트 값
        속
            Color   형광펜 색                             [RGB-Color]
        성
                             표 71 MARKPENBEGIN 엘리먼트

                                   MARKPENEND
           설명       형광펜 끝
        부모 엘리먼트     CHAR
         엘리먼트 값
                             표 72 MARKPENEND 엘리먼트

                                    TITLEMARK
           설명       제목 차례 표시
        부모 엘리먼트     CHAR
         엘리먼트 값
        속           제목 차례를 표시 = false
           Ignore                                     true | false
        성           차례 만들기 무시 = true

                              표 73 TITLEMARK 엘리먼트

                                        TAB
          설명        탭
        부모 엘리먼트     CHAR
        자식 엘리먼트
                                 표 74 TAB 엘리먼트

                                   LINEBREAK
          설명        강제 줄 나눔 (SHIFT-ENTER)
        부모 엘리먼트     CHAR
        자식 엘리먼트
                              표 75 LINEBREAK 엘리먼트

76

--- page break ---

                                         HWPML

                            HYPEN
  설명      하이픈 (CTRL-SHIFT-'-')
부모 엘리먼트   CHAR
자식 엘리먼트
                      표 76 HYPEN 엘리먼트

                         NBSPACE
  설명      묶음 빈 칸 (CTRL-ALT-SPACE)
부모 엘리먼트   CHAR
자식 엘리먼트
                     표 77 NBSPACE 엘리먼트

                         FWSPACE
  설명      고정폭 빈 칸 (ALT-SPACE)
부모 엘리먼트   CHAR
자식 엘리먼트
                     표 78 FWSPACE 엘리먼트

                                            77

--- page break ---

HWPML

     5.2. 구역 정의 엘리먼트

                                             SECDEF
               설명                   구역 정의
             부모 엘리먼트                TEXT
                                    PARAMETERSET, STARTNUMBER, HIDE, PAGEDEF,
             자식 엘리먼트                FOOTNOTESHAPE, ENDNOTESHAPE, PAGEBORDERFILL,
                                    MASTERPAGE, EXT_MASTERPAGE
                TextDirection       텍스트 방향                     0 (가로) | 1 (세로)     0
                                    동일한 페이지에서 서로 다른
               SpaceColumns                                      [hwpunit]
                                    단 사이의 간격
                  TabStop           기본 탭 간격                  [hwpunit] 또는 글자수     8000
                                    개요 번호 모양 아이디             NUMBERING엘리먼트의
                OutlineShape                                                       1
                                    참조값.                           Id속성값
                                                                    0 : off,
                  LineGrid          세로로 줄맞춤을 할지 여부                                 0
                                                           1~n : hwpunit 단위의 간격
        속
                                                                    0 : off,
        성         CharGrid          가로로 줄맞춤을 할지 여부                                 0
                                                           1~n : hwpunit 단위의 간격
                                    구역의 첫쪽에만 테두리를
                 FirstBorder                                    true | false      false
                                    표시할지 여부
                                    구역의 첫쪽에만 배경을
                  FirstFill                                     true | false      false
                                    표시할지 여부
             ExtMasterpageCount                                                    0
                MemoShapeId
            TextVerticalWidthHead

                                        표 79 SECDEF 엘리먼트

                                          PARAMETERSET
                          Parameter Set. 한글 내부에서 모듈 간 데이터 전달을 위해 사용하는
            설명            데이터 구조로 정의되었으나, 컨트롤 개체에 따라 특정 데이터를
                          Parameter Set 형태로 저장하는 경우가 있다. 극히 드물게 사용된다.
        부모 엘리먼트           SECDEF, ITEM, COLDEF, SHAPECOMPONENT, FORMOBJECT
        자식 엘리먼트           ITEM
        속   SetId         Parameter Set 아이디
        성  Count          Set안의 Item 개수                    0 이상의 정수

                                     표 80 PARAMETERSET 엘리먼트

                                         PARAMETERARRAY
                          Parameter Array. Parameter Set에서 배열을 표현하기 위한 구조. 자식
            설명
                          엘리먼트인 ITEM 엘리먼트가 배열의 각 원소에 해당한다.
        부모 엘리먼트           ITEM
        자식 엘리먼트           ITEM
        속
           Count          Set안의 Item 개수                       0 이상의 정수
        성
                                    표 81 PARAMETERARRAY 엘리먼트

78

--- page break ---

                                                                                                    HWPML

                                         ITEM
      설명             Parameter Set 및 Parameter Array에서 각 아이템을 표시하는 단위.
   부모 엘리먼트           PARAMETERSET, PARAMETERARRAY
    엘리먼트 값           문자열, PARAMETERSET, PARAMETERARRAY
      ItemId         Item 아이디
   속                                                 Bstr (문자열) | Integer (정수) |
   성   Type          Item의 종류                  Set (Parameter Set) | Array (Parameter Array)
                                                         | BinBata (binary data)

                                   표 82 ITEM 엘리먼트

5.2.1. 시작 번호 정보

                               STARTNUMBER
         설명            문서의 구역내 존재하는 그림, 표, 수식 및 쪽의 시작 번호 정보.
       부모 엘리먼트         SECDEF
       엘리먼트 값
                       구역 나눔으로 새 페이지가 생길                   Both (양쪽) | Even (짝수쪽) |
        PageStartsOn                                                                           Both
                       때의 페이지 번호 적용 옵션                            Odd (홀수쪽)
                                                                0 (앞 구역에 이어),
           Page        쪽 시작 번호                                                                  0
                                                               n (임의의 번호로 시작)
   속                                                            0 (앞 구역에 이어),
           Figure      그림 시작 번호                                                                 0
   성                                                           n (임의의 번호로 시작)
                                                                0 (앞 구역에 이어),
           Table       표 시작 번호                                                                  0
                                                               n (임의의 번호로 시작)
                                                                0 (앞 구역에 이어),
          Equation     수식 시작 번호                                                                 0
                                                               n (임의의 번호로 시작)

                                표 83 STARTNUMBER 엘리먼트

5.2.2. 감추기 정보

                                        HIDE
         설명            감추기 옵션들.
     부모 엘리먼트           SECDEF
      엘리먼트 값
         Header        첫쪽에만 머리말 감추기 여부                                    true | false         false
          Footer       첫쪽에만 꼬리말 감추기 여부                                    true | false         false
   속    MasterPage     첫쪽에만 바탕쪽 감추기 여부                                    true | false         false
          Border       첫쪽에만 테두리 감추기 여부                                    true | false         false
   성        Fill       첫쪽에만 배경 감추기 여부                                     true | false         false
       PageNumPos      첫쪽에만 쪽번호 감추기 여부                                    true | false         false
        EmptyLine      빈줄 감추기 여부                                          true | false         false

                                   표 84 HIDE 엘리먼트

                                                                                                       79

--- page break ---

HWPML

     5.2.3. 용지 설정 정보

                                         PAGEDEF
                          용지 설정 정보.
                          용지 크기의 디폴트는 'A4'(210 mm X 297 mm)이다.
             설명
                          이때, 용지 가로 크기인 210 mm = 59529 hwpunit 이나 한글97과의
                          호환을 위해 59528 hwpunit을 사용한다.
         부모 엘리먼트          SECDEF
         자식 엘리먼트          PAGEMARGIN
           Landscape      용지 방향                         0 (좁게) | 1 (넓게)   0
             Width        용지 가로 크기                        [hwpunit]     59528
        속    Height       용지 세로 크기                        [hwpunit]     84188
        성                                                    LeftOnly (한쪽 편집) |
           GutterType     제책 방법                              LeftRight (맞쪽 편집) |      LeftOnly
                                                            TopBottom (위로 넘기기)

                                       표 85 PAGEDEF 엘리먼트

                                          PAGEMARGIN
           설명             용지 여백.
        부모 엘리먼트           PAGEDEF
         엘리먼트 값
             Left         왼쪽 여백                                [hwpunit]              8504
            Right         오른쪽 여백                               [hwpunit]              8504
        속    Top          위 여백                                 [hwpunit]              5668
           Bottom         아래 여백                                [hwpunit]              4252
        성  Header         머리말 여백                               [hwpunit]              4252
           Footer         꼬리말 여백                               [hwpunit]              4252
            Gutter        제본 여백                                [hwpunit]               0

                                     표 86 PAGEMARGIN 엘리먼트

     5.2.4. 각주/미주 모양 정보

                                         FOOTNOTESHAPE
                                       ENDNOTESHAPE
           설명             각주모양 / 미주모양 정보
         부모 엘리먼트          SECDEF
                          AUTONUMFORMAT, NOTELINE, NOTESPACING, NOTENUMBERING,
         자식 엘리먼트
                          NOTEPLACEMENT
                            표 87 FOOTNOTESHAPE, ENDNOTESHAPE 엘리먼트

                                     AUTONUMFORMAT
           설명             번호 서식
         부모 엘리먼트          FOOTNOTESHAPE, ENDNOTESHAPE
         엘리먼트 값
             Type         번호 모양 종류                                   [NumberType2]       Digit
           UserChar       사용자 기호
        속 PrefixChar      앞 장식 문자
        성 SuffixChar      뒤 장식 문자                                                            )
                          각주 내용중 번호 코드의 모양을 윗첨자
            Superscript                                                true | false      false
                          형식으로 할지 여부
                                    표 88 AUTONUMFORMAT 엘리먼트

80

--- page break ---

                                                                                         HWPML

                                   NOTELINE
   설명              구분선
부모 엘리먼트            FOOTNOTESHAPE, ENDNOTESHAPE
 엘리먼트 값
                                       0 (구분선 없음) | 5cm (5cm) | 2cm (2cm) |
      Length       구분선 길이            Column/3 (단 크기의 1/3) | Column (단 크기) |
속                                           그 외 (사용자 지정 길이, hwpunit)
성     Type         구분선 종류                         [LineType1]                      Solid
      Width        구분선 굵기                         [LineWidth]                    0.12mm
      Color        구분선 색                          [RGB-Color]

                              표 89 NOTELINE 엘리먼트

                                NOTESPACING
     설명               여백
  부모 엘리먼트             FOOTNOTESHAPE, ENDNOTESHAPE
  엘리먼트 값
속   AboveLine         구분선 위 여백                                [hwpunit]       567(2mm)
    BelowLine         구분선 아래 여백                               [hwpunit]       567(2mm)
성 BetweenNotes        주석 사이 여백                                [hwpunit]       850(3mm)

                            표 90 NOTESPACING 엘리먼트

                                NOTENUMBERING
       설명           번호 매기기
    부모 엘리먼트         FOOTNOTESHAPE, ENDNOTESHAPE
     엘리먼트 값
                                            Continuous (앞 구역에 이어서) |
        Type        번호 매기기                OnSection (현재 구역부터 새로 시작) |         Continuous
속                                          OnPage (쪽마다 새로 시작, 각주 전용)
성                   시작 번호
     NewNumber      (Type이 "OnSection"일           1 이상의 정수                         1
                    때만 사용한다.)
                           표 91 NOTENUMBERING 엘리먼트

                                NOTEPLACEMENT
       설명           위치
    부모 엘리먼트         FOOTNOTESHAPE, ENDNOTESHAPE
     엘리먼트 값
                                    - 각주일 때
                                    EachColumn (각 단마다 따로 배열) |            - 각주일 때 :
                    한 페이지 내에서
                                    MergedColumn (통단으로 배열) |              EachColumn
                    미주/각주를 다단에
                                    RightMostColumn (가장 오른쪽 단에 배열)
속       Place       어떻게
                                                                          - 미주일 때 :
                    위치시킬지를
성                                   - 미주일 때                               EndOfDocume
                    표시한다.
                                    EndOfDocument (문서의 마지막) |             nt
                                    EndOfSection (구역의 마지막)
                    텍스트에 이어 바로
     BeneathText                               true | false                     false
                    출력할지 여부

                           표 92 NOTEPLACEMENT 엘리먼트

                                                                                            81

--- page break ---

HWPML

     5.2.5. 쪽 테두리/배경 정보

                                        PAGEBORDERFILL
              설명             쪽 테두리/배경
            부모 엘리먼트          SECDEF
            자식 엘리먼트          PAGEOFFSET
                                                         Both (양쪽) | Even (짝수쪽) |
                Type         종류                                                      Both
                                                                 Odd (홀수쪽)
                                                         BORDERFILL엘리먼트의
              BorderFill     테두리/배경 아이디 참조값
                                                                  Id속성값
        속                                                     true (본문 기준) |
             TextBorder      쪽 테두리 위치 기준                                             false
        성                                                      false (종이 기준)
             HeaderInside    머리말 포함                             true | false         false
             FooterInside    꼬리말 포함                             true | false         false
                                                          Paper (종이) | Page (쪽) |
               FillArea      채울 영역                                                  Paper
                                                               Border (테두리)

                                   표 93 PAGEBORDERFILL 엘리먼트

                                            PAGEOFFSET
           설명              테두리/배경 위치
        부모 엘리먼트            PAGEBORDERFILL
         엘리먼트 값
            Left           왼쪽 간격                              [hwpunit]         1417(5mm)
        속   Right          오른쪽 간격                             [hwpunit]         1417(5mm)
        성    Top           위쪽 간격                              [hwpunit]         1417(5mm)
           Bottom          아래쪽 간격                             [hwpunit]         1417(5mm)

                                     표 94 PAGEOFFSET 엘리먼트

82

--- page break ---

                                                                                             HWPML

5.2.6. 바탕쪽 정보

                                         MASTERPAGE
      설명                 바탕쪽
    부모 엘리먼트              SECDEF
    자식 엘리먼트              PARALIST
                                                          Both (양쪽) | Even (짝수쪽) |
           Type          종류                                                             Both
                                                                 Odd (홀수쪽)
        TextWidth        텍스트 영역의 폭
   속    TextHeight       텍스트 영역의 높이
   성                     각 비트가 해당 레벨의 텍스트에
        HasTextRef                                              true | false            false
                         대한 참조를 했는지 여부
                         각 비트가 해당 레벨의 번호에 대한
        HasNumRef                                               true | false            false
                         참조를 했는지 여부
                                    표 95 MASTERPAGE 엘리먼트

                                        PARALIST
           설명              문단 리스트
                           MASTERPAGE, EXT_MASTERPAGE, CELL, DRAWTEXT,
       부모 엘리먼트             CAPTION, HEADER, FOOTER, FOOTNOTE, ENDNOTE,
                           HIDDENCOMMENT
       자식 엘리먼트             P
        TextDirection      텍스트 방향                      0 (가로) | 1 (세로)                 0
          LineWrap         경계에서 줄나눔 방식                [LineWrapType]                 Break
   속                                               Top (위) | Center (가운데) |
          VertAlign        세로 정렬                                                     Top
   성                                                     Bottom (아래)
          LinkListID
        LinkListIDNext
                                     표 96 PARALIST 엘리먼트

5.2.7. 확장 바탕쪽 정보

                                       EXT_MASTERPAGE
         설명              확장 바탕쪽
       부모 엘리먼트           SECDEF
       자식 엘리먼트           PARALIST
                                                                 LastPage (마지막쪽) |
            Type         종류
                                                                 OptionalPage (임의쪽)
   속    PageNumber       (Type이 "OptionalPage"일 때) 임의의 쪽 번호        1 이상의 정수
   성                     기존 바탕쪽(양쪽, 홀수쪽, 짝수쪽)과
        PageDuplicate                                               true | false
                         확장바탕쪽 겹침
         PageFront       바탕쪽 앞으로 보내기                                true | false

                               표 97 EXT_MASTERPAGE 엘리먼트

                                                                                                83

--- page break ---

HWPML

     5.3. 단 정의 정보

                                        COLDEF
          설명           단 정의
        부모 엘리먼트        TEXT
        자식 엘리먼트        PARAMETERSET, COLUMNLINE, COLUMNTABLE
                                                         Newspaper (보통) |
                                                                                   Newspape
              Type     단 종류                         BalancedNewspaper (배분) |
                                                                                      r
                                                           Parallel (평행)
             Count     단 개수                                  1 ~ 255                  1
        속                                          Left (왼쪽부터) | Right (오른쪽부터) |
             Layout    단 방향 지정                                                       Left
        성                                                   Mirror (맞쪽)
                       단 너비 각자 지정 = false.
            SameSize                                       true | false              false
                       단 너비 동일 = true
                       단 사이 간격.
            SameGap                                         [hwpunit]                 0
                       SameSize가 "true"일 때만 사용.

                                    표 98 COLDEF 엘리먼트

                                        COLUMNLINE
           설명          단 구분선
        부모 엘리먼트        COLDEF
         엘리먼트 값
        속   Type       구분선 종류                                         [LineType]      Solid
            Width      구분선 굵기                                        [LineWidth]    0.12mm
        성   Color      구분선 색                                         [RGB-Color]

                                  표 99 COLUMNLINE 엘리먼트

                                       COLUMNTABLE
          설명           단 테이블
        부모 엘리먼트        COLDEF
        자식 엘리먼트        COLUMN
                                 표 100 COLUMNTABLE 엘리먼트

                                          COLUMN
           설명          단
        부모 엘리먼트        COLUMNTABLE
         엘리먼트 값
        속   Width      단의 폭                                      [hwpunit]
        성    Gap       단 사이 간격                                   [hwpunit]

                                   표 101 COLUMN 엘리먼트

84

--- page break ---

                                                                                             HWPML

5.4. 표

                                         TABLE
         설명              표
       부모 엘리먼트           TEXT
       엘리먼트 값            SHAPEOBJECT, INSIDEMARGIN, CELLZONELIST, ROW
                                                    Table (테이블은 나누지만 셀은 나누지 않는다)
          PageBreak      페이지 경계에서 나누는 방식                 | Cell ( 내의 텍스트도 나눈다)          Cell
                                                           | None (나누지 않는다)
                                                                                        tru
   속     RepeatHeader    제목행을 반복할지 여부                        true | false
                                                                                         e
   성      RowCount       행 갯수
          ColCount       열 갯수
                         셀 간격
          CellSpacing                                         [hwpunit]                  0
                         (HTML의 셀간격과 동일 의미)
          BorderFill     테두리/배경 아이디 참조값             BORDERFILL엘리먼트의 Id속성값

                                   표 102 TABLE 엘리먼트

                                       SHAPEOBJECT
           설명            개체 속성
                         TABLE, PICTURE, LINE, RECTANGLE, ELLIPSE, ARC, POLYGON,
       부모 엘리먼트
                         CURVE, OLE, EQUATION
       자식 엘리먼트           SIZE, POSITION, OUTSIDEMARGIN, CAPTION, SHAPECOMMENT
                         문서내 각 개체에 대한 고유
             InstId
                         아이디
            ZOrder       z-order                                                    0
                                                    None (없음) | Figure (그림) |
         NumberingType   이 개체가 속하는 번호 범주                                           None
                                                    Table (표) | Equation (수식)
                         오브젝트 주위를 텍스트가
                         어떻게 흘러갈지 지정하는 옵션
   속       TextWrap      (<POSITION                     [TextWrapType]
   성                     TreatAsChar="false">일 때만
                         사용)
                         오브젝트의 좌/우 어느쪽에
                                                       BothSides (양쪽) |
                         글을 배치할지 지정하는 옵션
                                                        LeftOnly (왼쪽) |
           TextFlow      (TextWrap이 "Square" 또는                                 BothSides
                                                      RightOnly (오른쪽) |
                         "Tight" 또는 "Through"일때만
                                                       LargestOnly (큰쪽)
                         사용)
             Lock        개체 선택 가능 여부                      true | false             false

                                표 103 SHAPEOBJECT 엘리먼트

                                                                                                85

--- page break ---

HWPML

                                              SIZE
           설명             크기
        부모 엘리먼트           SHAPEOBJECT
         엘리먼트 값
                          오브젝트 폭.
                          WidthRelTo의 값에 따라 다음과 같은 다른
                          단위를 뜻한다.
                           - “Paper” : 종이의 몇 %
              Width                                            [hwpunit]
                           - “Page” : 본문 영역의 몇 %
                           - “Column” : 단의 몇 %
                           - “Para” : 문단의 몇 %
                           - “Absolute” : 고정값 hwpunit
                          오브젝트의 높이.
                          HeightRelTo의 값에 따라 다음과 같은 다른
                          단위를 뜻한다.
        속     Height                                           [hwpunit]
                           - "Paper" : 종이의 몇 %
        성                  - "Page" : 본문 영역의 몇 %
                           - "Absolute" : 고정값 hwpunit
                                                            Paper (종이에 따라) |
                          오브젝트 폭의 기준.                        Page (쪽에 따라) |
                                                                               Absolut
            WidthRelTo    “Para” 값은 <POSITION VertRelTo =   Column (단에 따라) |
                                                                                  e
                          "Para">일 때만 가능함.                   Para (문단에 따라) |
                                                             Absolute (고정값)
                                                            Paper (종이에 따라) |
                                                                               Absolut
            HeightRelTo   오브젝트 높이의 기준                        Page (쪽에 따라) |
                                                                                  e
                                                             Absolute (고정값)
              Protect     크기 보호 여부                             true | false     false

                                        표 104 SIZE 엘리먼트

86

--- page break ---

                                                                                          HWPML

                                      POSITION
       설명               위치
    부모 엘리먼트             SHAPEOBJECT
     엘리먼트 값
      TreatAsChar       글자처럼 취급 여부                                   true | false
                        줄 간격에 영향을 줄지 여부.
     AffectLSpacing                                                  true | false    false
                        (TreatAsChar가 "true"일 때만 사용)
                                                                     Paper (종이) |
                        세로 위치의 기준.
       VertRelTo                                                      Page (쪽) |
                        (TreatAsChar가 "false"일 때만 사용)
                                                                       Para (문단)
                        VertRelTo에 대한 상대적인 배열 방식.                      Top (위) |
                        VertRelTo의 값에 따라 가능한 범위가                    Center (가운데) |
       VertAlign        제한된다.                                       Bottom (아래) |
                        (VertRelTo가 "Para"인 경우 “Para" 값만             Inside (안쪽) |
                        가능, 나머지 경우에는 모든 값 가능.)                      Outside (바깥쪽)
                                                                     Paper (종이) |
                        가로 위치의 기준.                                    Page (쪽) |
       HorzRelTo
                        (TreatAsChar가 "false"일 때만 사용)                Column (단) |
                                                                       Para (문단)
                                                                      Left (왼쪽) |
속                                                                   Center (가운데) |
       HorzAlign        HorzRelTo에 대한 상대적인 배열 방식.                    Right (오른쪽) |
성                                                                    Inside (안쪽) |
                                                                    Outside (바깥쪽)
                        VertRelTo와 VertAlign을 기준점으로 한
       VertOffset                                                     [hwpunit]       0
                        상대적인 오프셋 값
                        HorzRelTo와 HorzAlign을 기준점으로 한
       HorzOffset                                                     [hwpunit]       0
                        상대적인 오프셋 값
                        오브젝트의 세로 위치를 본문 영역으로
      FlowWithText      제한할지 여부.                                     true | false    false
                        (VertRelTo가 "Para"일 때만 사용)
                        다른 오브젝트와 겹치는 것을 허용할지 여부.
                        (TreatAsChar가 "false"일 때만 사용,
      AllowOverlap                                                   true | false    false
                        FlowWithText가 "true"이면 언제나 false로
                        간주함)
                        개체와 조판부호를 항상 같은 쪽에 놓기 속성.
                        SHAPEOBJECT엘리먼트가 TABLE엘리먼트의
    HoldAnchorAndSO                                                  true | false    false
                        자식인 경우에만 적용되며, 한글 빌드번호
                        5.7.5.2992부터 추가된 속성임.

                              표 105 POSITION 엘리먼트

                                  OUTSIDEMARGIN
   설명               바깥 여백
부모 엘리먼트             SHAPEOBJECT
 엘리먼트 값
    Left            오브젝트의 바깥 왼쪽 여백                      [hwpunit]    TABLE일때 : 283
속                                                                    PICTURE일때 : 0
    Right           오브젝트의 바깥 오른쪽 여백                     [hwpunit]
                                                                     EQUATION일때 : 56
성    Top            오브젝트의 바깥 위쪽 여백                      [hwpunit]    그리기개체일때 : 0
   Bottom           오브젝트의 바깥 아래쪽 여백                     [hwpunit]    OLE일때 : ?

                            표 106 OUTSIDEMARGIN 엘리먼트

                                                                                             87

--- page break ---

HWPML

                                           CAPTION
          설명            캡션
        부모 엘리먼트         SHAPEOBJECT
        자식 엘리먼트         PARALIST
                                                             Left | Right |
              Side      방향                                                          Left
                                                             Top | Bottom
                        캡션 폭에 마진을 포함할지 여부.
            FullSize                                          true | false          false
        속               (Side가 가로 방향일 때만 사용)
        성               캡션 폭.
             Width
                        (Side가 세로 방향일 때만 사용)
               Gap      캡션과 틀 사이 간격                                                  850
            LastWidth   텍스트의 최대 길이 (=개체의 폭)
                                      표 107 CAPTION 엘리먼트

                                         SHAPECOMMENT
           설명           주석
        부모 엘리먼트         SHAPEOBJECT
         엘리먼트 값         문자열
                                 표 108 SHAPECOMMENT 엘리먼트

                                         INSIDEMARGIN
           설명           안쪽 여백
        부모 엘리먼트         TABLE, PICTURE
         엘리먼트 값
            Left        왼쪽 여백                              [hwpunit]
        속   Right       오른쪽 여백                             [hwpunit]          TABLE일때 : 141
        성    Top        위쪽 여백                              [hwpunit]          PICTURE일때 : 0
           Bottom       아래쪽 여백                             [hwpunit]

                                   표 109 INSIDEMARGIN 엘리먼트

                                         CELLZONELIST
          설명            셀존 리스트
        부모 엘리먼트         TABLE
        자식 엘리먼트         CELLZONE
        속
           Count        셀존의 개수                               0 이상의 정수
        성
                                   표 110 CELLZONELIST 엘리먼트

                                           CELLZONE
              설명           셀존
          부모 엘리먼트          CELLZONELIST
          엘리먼트 값
           StartRowAddr    셀존의 Row의 시작주소
        속 StartColAddr     셀존의 Column의 시작주소
           EndRowAddr      셀존의 Row의 끝주소
        성   EndColAddr     셀존의 Column의 끝주소
             BorderFill    테두리/배경 아이디
                                    표 111 CELLZONE 엘리먼트

88

--- page break ---

                                                                               HWPML

                                    ROW
  설명             표의 행
부모 엘리먼트          TABLE
자식 엘리먼트          CELL
                             표 112 ROW 엘리먼트

                                    CELL
  설명             표의 셀
부모 엘리먼트          ROW
자식 엘리먼트          CELLMARGIN, PARALIST
   Name          셀 필드 이름
                 셀 주소 (Column, 맨 왼쪽 셀이 0부터 시작하여
     ColAddr
                 1씩 증가)
                 셀 주소 (Row, 맨 위쪽 셀이 0부터 시작하여 1씩
    RowAddr
                 증가)
    ColSpan      열의 병합 갯수                                                  1
    RowSpan      행의 병합 갯수                                                  1
      Width      셀의 폭                                   [hwpunit]
속    Height      셀의 높이                                  [hwpunit]
성    Header      제목 셀인지 여부                             true | false       false
                 테이블의 기본 셀마진이 아닌 독자적인 마진을
    HasMargin                                          true | false       false
                 사용할지 여부
     Protect     사용자 편집을 막을지 여부                        true | false       false
     Editable    읽기 전용 상태에서도 수정 가능한지 여부                true | false       false
                 마지막으로 업데이트된 이후 사용자가 내용을
      Dirty                                            true | false       false
                 변경했는지 여부
                                                     BORDERFILL엘리
    BorderFill   테두리/배경 아이디 참조값
                                                      먼트의 Id속성값

                             표 113 CELL 엘리먼트

                                CELLMARGIN
   설명            셀 여백
부모 엘리먼트          CELL
 엘리먼트 값
    Left         왼쪽 여백                            [hwpunit]           0
속   Right        오른쪽 여백                           [hwpunit]           0
성    Top         위쪽 여백                            [hwpunit]           0
   Bottom        아래쪽 여백                           [hwpunit]           0

                          표 114 CELLMARGIN 엘리먼트

                                                                                  89

--- page break ---

HWPML

     5.5. 그림

                                           PICTURE
          설명             그림
        부모 엘리먼트          TEXT
                         SHAPEOBJECT, SHAPECOMPONENT, LINESHAPE, IMAGERECT,
        자식 엘리먼트
                         IMAGECLIP, EFFECTS, INSIDEMARGIN, IMAGE
        속
            Reverse                                         true | false           false
        성
                                    표 115 PICTURE 엘리먼트

                                      SHAPECOMPONENT
           설명             개체 요소 속성
         부모 엘리먼트          PICTURE, DRAWINGOBJECT
         자식 엘리먼트          PARAMETERSET, ROTATIONINFO, RENDERINGINFO
                          하이퍼링크 속성.
               HRef
                          하이퍼링크 필드 컨트롤의 Command속성과 동일.
                XPos      개체가 속한 그룹내에서의 X offset                     [hwpunit]             0
                YPos      개체가 속한 그룹내에서의 Y offset                     [hwpunit]             0
            GroupLevel    그룹으로 묶인 횟수                                                       0
        속    OriWidth     개체 생성시 최초 폭                                [hwpunit]
        성    OriHeight    개체 생성시 최초 높이                               [hwpunit]
             CurWidth     개체의 현재 폭                                   [hwpunit]
            CurHeight     개체의 현재 높이                                  [hwpunit]
             HorzFlip     좌/우로 뒤집어진 상태인지 여부                         true | false       false
              VertFlip    상/하로 뒤집어진 상태인지 여부                         true | false       false
               InstID
                                표 116 SHAPECOMPONENT 엘리먼트

                                      ROTATIONINFO
           설명            개체 회전
        부모 엘리먼트          SHAPECOMPONENT
         엘리먼트 값
        속   Angle        회전각                                                               0
           CenterX       회전중심의 x좌표                   개체 좌표계-개체 width의 반
        성  CenterY       회전중심의 y좌표                   개체 좌표계-개체 height의 반

                                  표 117 ROTATIONINFO 엘리먼트

                                       RENDERINGINFO
          설명             랜더링 정보
        부모 엘리먼트          SHAPECOMPONENT
        자식 엘리먼트          TRANSMATRIX, SCAMATRIX, ROTMATRIX
                                 표 118 RENDERINGINFO 엘리먼트

90

--- page break ---

                                                                                      HWPML

                                     TRANSMATRIX
                                       SCAMATRIX
                                       ROTMATRIX
                   Translation Matrix, Scaling Matrix, Rotation Matrix.
    설명
                   E7 ~ E9는 (0,0,1)로 일정하므로 생략.
부모 엘리먼트            RENDERINGINFO
 엘리먼트 값
    E1             9 X 9 행렬의 첫번째 요소 (0,0)
    E2             9 X 9 행렬의 두번째 요소 (0,1)
속   E3             9 X 9 행렬의 세번째 요소 (0,2)
성   E4             9 X 9 행렬의 네번째 요소 (1,0)
    E5             9 X 9 행렬의 다섯번째 요소 (1,1)
    E6             9 X 9 행렬의 여섯번째 요소 (1,2)
                   표 119 TRANSMATRIX, SCAMATRIX, ROTMATRIX 엘리먼트

                                   LINESHAPE
   설명               테두리 선 모양
부모 엘리먼트             PICTURE, DRAWINGOBJECT, OLE
 엘리먼트 값
    Color           선 색상                                    [RGB-Color]
    Width           선 굵기                                     [hwpunit]
    Style           선 종류                                    [LineType1]       Solid
                    선 끝 모양.
                                                          Round (둥근 모양) |
      EndCap        그림 일때는 "Round", 그리기개 체들일 때는                                Flat
                                                           Flat (편평한 모양)
                    "Flat"이 디폴트.
속
    HeadStyle       화살표 시작 모양                              [ArrowType]       Normal
성    TailStyle      화살표 끝 모양                               [ArrowType]       Normal
    HeadSize        화살표 시작 크기                               [ArrowSize]     SmallSmall
     TailSize       화살표 끝 크기                                [ArrowSize]     SmallSmall
                                                          Normal | Outer
    OutlineStyle                                                             Normal
                                                              | Inner
       Alpha
                               표 120 LINESHAPE 엘리먼트

                                       IMAGERECT
   설명              이미지 좌표 정보
부모 엘리먼트            PICTURE
 엘리먼트 값
    X0
    Y0
속   X1             이미지의 테두리 사각형의 좌표 (최초 그림 삽입시 크기)
성   Y1             (X0, Y0) (X1, Y1) (X2, Y2) (X3, Y3)
    X2
    Y2
                               표 121 IMAGERECT 엘리먼트

                                                                                         91

--- page break ---

HWPML

                                        IMAGECLIP
           설명          이미지 자르기 정보
        부모 엘리먼트        PICTURE
         엘리먼트 값
            Left
        속    Top
                       (Left, Top) (Right, Bottom) : 자르기한후 사각형
        성   Right
           Bottom
                                  표 122 IMAGECLIP 엘리먼트

                                         EFFECTS
          설명           이미지 효과 정보
        부모 엘리먼트        PICTURE
        자식 엘리먼트        SHADOWEFFECT, GLOW, SOFTEDGE, REFLECTION
                                   표 123 EFFECTS 엘리먼트

                                      SHADOWEFFECT
             설명            그림자 효과
          부모 엘리먼트          EFFECTS
          자식 엘리먼트          EFFECTSCOLOR
               Style       그림자 스타일 (바깥쪽/안쪽)
              Alpha        시작 투명도
              Radius       흐릿하게
             Direction     방향 각도
        속    Distance      거리
            AlignStyle     그림자 정렬
        성     SkewX        기울기 각도(X)
              SkewY        기울기 각도(Y)
              ScaleX       확대 비율(X)
              ScaleY       확대 비율(Y)
           RotationStyle   도형과 함께 그림자 회전
                                표 124 SHADOWEFFECT 엘리먼트

                                          GLOW
          설명           네온 효과
        부모 엘리먼트        EFFECTS
        자식 엘리먼트        EFFECTSCOLOR
        속  Alpha       시작 투명도
        성  Radius      네온 크기

                                    표 125 GLOW 엘리먼트

                                  SOFTEDGE
           설명          부드러운 가장자리 효과
        부모 엘리먼트        EFFECTS
         엘리먼트 값
        속
           Radius      부드러운 가장자리 크기
        성
                                  표 126 SOFTEDGE 엘리먼트

92

--- page break ---

                                                       HWPML

                                   REFLECTION
      설명           반사 효과
  부모 엘리먼트          EFFECTS
  엘리먼트 값
     AlignStyle    그림자 정렬
      Radius       흐릿하게
     Direction     방향 각도
     Distance      거리
      SkewX        기울기 각도(X)
      SkewY        기울기 각도(Y)
속     ScaleX       확대 비율(X)
성     ScaleY       확대 비율(Y)
   RotationStyle   도형과 함께 그림자 회전
    StartAlpha     시작 투명도
     StartPos      시작 위치
     EndAlpha      끝 투명도
      EndPos       끝 위치
   FadeDirection   오프셋 방향
                          표 127 REFLECTION 엘리먼트

                                  EFFECTSCOLOR
     설명            도형엔진용 색상
  부모 엘리먼트          SHADOWEFFECT, GLOW
  자식 엘리먼트          COLOREFFECT
       Type        컬러 종류. rgb, cmyk, scheme, system.
   SchemeIndex     Scheme Index
   SystemIndex     System Index
    PresetIndex    Preset Index
      ColorR       red 컬러
      ColorG       green 컬러
      ColorB       blue 컬러
속     ColorC       cyan 컬러
      ColorM       magenta 컬러
성     ColorY       yellow 컬러
      ColorK       black 컬러
     ColorSCR      ScRed 컬러
     ColorSCG      ScGreen 컬러
     ColorSCB      ScBlue 컬러
      ColorH       Hue 컬러
      ColorS       Sat 컬러
      ColorL       Lum 컬러
                         표 128 EFFECTSCOLOR 엘리먼트

                            COLOREFFECT
   설명          도형엔진용 색상 효과
부모 엘리먼트        EFFECTSCOLOR
 엘리먼트 값
속   Type       ColorEffect Type
성   Value      Value

                          표 129 COLOREFFECT 엘리먼트

                                                          93

--- page break ---

HWPML

     5.6. 그리기 개체

                                            DRAWINGOBJECT
            설명                그리기 개체 공통 속성
          부모 엘리먼트             LINE, RECTANGLE, ELLIPSE, ARC, POLYGON, CURVE
                              SHAPECOMPONENT, LINESHAPE, FILLBRUSH, DRAWTEXT,
          자식 엘리먼트
                              SHADOW
                                      표 130 DRAWINGOBJECT 엘리먼트

                                              DRAWTEXT
            설명                그리기 개체 글상자용 텍스트
          부모 엘리먼트             DRAWINGOBJECT
          자식 엘리먼트             TEXTMARGIN, PARALIST
                              텍스트 문자열의 최대 폭.
         속      LastWidth
                              (보통 그리기 개체의 가로 크기와 동일)
         성       Name         글상자 이름
                Editable      편집 가능 여부                                   true | false    false

                                        표 131 DRAWTEXT 엘리먼트

                                             TEXTMARGIN
            설명                글상자 텍스트 여백
         부모 엘리먼트              DRAWTEXT
          엘리먼트 값
             Left             왼쪽 여백                              [hwpunit]          238(1mm)
         속   Right            오른쪽 여백                             [hwpunit]          238(1mm)
         성    Top             위쪽 여백                              [hwpunit]          238(1mm)
            Bottom            아래쪽 여백                             [hwpunit]          238(1mm)

                                       표 132 TEXTMARGIN 엘리먼트

     5.6.1. 선

                                                 LINE
             설명                 그리기 개체 : 선
           부모 엘리먼트              TEXT
           자식 엘리먼트              SHAPEOBJECT, DRAWINGOBJECT
              StartX            시작점 X좌표                                      [hwpunit]
              StartY            시작점 Y좌표                                      [hwpunit]

         속     EndX             끝점 X좌표                                       [hwpunit]
               EndY             끝점 Y좌표                                       [hwpunit]
         성                      처음 생성시 수직 또는 수평선일때, 선의 방향이
                IsReverseHV     언제나 오른쪽(위쪽)으로 잡힘으로 인한                    true | false    false
                                현상때문에, 방향을 바로 잡아주기 위한 플래그
                                          표 133 LINE 엘리먼트

94

--- page break ---

                                                                                       HWPML

5.6.2. 사각형

                                        RECTANGLE
      설명                그리기 개체 : 사각형
    부모 엘리먼트             TEXT
    자식 엘리먼트             SHAPEOBJECT, DRAWINGOBJECT
                                                        직각은 0, 둥근모양은 20, 반원은
            Ratio       사각형 모서리 곡률 (%).                 50, 그외는 적당한 값을 %단위로
                                                                사용한다.
             X0
    속
             Y0
    성        X1         (X0, Y0) (X1, Y1) (X2, Y2)
             Y1         (X3, Y3) : 사각형의 좌표
             X2
             Y2
                                    표 134 RECTANGLE 엘리먼트

5.6.3. 타원

                                          ELLIPSE
          설명                 그리기 개체 : 타원
        부모 엘리먼트              TEXT
        자식 엘리먼트              SHAPEOBJECT, DRAWINGOBJECT
                             호(ARC)로 바뀌었을 때, 타원의
            IntervalDirty    선상에 존재하는 호의 두 점 사이를                   true | false    false
                             다시 계산해야 할 필요가 있는지 여부
        HasArcProperty       호(ARC)로 바뀌었는지 여부                      true | false    false
                                                                 Normal (호 모양) |
                                                                                   Norma
              ArcType        호(ARC)의 종류                             Pie (부채꼴) |
                                                                                     l
                                                                  Chord (활 모양)
              CenterX        중심 좌표의 X값
              CenterY        중심 좌표의 Y값
               Axis1X        제1축 X좌표값
    속          Axis1Y        제1축 Y좌표값
    성          Axis2X        제2축 X좌표값
               Axis2Y        제2축 Y좌표값
              Start1X
              Start1Y
               End1X
               End1Y
                             interval of curve (effective only
              Start2X
                             when it is an arc)
              Start2Y
              End2X
              End2Y
                                      표 135 ELLIPSE 엘리먼트

                                                                                           95

--- page break ---

HWPML

     5.6.4. 호

                                             ARC
            설명            그리기 개체 : 호
          부모 엘리먼트         TEXT
          자식 엘리먼트         SHAPEOBJECT, DRAWINGOBJECT
                                                          Normal (호 모양) |
                 Type     종류                                Pie (부채꼴) |     Normal
                                                           Chord (활 모양)
                CenterX   타원의 중심 좌표 X값
         속
                CenterY   타원의 중심 좌표 Y값
         성       Axis1X   제1축 X좌표값
                 Axis1Y   제1축 Y좌표값
                 Axis2X   제2축 X좌표값
                 Axis2Y   제2축 Y좌표값
                                       표 136 ARC 엘리먼트

     5.6.5. 다각형

                                           POLYGON
            설명            그리기 개체 : 다각형
          부모 엘리먼트         TEXT
          자식 엘리먼트         SHAPEOBJECT, DRAWINGOBJECT, POINT
                                     표 137 POLYGON 엘리먼트

                                            POINT
            설명            다각형 개체를 이루는 포인트
         부모 엘리먼트          POLYGON, OUTLINEDATA
          엘리먼트 값
         속    X           X 좌표
         성    Y           Y 좌표

                                      표 138 POINT 엘리먼트

     5.6.6. 곡선

                                            CURVE
            설명            그리기 개체 : 곡선
          부모 엘리먼트         TEXT
          자식 엘리먼트         SHAPEOBJECT, DRAWINGOBJECT, SEGMENT
                                      표 139 CURVE 엘리먼트

96

--- page break ---

                                                                                    HWPML

                               SEGMENT
      설명           곡선 개체를 이루는 세그먼트
   부모 엘리먼트         CURVE
    엘리먼트 값
       Type        세그먼트의 타입                             Line (직선) | Curve (곡선)   Curve
   속    X1         세그먼트의 시작점 X좌표
        Y1         세그먼트의 시작점 Y좌표
   성    X2         세그먼트의 끝점 X좌표
        Y2         세그먼트의 끝점 Y좌표
                                표 140 SEGMENT 엘리먼트

5.6.7. 연결선

                                    CONNECTLINE
            설명             그리기 개체 : 연결선
       부모 엘리먼트             TEXT
        엘리먼트 값             SHAPEOBJECT, DRAWINGOBJECT
              Type
             StartX
             StartY
   속         EndX
             EndY
   성    StartSubjectID
       StartSubjectIndex
         EndSubjectID
       EndSubjectIndex
                              표 141 CONNECTLINE 엘리먼트

5.7. Unknown Object

                                   UNKNOWNOBJECT
      설명           Unknown Object
   부모 엘리먼트         TEXT
    엘리먼트 값         SHAPEOBJECT, DRAWINGOBJECT
       Ctrlid      ID
        X0
        Y0
   속    X1
        Y1
   성               (X0, Y0) (X1, Y1) (X2, Y2) (X3, Y3) : master 좌표
        X2
        Y2
        X3
        Y3
                            표 142 UNKNOWNOBJECT 엘리먼트

                                                                                         97

--- page break ---

HWPML

     5.8. 양식 객체

                                        FORMOBJECT
             설명           양식 개체 공통 속성
                          BUTTON, RADIOBUTTON, CHECKBUTTON, COMBOBOX, EDIT,
         부모 엘리먼트
                          LISTBOX, SCROLLBAR
         자식 엘리먼트          PARAMETERSET, FORMCHARSHAPE, BUTTONSET
             Name         이름
           ForeColor      전경색
           BackColor      배경색
          GroupName       그룹 이름
        속                 탭키로 객체들을 이동할 때 해당 객체에 머물 수
             TabStop                                           true | false true
                          있는지를 결정.
        성    TapOrder     탭키 이동 순서
              Enabled     활성화 여부.                              true | false true
            BorderType    경계선 종류                                              0
            DrawFrame                                            true | false   true
             Printable    출력 가능 여부                               true | false   true

                                  표 143 FORMOBJECT 엘리먼트

                                      FORMCHARSHAPE
             설명            양식 개체의 글자 속성
          부모 엘리먼트          FORMOBJECT
          엘리먼트 값
            CharShape      글자 모양                                                 0
        속 FollowContext                                          true | false   false
        성    AutoSize      자동 크기 설정 여부                           true | false   false
            WordWrap       줄 내림 여부                               true | false   false
                                표 144 FORMCHARSHAPE 엘리먼트

                                        BUTTONSET
                설명            버튼 개체 공통 속성
            부모 엘리먼트           FORMOBJECT
             엘리먼트 값
                Caption
        속        Value
            RadioGroupName
        성       TriState                                       true | false
               BackStyle
                                  표 145 BUTTONSET 엘리먼트

                                          BUTTON
          설명             양식 개체 : 버튼
        부모 엘리먼트          TEXT
        자식 엘리먼트          SHAPEOBJECT, FORMOBJECT
                                    표 146 BUTTON 엘리먼트

98

--- page break ---

                                                                           HWPML

5.8.1. 라디오 버튼

                               RADIOBUTTON
      설명        양식 개체 : 라디오 버튼
    부모 엘리먼트     TEXT
    자식 엘리먼트     SHAPEOBJECT, FORMOBJECT
                            표 147 RADIOBUTTON 엘리먼트

5.8.2. 체크 버튼

                               CHECKBUTTON
      설명        양식 개체 : 체크 버튼
    부모 엘리먼트     TEXT
    자식 엘리먼트     SHAPEOBJECT, FORMOBJECT
                            표 148 CHECKBUTTON 엘리먼트

5.8.3. 콤보 박스

                                    COMBOBOX
        설명            양식 개체 : 콤보 박스
     부모 엘리먼트          TEXT
     자식 엘리먼트          SHAPEOBJECT, FORMOBJECT
      ListBoxRows
   속 ListBoxWidth     넓이
   성      Text        내용
       EditEnable     텍스트로 수정 가능 여부
                             표 149 COMBOBOX 엘리먼트

5.8.4. 에디트

                                       EDIT
         설명             양식 개체 : 에디트
     부모 엘리먼트            TEXT
     자식 엘리먼트            SHAPEOBJECT, FORMOBJECT, EDITTEXT
        MultiLine       다중 라인
      PasswordChar      비밀번호
       MaxLength        최대 길이
   속    ScrollBars      스크롤바 활성화
   성 TabKeyBehavior
         Number                                             true | false
        ReadOnly        읽기 전용 여부                            true | false
        AlignText
                                표 150 EDIT 엘리먼트

                            EDITTEXT
       설명       에디트의 텍스트 데이터
    부모 엘리먼트     EDIT
     엘리먼트 값     문자열
                              표 151 EDITTEXT 엘리먼트

                                                                              99

--- page break ---

HWPML

  5.8.5. 리스트 박스

                                          LISTBOX
             설명           양식 개체 : 리스트 박스
         부모 엘리먼트          TEXT
          엘리먼트 값          SHAPEOBJECT, FORMOBJECT
        속     Text
           ItemHeight
        성   TopIndex
                                    표 152 LISTBOX 엘리먼트

  5.8.6. 스크롤바

                                         SCROLLBAR
              설명           양식 개체 : 스크롤바
          부모 엘리먼트          TEXT
           엘리먼트 값          SHAPEOBJECT, FORMOBJECT
               Delay
            LargeChange
            SmallChange
        속        Min
        성       Max
                Page
               Value
                Type
                                   표 153 SCROLLBAR 엘리먼트

100

--- page break ---

                                                                                     HWPML

5.9. 묶음 객체

                                     CONTAINER
     설명             묶음 개체
   부모 엘리먼트          TEXT
                    SHAPEOBJECT, SHAPECOMPONENT, CONTAINER, LINE,
   자식 엘리먼트          RECTANGLE, ELLIPSE, ARC, POLYGON, CURVE, CONNECTLINE,
                    PICTURE, OLE
                              표 154 CONTAINER 엘리먼트

5.10. OLE 객체

                                       OLE
      설명             OLE
    부모 엘리먼트          TEXT
    자식 엘리먼트          SHAPEOBJECT, SHAPECOMPONENT, LINESHAPE
                                                      Unknown | Embedded |
       ObjetType     OLE개체의 종류                            Link | Static |
                                                         Equation (수식 ocx)
        ExtentX
   속    ExtentY
   성    BinItem
                                                       Content | ThumbNail |
       DrawAspect
                                                          Icon | DocPrint
       HasMoniker                                           true | false          false
       EqBaseLine
                                 표 155 OLE 엘리먼트

5.11. 한글 97 수식

                                     EQUATION
      설명            한글 97 수식
   부모 엘리먼트          TEXT
    엘리먼트 값          SHAPEOBJECT, SCRIPT
     LineMode       차지 범위                           true (줄 단위) | false (글자 단위)   false
   속 BaseUnit       수식 글자 크기                                [hwpunit]             1000
     TextColor      글자 색상                                  [RGB-Color]              0
   성 BaseLine
      Version
                               표 156 EQUATION 엘리먼트

                                  SCRIPT
      설명            수식 스트립트 내용
   부모 엘리먼트          EQUATION
    엘리먼트 값          한글 97 수식 스크립트 문자열이 온다.
                                표 157 SCRIPT 엘리먼트

                                                                                          101

--- page break ---

HWPML

  5.12. 글맵시

                                          TEXTART
          설명             글맵시
        부모 엘리먼트          TEXT
        자식 엘리먼트          TEXTARTSHAPE, OUTLINEDATA
            Text         내용
             X0
             Y0
        속    X1
             Y1
        성                (X0, Y0) (X1, Y1) (X2, Y2) (X3, Y3) : master 좌표
             X2
             Y2
             X3
             Y3
                                      표 158 TEXTART 엘리먼트

                                          TEXTARTSHAPE
             설명            TEXTARTSHAPE
          부모 엘리먼트          TEXTART
          자식 엘리먼트          SHADOW
            FontName       폰트 이름
            FontStyle      폰트 스타일                                                    Regular
             FontType      HFDT_TTF or HFDT_HFT                      ttf | htf          ttf
        속   TextShape      0(shape 1) ~ 39(shape 40)                  0 ~ 39             0
        성 LineSpacing                                               50 ~ 500           120
           CharSpacing                                              50 ~ 500           100
                                                             Left | Right | Center
              Align                                                                   Left
                                                                 | Full | Table

                                   표 159 TEXTARTSHAPE 엘리먼트

                                          OUTLINEDATA
          설명             외각선
        부모 엘리먼트          TEXTART
        자식 엘리먼트          POINT
        속
           Count         외각선 포인트 개수                             0 이상의 정수
        성
                                    표 160 OUTLINEDATA 엘리먼트

102

--- page break ---

                                                                         HWPML

5.13. 필드 시작

                               FIELDBEGIN
      설명         필드 시작
   부모 엘리먼트       TEXT
    엘리먼트 값
        Type     필드의 종류                           [FieldType]
       Name      필드 이름
       InstId    인스턴스 아이디 (문서내 고유 아이디)
   속  Editable   읽기 전용 상태에서도 수정 가능한지 여부           true | false   true
   성    Dirty    필드 내용이 수정되었는지 여부                 true | false   false
      Property   기타 속성
                 명령 문자열 (각각의 필드 종류마다
      Command
                 처리해야할 고유 정보)

                          표 161 FIELDBEGIN 엘리먼트

5.14. 필드 끝

                                FIELDEND
      설명         필드 끝
   부모 엘리먼트       TEXT
    엘리먼트 값
   속   Type      필드의 종류                           [FieldType]
      Editable   읽기 전용 상태에서도 수정 가능한지 여부           true | false   true
   성 Property    기타 속성

                          표 162 FIELDEND 엘리먼트

5.15. 책갈피

                                BOOKMARK
      설명         책갈피
   부모 엘리먼트       TEXT
    엘리먼트 값
   속
       Name      책갈피 이름
   성
                          표 163 BOOKMARK 엘리먼트

                                                                           103

--- page break ---

HWPML

  5.16. 머리말, 꼬리말

                                              HEADER
                                              FOOTER
              설명              머리말, 꼬리말
            부모 엘리먼트           TEXT
            자식 엘리먼트           PARALIST
                                                            Both (양쪽) | Even (짝수쪽) |
        속    ApplyPageType    머리말/꼬리말이 적용될 페이지 종류                                       Both
                                                                   Odd (홀수쪽)
        성      SeriesNum      구역내의 일련번호

                                     표 164 HEADER, FOOTER 엘리먼트

  5.17. 각주, 미주

                                             FOOTNOTE
                                             ENDNOTE
          설명              각주, 미주
        부모 엘리먼트           TEXT
        자식 엘리먼트           PARALIST
                                표 165 FOOTNOTE, ENDNOTE 엘리먼트

  5.18. 자동 번호, 새 번호

                                             AUTONUM
                                          NEWNUM
              설명             자동 번호, 새 번호
            부모 엘리먼트          TEXT
            자식 엘리먼트          AUTONUMFORMAT
               Number        번호                                                          1
        속                                                 Page (쪽) | Footnote (각주) |
                                                         Endnote (미주) | Figure (그림) |
        성    NumberType      번호의 종류
                                                          Table (표) | Equation (수식) |
                                                                  TotalPage

                                 표 166 AUTONUM, NEWNUM 엘리먼트

  5.19. 홀/짝수 조정

                                           PAGENUMCTRL
              설명             홀/짝수 조정
            부모 엘리먼트          TEXT
            엘리먼트 값
        속                                                  Both (양쪽) | Even (짝수쪽) |
             PageStartsOn    홀/짝수 구분                                                    Both
        성                                                         Odd (홀수쪽)

                                     표 167 PAGENUMCTRL 엘리먼트

104

--- page break ---

                                                                                        HWPML

5.20. 감추기

                                  PAGEHIDING
         설명           감추기
     부모 엘리먼트          TEXT
      엘리먼트 값
       HideHeader     머리말 감추기 여부                         true | false         false
       HideFooter     꼬리말 감추기 여부                         true | false         false
   속 HideMasterPage   바탕쪽 감추기 여부                         true | false         false
   성   HideBorder     테두리 감추기 여부                         true | false         false
         HideFill     배경 감추기 여부                          true | false         false
      HidePageNum     쪽 번호 감추기 여부                        true | false         false

                             표 168 PAGEHIDING 엘리먼트

5.21. 쪽번호 위치

                                   PAGENUM
       설명           쪽번호 위치
    부모 엘리먼트         TEXT
     엘리먼트 값
                                          None (없음) | TopLeft (왼쪽 위) |
                                    TopCenter (가운데 위) | TopRight (오른쪽 위) |
                                             BottomLeft (왼쪽 아래) |
                                           BottomCenter (가운데 아래) |
   속      Pos       번호 위치                                                      TopLeft
                                            BottomRight (오른쪽 아래) |
   성                                         OutsideTop (바깥쪽 위) |
                                           OutsideBottom (바깥쪽 아래) |
                                    InsideTop (안쪽 위) | InsideBottom (안쪽 아래)
       FormatType   번호 모양 종류                    [NumberType1]                   Digit
        SideChar    줄 표                                                          -

                             표 169 PAGENUM 엘리먼트

                                                                                          105

--- page break ---

HWPML

  5.22. 찾아보기 표식

                                      INDEXMARK
                      찾아보기 표식.
            설명
                      찾아보기 표식을 달수 있고, 찾아보기 만들기에서 참조한다.
        부모 엘리먼트       TEXT
        자식 엘리먼트       KEYFIRST, KEYSECOND
                                표 170 INDEXMARK 엘리먼트

                                  KEYFIRST
           설명         찾아보기에 사용할 첫번째 키워드
        부모 엘리먼트       INDEXMARK
         엘리먼트 값       키워드 내용
                                 표 171 KEYFIRST 엘리먼트

                                 KEYSECOND
           설명         찾아보기에 사용할 두번째 키워드
        부모 엘리먼트       INDEXMARK
         엘리먼트 값       키워드 내용
                                표 172 KEYSECOND 엘리먼트

  5.23. 글자 겹침

                                       COMPOSE
              설명           글자 겹침
          부모 엘리먼트          TEXT
          자식 엘리먼트          COMPCHARSHAPE
             CircleType    테투리 타입
        속     CharSize     내부 글자 크기
        성 ComposeType      겹치기 종류
           CharShapeSize   글자 모양 갯수
                                 표 173 COMPOSE 엘리먼트

                                COMPCHARSHAPE
           설명         글자 겹침 글자 모양
        부모 엘리먼트       COMPOSE
         엘리먼트 값
        속
           ShapeID    글자 모양 식별자
        성
                              표 174 COMPCHARSHAPE 엘리먼트

106

--- page break ---

                                                                                    HWPML

5.24. 덧말

                                     DUTMAL
     설명          덧말
   부모 엘리먼트       TEXT
   자식 엘리먼트       MAINTEXT, SUBTEXT
     PosType     덧말의 위치                            Top (위쪽) | Bottom (아래쪽)    Top

   속 SizeRatio
      Option
   성  StyleNo
       Align     정렬 기준                                [AlignmentType1]       Center

                               표 175 DUTMAL 엘리먼트

                             MAINTEXT
      설명         덧말넣기의 본말 내용
   부모 엘리먼트       DUTMAL
    엘리먼트 값       덧말넣기의 본말 내용이 온다.
                              표 176 MAINTEXT 엘리먼트

                             SUBTEXT
      설명         덧말넣기의 덧말 내용
   부모 엘리먼트       DUTMAL
    엘리먼트 값       덧말넣기의 덧말 내용이 온다.
                               표 177 SUBTEXT 엘리먼트

5.25. 숨은 설명

                                 HIDDENCOMMENT
     설명          숨은 설명
   부모 엘리먼트       TEXT
   자식 엘리먼트       PARALIST
                            표 178 HIDDENCOMMENT 엘리먼트

                                                                                      107

--- page break ---

HWPML

  6. 부가 정보 엘리먼트
                                        TAIL
          설명         헤더와 본문정보 외에 기타 정보를 담고 있다.
        부모 엘리먼트      HWPML
        자식 엘리먼트      BINDATASTORAGE, SCRIPTCODE, XMLTEMPLATE
                                  표 179 TAIL 엘리먼트

                               BINDATASTORAGE
          설명         바이너리 데이타 저장소
        부모 엘리먼트      TAIL
        자식 엘리먼트      BINDATA
                             표 180 BINDATASTORAGE 엘리먼트

                                    BINDATA
           설명        바이너리 데이타
        부모 엘리먼트      BINDATASTORAGE
         엘리먼트 값      문자열
             Id      바이너리 데이타 아이디
        속   Size     바이너리 데이타 크기
        성 Encoding   인코딩 방식. Base64로 고정되어 있다.              Base64       Base64
          Compress   압축 여부                               true | false    true

                                 표 181 BINDATA 엘리먼트

                                     SCRIPTCODE
           설명        스크립트 코드
        부모 엘리먼트      TAIL
         엘리먼트 값      SCRIPTHEADER, SCRIPTSOURCE, PRESCRIPT, POSTSCRIPT
        속   Type     스크립트 코드 종류                          JScript       JScript
        성  Version   스크립트 코드 버전

                               표 182 SCRIPTCODE 엘리먼트

                                    SCRIPTHEADER
           설명        스크립트 코드 헤더
        부모 엘리먼트      SCRIPTCODE
         엘리먼트 값      문자열
                              표 183 SCRIPTHEADER 엘리먼트

                                    SCRIPTSOURCE
           설명        스크립트 코드 소스
        부모 엘리먼트      SCRIPTCODE
         엘리먼트 값      문자열
                              표 184 SCRIPTSOURCE 엘리먼트

108

--- page break ---

                                                                         HWPML

                                PRESCRIPT
                               POSTSCRIPT
  설명           PRESCRIPT, POSTSCRIPT
부모 엘리먼트        SCRIPTCODE
자식 엘리먼트        SCRIPTFUNCTION
속
   Count       SCRIPTFUNCTION의 개수                0 이상의 정수
성
                     표 185 PRESCRIPT, POSTSCRIPT 엘리먼트

                             SCRIPTFUNCTION
  설명           스크립트 코드 함수
부모 엘리먼트        PRESCRIPT, POSTSCRIPT
 엘리먼트 값        문자열
                       표 186 SCRIPTFUNCTION 엘리먼트

                              XMLTEMPLATE
  설명           XML 템플릿
부모 엘리먼트        TAIL
자식 엘리먼트        SCHEMA, INSTANCE
                        표 187 XMLTEMPLATE 엘리먼트

                                 SCHEMA
   설명          XML 스키마
부모 엘리먼트        XMLTEMPLATE
 엘리먼트 값        문자열
                           표 188 SCHEMA 엘리먼트

                                INSTANCE
   설명          XML 인스턴스
부모 엘리먼트        XMLTEMPLATE
 엘리먼트 값        문자열
                          표 189 INSTANCE 엘리먼트

                          COMPATIBLEDOCUMENT
      설명            호환문서
    부모 엘리먼트         HEAD
    자식 엘리먼트         LAYOUTCOMPATIBILITY
속
    TargetProgram                               None | Hwp70 | Word   None
성
                     표 190 COMPATIBLEDOCUMENT 엘리먼트

                                                                             109

--- page break ---

HWPML

                                        LAYOUTCOMPATIBILITY
                      설명                      서식
                    부모 엘리먼트                   COMPATIBLEDOCUMENT
                    엘리먼트 값
                    ApplyFontWeightToBold                          true | false   false
                      UseInnerUnderline                            true | false   false
                     FixedUnderlineWidth                           true | false   false
                     DoNotApplyStrikeout                           true | false   false
                    UseLowercaseStrikeout                          true | false   false
                   ExtendLineheightToOffset                        true | false   false
                    TreatQuotationAsLatin                          true | false   false
                DoNotAlignWhitespaceOnRight                        true | false   false
                  DoNotAdjustWordInJustify                         true | false   false
                    BaseCharUnitOnEAsian                           true | false   false
             BaseCharUnitOfIndentOnFirstChar                       true | false   false
                    AdjustLineheightToFont                         true | false   false
             AdjustBaselineInFixedLinespacing                      true | false   false
               ExcludeOverlappingParaSpacing                       true | false   false
                 ApplyNextspacingOfLastPara                        true | false   false
                ApplyAtLeastToPercent100Pct                        true | false   false
              DoNotApplyAutoSpaceEAsianEng                         true | false   false
              DoNotApplyAutoSpaceEAsianNum                         true | false   false
        속
                AdjustParaBorderfillToSpacing                      true | false   false
        성   ConnectParaBorderfillOfEqualBorder                     true | false   false
             AdjustParaBorderOffsetWithBorder                      true | false   false
            ExtendLineheightToParaBorderOffset                     true | false   false
                  ApplyParaBorderToOutside                         true | false   false
                 BaseLinespacingOnLinegrid                         true | false   false
                 ApplyCharSpacingToCharGrid                        true | false   false
               DoNotApplyGridInHeaderfooter                        true | false   false
                  ExtendHeaderfooterToBody                         true | false   false
             AdjustEndnotePositionToFootnote                       true | false   false
                    DoNotApplyImageEffect                          true | false   false
                  DoNotApplyShapeComment                           true | false   false
                DoNotAdjustEmptyAnchorLine                         true | false   false
                   OverlapBothAllowOverlap                         true | false   false
               DoNotApplyVertOffsetOfForward                       true | false   false
               ExtendVertLimitToPageMargins                        true | false   false
                   DoNotHoldAnchorOfTable                          true | false   false
             DoNotFormattingAtBeneathAnchor                        true | false   false
             DoNotApplyExtensionCharCompose                        true | false   false

                                 표 191 LAYOUTCOMPATIBILITY 엘리먼트

110

--- page break ---

                              HWPML

변경 사항 이력
  §   revision 1.2:20141105
      - 구 5.0 내용 삭제

      - 회사 주소 정보 수정

  §   revision 1.1:20110124
      - 저작권 내용 수정

      - 내용 중 일부 오타 수정

  §   revision 1.0:20100701
      - 한글 문서 파일 형식 공개

                                111

--- page break ---

HWPML

112

--- page break ---
```
