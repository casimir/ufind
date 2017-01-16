// https://tools.ietf.org/html/rfc1345

pub struct Digraph {
    pub sequence: [char; 2],
    pub character: char,
    pub description: &'static str,
}

pub const TABLE: &'static [Digraph; 1334] =
    &[Digraph {
          sequence: ['S', 'P'],
          character: '\u{0020}',
          description: "SPACE",
      },
      Digraph {
          sequence: ['N', 'b'],
          character: '\u{0023}',
          description: "NUMBER SIGN",
      },
      Digraph {
          sequence: ['D', 'O'],
          character: '\u{0024}',
          description: "DOLLAR SIGN",
      },
      Digraph {
          sequence: ['A', 't'],
          character: '\u{0040}',
          description: "COMMERCIAL AT",
      },
      Digraph {
          sequence: ['<', '('],
          character: '\u{005b}',
          description: "LEFT SQUARE BRACKET",
      },
      Digraph {
          sequence: ['/', '/'],
          character: '\u{005c}',
          description: "REVERSE SOLIDUS",
      },
      Digraph {
          sequence: [')', '>'],
          character: '\u{005d}',
          description: "RIGHT SQUARE BRACKET",
      },
      Digraph {
          sequence: ['\'', '>'],
          character: '\u{005e}',
          description: "CIRCUMFLEX ACCENT",
      },
      Digraph {
          sequence: ['\'', '!'],
          character: '\u{0060}',
          description: "GRAVE ACCENT",
      },
      Digraph {
          sequence: ['(', '!'],
          character: '\u{007b}',
          description: "LEFT CURLY BRACKET",
      },
      Digraph {
          sequence: ['!', '!'],
          character: '\u{007c}',
          description: "VERTICAL LINE",
      },
      Digraph {
          sequence: ['!', ')'],
          character: '\u{007d}',
          description: "RIGHT CURLY BRACKET",
      },
      Digraph {
          sequence: ['\'', '?'],
          character: '\u{007e}',
          description: "TILDE",
      },
      Digraph {
          sequence: ['N', 'S'],
          character: '\u{00a0}',
          description: "NO-BREAK SPACE",
      },
      Digraph {
          sequence: ['!', 'I'],
          character: '\u{00a1}',
          description: "INVERTED EXCLAMATION MARK",
      },
      Digraph {
          sequence: ['C', 't'],
          character: '\u{00a2}',
          description: "CENT SIGN",
      },
      Digraph {
          sequence: ['P', 'd'],
          character: '\u{00a3}',
          description: "POUND SIGN",
      },
      Digraph {
          sequence: ['C', 'u'],
          character: '\u{00a4}',
          description: "CURRENCY SIGN",
      },
      Digraph {
          sequence: ['Y', 'e'],
          character: '\u{00a5}',
          description: "YEN SIGN",
      },
      Digraph {
          sequence: ['B', 'B'],
          character: '\u{00a6}',
          description: "BROKEN BAR",
      },
      Digraph {
          sequence: ['S', 'E'],
          character: '\u{00a7}',
          description: "SECTION SIGN",
      },
      Digraph {
          sequence: ['\'', ':'],
          character: '\u{00a8}',
          description: "DIAERESIS",
      },
      Digraph {
          sequence: ['C', 'o'],
          character: '\u{00a9}',
          description: "COPYRIGHT SIGN",
      },
      Digraph {
          sequence: ['-', 'a'],
          character: '\u{00aa}',
          description: "FEMININE ORDINAL INDICATOR",
      },
      Digraph {
          sequence: ['<', '<'],
          character: '\u{00ab}',
          description: "LEFT-POINTING DOUBLE ANGLE QUOTATION MARK",
      },
      Digraph {
          sequence: ['N', 'O'],
          character: '\u{00ac}',
          description: "NOT SIGN",
      },
      Digraph {
          sequence: ['-', '-'],
          character: '\u{00ad}',
          description: "SOFT HYPHEN",
      },
      Digraph {
          sequence: ['R', 'g'],
          character: '\u{00ae}',
          description: "REGISTERED SIGN",
      },
      Digraph {
          sequence: ['\'', 'm'],
          character: '\u{00af}',
          description: "MACRON",
      },
      Digraph {
          sequence: ['D', 'G'],
          character: '\u{00b0}',
          description: "DEGREE SIGN",
      },
      Digraph {
          sequence: ['+', '-'],
          character: '\u{00b1}',
          description: "PLUS-MINUS SIGN",
      },
      Digraph {
          sequence: ['2', 'S'],
          character: '\u{00b2}',
          description: "SUPERSCRIPT TWO",
      },
      Digraph {
          sequence: ['3', 'S'],
          character: '\u{00b3}',
          description: "SUPERSCRIPT THREE",
      },
      Digraph {
          sequence: ['\'', '\''],
          character: '\u{00b4}',
          description: "ACUTE ACCENT",
      },
      Digraph {
          sequence: ['M', 'y'],
          character: '\u{00b5}',
          description: "MICRO SIGN",
      },
      Digraph {
          sequence: ['P', 'I'],
          character: '\u{00b6}',
          description: "PILCROW SIGN",
      },
      Digraph {
          sequence: ['.', 'M'],
          character: '\u{00b7}',
          description: "MIDDLE DOT",
      },
      Digraph {
          sequence: ['\'', ','],
          character: '\u{00b8}',
          description: "CEDILLA",
      },
      Digraph {
          sequence: ['1', 'S'],
          character: '\u{00b9}',
          description: "SUPERSCRIPT ONE",
      },
      Digraph {
          sequence: ['-', 'o'],
          character: '\u{00ba}',
          description: "MASCULINE ORDINAL INDICATOR",
      },
      Digraph {
          sequence: ['>', '>'],
          character: '\u{00bb}',
          description: "RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK",
      },
      Digraph {
          sequence: ['1', '4'],
          character: '\u{00bc}',
          description: "VULGAR FRACTION ONE QUARTER",
      },
      Digraph {
          sequence: ['1', '2'],
          character: '\u{00bd}',
          description: "VULGAR FRACTION ONE HALF",
      },
      Digraph {
          sequence: ['3', '4'],
          character: '\u{00be}',
          description: "VULGAR FRACTION THREE QUARTERS",
      },
      Digraph {
          sequence: ['?', 'I'],
          character: '\u{00bf}',
          description: "INVERTED QUESTION MARK",
      },
      Digraph {
          sequence: ['A', '!'],
          character: '\u{00c0}',
          description: "LATIN CAPITAL LETTER A WITH GRAVE",
      },
      Digraph {
          sequence: ['A', '\''],
          character: '\u{00c1}',
          description: "LATIN CAPITAL LETTER A WITH ACUTE",
      },
      Digraph {
          sequence: ['A', '>'],
          character: '\u{00c2}',
          description: "LATIN CAPITAL LETTER A WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['A', '?'],
          character: '\u{00c3}',
          description: "LATIN CAPITAL LETTER A WITH TILDE",
      },
      Digraph {
          sequence: ['A', ':'],
          character: '\u{00c4}',
          description: "LATIN CAPITAL LETTER A WITH DIAERESIS",
      },
      Digraph {
          sequence: ['A', 'A'],
          character: '\u{00c5}',
          description: "LATIN CAPITAL LETTER A WITH RING ABOVE",
      },
      Digraph {
          sequence: ['A', 'E'],
          character: '\u{00c6}',
          description: "LATIN CAPITAL LETTER AE",
      },
      Digraph {
          sequence: ['C', ','],
          character: '\u{00c7}',
          description: "LATIN CAPITAL LETTER C WITH CEDILLA",
      },
      Digraph {
          sequence: ['E', '!'],
          character: '\u{00c8}',
          description: "LATIN CAPITAL LETTER E WITH GRAVE",
      },
      Digraph {
          sequence: ['E', '\''],
          character: '\u{00c9}',
          description: "LATIN CAPITAL LETTER E WITH ACUTE",
      },
      Digraph {
          sequence: ['E', '>'],
          character: '\u{00ca}',
          description: "LATIN CAPITAL LETTER E WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['E', ':'],
          character: '\u{00cb}',
          description: "LATIN CAPITAL LETTER E WITH DIAERESIS",
      },
      Digraph {
          sequence: ['I', '!'],
          character: '\u{00cc}',
          description: "LATIN CAPITAL LETTER I WITH GRAVE",
      },
      Digraph {
          sequence: ['I', '\''],
          character: '\u{00cd}',
          description: "LATIN CAPITAL LETTER I WITH ACUTE",
      },
      Digraph {
          sequence: ['I', '>'],
          character: '\u{00ce}',
          description: "LATIN CAPITAL LETTER I WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['I', ':'],
          character: '\u{00cf}',
          description: "LATIN CAPITAL LETTER I WITH DIAERESIS",
      },
      Digraph {
          sequence: ['D', '-'],
          character: '\u{00d0}',
          description: "LATIN CAPITAL LETTER ETH (Icelandic)",
      },
      Digraph {
          sequence: ['N', '?'],
          character: '\u{00d1}',
          description: "LATIN CAPITAL LETTER N WITH TILDE",
      },
      Digraph {
          sequence: ['O', '!'],
          character: '\u{00d2}',
          description: "LATIN CAPITAL LETTER O WITH GRAVE",
      },
      Digraph {
          sequence: ['O', '\''],
          character: '\u{00d3}',
          description: "LATIN CAPITAL LETTER O WITH ACUTE",
      },
      Digraph {
          sequence: ['O', '>'],
          character: '\u{00d4}',
          description: "LATIN CAPITAL LETTER O WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['O', '?'],
          character: '\u{00d5}',
          description: "LATIN CAPITAL LETTER O WITH TILDE",
      },
      Digraph {
          sequence: ['O', ':'],
          character: '\u{00d6}',
          description: "LATIN CAPITAL LETTER O WITH DIAERESIS",
      },
      Digraph {
          sequence: ['*', 'X'],
          character: '\u{00d7}',
          description: "MULTIPLICATION SIGN",
      },
      Digraph {
          sequence: ['O', '/'],
          character: '\u{00d8}',
          description: "LATIN CAPITAL LETTER O WITH STROKE",
      },
      Digraph {
          sequence: ['U', '!'],
          character: '\u{00d9}',
          description: "LATIN CAPITAL LETTER U WITH GRAVE",
      },
      Digraph {
          sequence: ['U', '\''],
          character: '\u{00da}',
          description: "LATIN CAPITAL LETTER U WITH ACUTE",
      },
      Digraph {
          sequence: ['U', '>'],
          character: '\u{00db}',
          description: "LATIN CAPITAL LETTER U WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['U', ':'],
          character: '\u{00dc}',
          description: "LATIN CAPITAL LETTER U WITH DIAERESIS",
      },
      Digraph {
          sequence: ['Y', '\''],
          character: '\u{00dd}',
          description: "LATIN CAPITAL LETTER Y WITH ACUTE",
      },
      Digraph {
          sequence: ['T', 'H'],
          character: '\u{00de}',
          description: "LATIN CAPITAL LETTER THORN (Icelandic)",
      },
      Digraph {
          sequence: ['s', 's'],
          character: '\u{00df}',
          description: "LATIN SMALL LETTER SHARP S (German)",
      },
      Digraph {
          sequence: ['a', '!'],
          character: '\u{00e0}',
          description: "LATIN SMALL LETTER A WITH GRAVE",
      },
      Digraph {
          sequence: ['a', '\''],
          character: '\u{00e1}',
          description: "LATIN SMALL LETTER A WITH ACUTE",
      },
      Digraph {
          sequence: ['a', '>'],
          character: '\u{00e2}',
          description: "LATIN SMALL LETTER A WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['a', '?'],
          character: '\u{00e3}',
          description: "LATIN SMALL LETTER A WITH TILDE",
      },
      Digraph {
          sequence: ['a', ':'],
          character: '\u{00e4}',
          description: "LATIN SMALL LETTER A WITH DIAERESIS",
      },
      Digraph {
          sequence: ['a', 'a'],
          character: '\u{00e5}',
          description: "LATIN SMALL LETTER A WITH RING ABOVE",
      },
      Digraph {
          sequence: ['a', 'e'],
          character: '\u{00e6}',
          description: "LATIN SMALL LETTER AE",
      },
      Digraph {
          sequence: ['c', ','],
          character: '\u{00e7}',
          description: "LATIN SMALL LETTER C WITH CEDILLA",
      },
      Digraph {
          sequence: ['e', '!'],
          character: '\u{00e8}',
          description: "LATIN SMALL LETTER E WITH GRAVE",
      },
      Digraph {
          sequence: ['e', '\''],
          character: '\u{00e9}',
          description: "LATIN SMALL LETTER E WITH ACUTE",
      },
      Digraph {
          sequence: ['e', '>'],
          character: '\u{00ea}',
          description: "LATIN SMALL LETTER E WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['e', ':'],
          character: '\u{00eb}',
          description: "LATIN SMALL LETTER E WITH DIAERESIS",
      },
      Digraph {
          sequence: ['i', '!'],
          character: '\u{00ec}',
          description: "LATIN SMALL LETTER I WITH GRAVE",
      },
      Digraph {
          sequence: ['i', '\''],
          character: '\u{00ed}',
          description: "LATIN SMALL LETTER I WITH ACUTE",
      },
      Digraph {
          sequence: ['i', '>'],
          character: '\u{00ee}',
          description: "LATIN SMALL LETTER I WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['i', ':'],
          character: '\u{00ef}',
          description: "LATIN SMALL LETTER I WITH DIAERESIS",
      },
      Digraph {
          sequence: ['d', '-'],
          character: '\u{00f0}',
          description: "LATIN SMALL LETTER ETH (Icelandic)",
      },
      Digraph {
          sequence: ['n', '?'],
          character: '\u{00f1}',
          description: "LATIN SMALL LETTER N WITH TILDE",
      },
      Digraph {
          sequence: ['o', '!'],
          character: '\u{00f2}',
          description: "LATIN SMALL LETTER O WITH GRAVE",
      },
      Digraph {
          sequence: ['o', '\''],
          character: '\u{00f3}',
          description: "LATIN SMALL LETTER O WITH ACUTE",
      },
      Digraph {
          sequence: ['o', '>'],
          character: '\u{00f4}',
          description: "LATIN SMALL LETTER O WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['o', '?'],
          character: '\u{00f5}',
          description: "LATIN SMALL LETTER O WITH TILDE",
      },
      Digraph {
          sequence: ['o', ':'],
          character: '\u{00f6}',
          description: "LATIN SMALL LETTER O WITH DIAERESIS",
      },
      Digraph {
          sequence: ['-', ':'],
          character: '\u{00f7}',
          description: "DIVISION SIGN",
      },
      Digraph {
          sequence: ['o', '/'],
          character: '\u{00f8}',
          description: "LATIN SMALL LETTER O WITH STROKE",
      },
      Digraph {
          sequence: ['u', '!'],
          character: '\u{00f9}',
          description: "LATIN SMALL LETTER U WITH GRAVE",
      },
      Digraph {
          sequence: ['u', '\''],
          character: '\u{00fa}',
          description: "LATIN SMALL LETTER U WITH ACUTE",
      },
      Digraph {
          sequence: ['u', '>'],
          character: '\u{00fb}',
          description: "LATIN SMALL LETTER U WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['u', ':'],
          character: '\u{00fc}',
          description: "LATIN SMALL LETTER U WITH DIAERESIS",
      },
      Digraph {
          sequence: ['y', '\''],
          character: '\u{00fd}',
          description: "LATIN SMALL LETTER Y WITH ACUTE",
      },
      Digraph {
          sequence: ['t', 'h'],
          character: '\u{00fe}',
          description: "LATIN SMALL LETTER THORN (Icelandic)",
      },
      Digraph {
          sequence: ['y', ':'],
          character: '\u{00ff}',
          description: "LATIN SMALL LETTER Y WITH DIAERESIS",
      },
      Digraph {
          sequence: ['A', '-'],
          character: '\u{0100}',
          description: "LATIN CAPITAL LETTER A WITH MACRON",
      },
      Digraph {
          sequence: ['a', '-'],
          character: '\u{0101}',
          description: "LATIN SMALL LETTER A WITH MACRON",
      },
      Digraph {
          sequence: ['A', '('],
          character: '\u{0102}',
          description: "LATIN CAPITAL LETTER A WITH BREVE",
      },
      Digraph {
          sequence: ['a', '('],
          character: '\u{0103}',
          description: "LATIN SMALL LETTER A WITH BREVE",
      },
      Digraph {
          sequence: ['A', ';'],
          character: '\u{0104}',
          description: "LATIN CAPITAL LETTER A WITH OGONEK",
      },
      Digraph {
          sequence: ['a', ';'],
          character: '\u{0105}',
          description: "LATIN SMALL LETTER A WITH OGONEK",
      },
      Digraph {
          sequence: ['C', '\''],
          character: '\u{0106}',
          description: "LATIN CAPITAL LETTER C WITH ACUTE",
      },
      Digraph {
          sequence: ['c', '\''],
          character: '\u{0107}',
          description: "LATIN SMALL LETTER C WITH ACUTE",
      },
      Digraph {
          sequence: ['C', '>'],
          character: '\u{0108}',
          description: "LATIN CAPITAL LETTER C WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['c', '>'],
          character: '\u{0109}',
          description: "LATIN SMALL LETTER C WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['C', '.'],
          character: '\u{010a}',
          description: "LATIN CAPITAL LETTER C WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['c', '.'],
          character: '\u{010b}',
          description: "LATIN SMALL LETTER C WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['C', '<'],
          character: '\u{010c}',
          description: "LATIN CAPITAL LETTER C WITH CARON",
      },
      Digraph {
          sequence: ['c', '<'],
          character: '\u{010d}',
          description: "LATIN SMALL LETTER C WITH CARON",
      },
      Digraph {
          sequence: ['D', '<'],
          character: '\u{010e}',
          description: "LATIN CAPITAL LETTER D WITH CARON",
      },
      Digraph {
          sequence: ['d', '<'],
          character: '\u{010f}',
          description: "LATIN SMALL LETTER D WITH CARON",
      },
      Digraph {
          sequence: ['D', '/'],
          character: '\u{0110}',
          description: "LATIN CAPITAL LETTER D WITH STROKE",
      },
      Digraph {
          sequence: ['d', '/'],
          character: '\u{0111}',
          description: "LATIN SMALL LETTER D WITH STROKE",
      },
      Digraph {
          sequence: ['E', '-'],
          character: '\u{0112}',
          description: "LATIN CAPITAL LETTER E WITH MACRON",
      },
      Digraph {
          sequence: ['e', '-'],
          character: '\u{0113}',
          description: "LATIN SMALL LETTER E WITH MACRON",
      },
      Digraph {
          sequence: ['E', '('],
          character: '\u{0114}',
          description: "LATIN CAPITAL LETTER E WITH BREVE",
      },
      Digraph {
          sequence: ['e', '('],
          character: '\u{0115}',
          description: "LATIN SMALL LETTER E WITH BREVE",
      },
      Digraph {
          sequence: ['E', '.'],
          character: '\u{0116}',
          description: "LATIN CAPITAL LETTER E WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['e', '.'],
          character: '\u{0117}',
          description: "LATIN SMALL LETTER E WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['E', ';'],
          character: '\u{0118}',
          description: "LATIN CAPITAL LETTER E WITH OGONEK",
      },
      Digraph {
          sequence: ['e', ';'],
          character: '\u{0119}',
          description: "LATIN SMALL LETTER E WITH OGONEK",
      },
      Digraph {
          sequence: ['E', '<'],
          character: '\u{011a}',
          description: "LATIN CAPITAL LETTER E WITH CARON",
      },
      Digraph {
          sequence: ['e', '<'],
          character: '\u{011b}',
          description: "LATIN SMALL LETTER E WITH CARON",
      },
      Digraph {
          sequence: ['G', '>'],
          character: '\u{011c}',
          description: "LATIN CAPITAL LETTER G WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['g', '>'],
          character: '\u{011d}',
          description: "LATIN SMALL LETTER G WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['G', '('],
          character: '\u{011e}',
          description: "LATIN CAPITAL LETTER G WITH BREVE",
      },
      Digraph {
          sequence: ['g', '('],
          character: '\u{011f}',
          description: "LATIN SMALL LETTER G WITH BREVE",
      },
      Digraph {
          sequence: ['G', '.'],
          character: '\u{0120}',
          description: "LATIN CAPITAL LETTER G WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['g', '.'],
          character: '\u{0121}',
          description: "LATIN SMALL LETTER G WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['G', ','],
          character: '\u{0122}',
          description: "LATIN CAPITAL LETTER G WITH CEDILLA",
      },
      Digraph {
          sequence: ['g', ','],
          character: '\u{0123}',
          description: "LATIN SMALL LETTER G WITH CEDILLA",
      },
      Digraph {
          sequence: ['H', '>'],
          character: '\u{0124}',
          description: "LATIN CAPITAL LETTER H WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['h', '>'],
          character: '\u{0125}',
          description: "LATIN SMALL LETTER H WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['H', '/'],
          character: '\u{0126}',
          description: "LATIN CAPITAL LETTER H WITH STROKE",
      },
      Digraph {
          sequence: ['h', '/'],
          character: '\u{0127}',
          description: "LATIN SMALL LETTER H WITH STROKE",
      },
      Digraph {
          sequence: ['I', '?'],
          character: '\u{0128}',
          description: "LATIN CAPITAL LETTER I WITH TILDE",
      },
      Digraph {
          sequence: ['i', '?'],
          character: '\u{0129}',
          description: "LATIN SMALL LETTER I WITH TILDE",
      },
      Digraph {
          sequence: ['I', '-'],
          character: '\u{012a}',
          description: "LATIN CAPITAL LETTER I WITH MACRON",
      },
      Digraph {
          sequence: ['i', '-'],
          character: '\u{012b}',
          description: "LATIN SMALL LETTER I WITH MACRON",
      },
      Digraph {
          sequence: ['I', '('],
          character: '\u{012c}',
          description: "LATIN CAPITAL LETTER I WITH BREVE",
      },
      Digraph {
          sequence: ['i', '('],
          character: '\u{012d}',
          description: "LATIN SMALL LETTER I WITH BREVE",
      },
      Digraph {
          sequence: ['I', ';'],
          character: '\u{012e}',
          description: "LATIN CAPITAL LETTER I WITH OGONEK",
      },
      Digraph {
          sequence: ['i', ';'],
          character: '\u{012f}',
          description: "LATIN SMALL LETTER I WITH OGONEK",
      },
      Digraph {
          sequence: ['I', '.'],
          character: '\u{0130}',
          description: "LATIN CAPITAL LETTER I WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['i', '.'],
          character: '\u{0131}',
          description: "LATIN SMALL LETTER I DOTLESS",
      },
      Digraph {
          sequence: ['I', 'J'],
          character: '\u{0132}',
          description: "LATIN CAPITAL LIGATURE IJ",
      },
      Digraph {
          sequence: ['i', 'j'],
          character: '\u{0133}',
          description: "LATIN SMALL LIGATURE IJ",
      },
      Digraph {
          sequence: ['J', '>'],
          character: '\u{0134}',
          description: "LATIN CAPITAL LETTER J WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['j', '>'],
          character: '\u{0135}',
          description: "LATIN SMALL LETTER J WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['K', ','],
          character: '\u{0136}',
          description: "LATIN CAPITAL LETTER K WITH CEDILLA",
      },
      Digraph {
          sequence: ['k', ','],
          character: '\u{0137}',
          description: "LATIN SMALL LETTER K WITH CEDILLA",
      },
      Digraph {
          sequence: ['k', 'k'],
          character: '\u{0138}',
          description: "LATIN SMALL LETTER KRA (Greenlandic)",
      },
      Digraph {
          sequence: ['L', '\''],
          character: '\u{0139}',
          description: "LATIN CAPITAL LETTER L WITH ACUTE",
      },
      Digraph {
          sequence: ['l', '\''],
          character: '\u{013a}',
          description: "LATIN SMALL LETTER L WITH ACUTE",
      },
      Digraph {
          sequence: ['L', ','],
          character: '\u{013b}',
          description: "LATIN CAPITAL LETTER L WITH CEDILLA",
      },
      Digraph {
          sequence: ['l', ','],
          character: '\u{013c}',
          description: "LATIN SMALL LETTER L WITH CEDILLA",
      },
      Digraph {
          sequence: ['L', '<'],
          character: '\u{013d}',
          description: "LATIN CAPITAL LETTER L WITH CARON",
      },
      Digraph {
          sequence: ['l', '<'],
          character: '\u{013e}',
          description: "LATIN SMALL LETTER L WITH CARON",
      },
      Digraph {
          sequence: ['L', '.'],
          character: '\u{013f}',
          description: "LATIN CAPITAL LETTER L WITH MIDDLE DOT",
      },
      Digraph {
          sequence: ['l', '.'],
          character: '\u{0140}',
          description: "LATIN SMALL LETTER L WITH MIDDLE DOT",
      },
      Digraph {
          sequence: ['L', '/'],
          character: '\u{0141}',
          description: "LATIN CAPITAL LETTER L WITH STROKE",
      },
      Digraph {
          sequence: ['l', '/'],
          character: '\u{0142}',
          description: "LATIN SMALL LETTER L WITH STROKE",
      },
      Digraph {
          sequence: ['N', '\''],
          character: '\u{0143}',
          description: "LATIN CAPITAL LETTER N WITH ACUTE",
      },
      Digraph {
          sequence: ['n', '\''],
          character: '\u{0144}',
          description: "LATIN SMALL LETTER N WITH ACUTE",
      },
      Digraph {
          sequence: ['N', ','],
          character: '\u{0145}',
          description: "LATIN CAPITAL LETTER N WITH CEDILLA",
      },
      Digraph {
          sequence: ['n', ','],
          character: '\u{0146}',
          description: "LATIN SMALL LETTER N WITH CEDILLA",
      },
      Digraph {
          sequence: ['N', '<'],
          character: '\u{0147}',
          description: "LATIN CAPITAL LETTER N WITH CARON",
      },
      Digraph {
          sequence: ['n', '<'],
          character: '\u{0148}',
          description: "LATIN SMALL LETTER N WITH CARON",
      },
      Digraph {
          sequence: ['\'', 'n'],
          character: '\u{0149}',
          description: "LATIN SMALL LETTER N PRECEDED BY APOSTROPHE",
      },
      Digraph {
          sequence: ['N', 'G'],
          character: '\u{014a}',
          description: "LATIN CAPITAL LETTER ENG (Lappish)",
      },
      Digraph {
          sequence: ['n', 'g'],
          character: '\u{014b}',
          description: "LATIN SMALL LETTER ENG (Lappish)",
      },
      Digraph {
          sequence: ['O', '-'],
          character: '\u{014c}',
          description: "LATIN CAPITAL LETTER O WITH MACRON",
      },
      Digraph {
          sequence: ['o', '-'],
          character: '\u{014d}',
          description: "LATIN SMALL LETTER O WITH MACRON",
      },
      Digraph {
          sequence: ['O', '('],
          character: '\u{014e}',
          description: "LATIN CAPITAL LETTER O WITH BREVE",
      },
      Digraph {
          sequence: ['o', '('],
          character: '\u{014f}',
          description: "LATIN SMALL LETTER O WITH BREVE",
      },
      Digraph {
          sequence: ['O', '"'],
          character: '\u{0150}',
          description: "LATIN CAPITAL LETTER O WITH DOUBLE ACUTE",
      },
      Digraph {
          sequence: ['o', '"'],
          character: '\u{0151}',
          description: "LATIN SMALL LETTER O WITH DOUBLE ACUTE",
      },
      Digraph {
          sequence: ['O', 'E'],
          character: '\u{0152}',
          description: "LATIN CAPITAL LIGATURE OE",
      },
      Digraph {
          sequence: ['o', 'e'],
          character: '\u{0153}',
          description: "LATIN SMALL LIGATURE OE",
      },
      Digraph {
          sequence: ['R', '\''],
          character: '\u{0154}',
          description: "LATIN CAPITAL LETTER R WITH ACUTE",
      },
      Digraph {
          sequence: ['r', '\''],
          character: '\u{0155}',
          description: "LATIN SMALL LETTER R WITH ACUTE",
      },
      Digraph {
          sequence: ['R', ','],
          character: '\u{0156}',
          description: "LATIN CAPITAL LETTER R WITH CEDILLA",
      },
      Digraph {
          sequence: ['r', ','],
          character: '\u{0157}',
          description: "LATIN SMALL LETTER R WITH CEDILLA",
      },
      Digraph {
          sequence: ['R', '<'],
          character: '\u{0158}',
          description: "LATIN CAPITAL LETTER R WITH CARON",
      },
      Digraph {
          sequence: ['r', '<'],
          character: '\u{0159}',
          description: "LATIN SMALL LETTER R WITH CARON",
      },
      Digraph {
          sequence: ['S', '\''],
          character: '\u{015a}',
          description: "LATIN CAPITAL LETTER S WITH ACUTE",
      },
      Digraph {
          sequence: ['s', '\''],
          character: '\u{015b}',
          description: "LATIN SMALL LETTER S WITH ACUTE",
      },
      Digraph {
          sequence: ['S', '>'],
          character: '\u{015c}',
          description: "LATIN CAPITAL LETTER S WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['s', '>'],
          character: '\u{015d}',
          description: "LATIN SMALL LETTER S WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['S', ','],
          character: '\u{015e}',
          description: "LATIN CAPITAL LETTER S WITH CEDILLA",
      },
      Digraph {
          sequence: ['s', ','],
          character: '\u{015f}',
          description: "LATIN SMALL LETTER S WITH CEDILLA",
      },
      Digraph {
          sequence: ['S', '<'],
          character: '\u{0160}',
          description: "LATIN CAPITAL LETTER S WITH CARON",
      },
      Digraph {
          sequence: ['s', '<'],
          character: '\u{0161}',
          description: "LATIN SMALL LETTER S WITH CARON",
      },
      Digraph {
          sequence: ['T', ','],
          character: '\u{0162}',
          description: "LATIN CAPITAL LETTER T WITH CEDILLA",
      },
      Digraph {
          sequence: ['t', ','],
          character: '\u{0163}',
          description: "LATIN SMALL LETTER T WITH CEDILLA",
      },
      Digraph {
          sequence: ['T', '<'],
          character: '\u{0164}',
          description: "LATIN CAPITAL LETTER T WITH CARON",
      },
      Digraph {
          sequence: ['t', '<'],
          character: '\u{0165}',
          description: "LATIN SMALL LETTER T WITH CARON",
      },
      Digraph {
          sequence: ['T', '/'],
          character: '\u{0166}',
          description: "LATIN CAPITAL LETTER T WITH STROKE",
      },
      Digraph {
          sequence: ['t', '/'],
          character: '\u{0167}',
          description: "LATIN SMALL LETTER T WITH STROKE",
      },
      Digraph {
          sequence: ['U', '?'],
          character: '\u{0168}',
          description: "LATIN CAPITAL LETTER U WITH TILDE",
      },
      Digraph {
          sequence: ['u', '?'],
          character: '\u{0169}',
          description: "LATIN SMALL LETTER U WITH TILDE",
      },
      Digraph {
          sequence: ['U', '-'],
          character: '\u{016a}',
          description: "LATIN CAPITAL LETTER U WITH MACRON",
      },
      Digraph {
          sequence: ['u', '-'],
          character: '\u{016b}',
          description: "LATIN SMALL LETTER U WITH MACRON",
      },
      Digraph {
          sequence: ['U', '('],
          character: '\u{016c}',
          description: "LATIN CAPITAL LETTER U WITH BREVE",
      },
      Digraph {
          sequence: ['u', '('],
          character: '\u{016d}',
          description: "LATIN SMALL LETTER U WITH BREVE",
      },
      Digraph {
          sequence: ['U', '0'],
          character: '\u{016e}',
          description: "LATIN CAPITAL LETTER U WITH RING ABOVE",
      },
      Digraph {
          sequence: ['u', '0'],
          character: '\u{016f}',
          description: "LATIN SMALL LETTER U WITH RING ABOVE",
      },
      Digraph {
          sequence: ['U', '"'],
          character: '\u{0170}',
          description: "LATIN CAPITAL LETTER U WITH DOUBLE ACUTE",
      },
      Digraph {
          sequence: ['u', '"'],
          character: '\u{0171}',
          description: "LATIN SMALL LETTER U WITH DOUBLE ACUTE",
      },
      Digraph {
          sequence: ['U', ';'],
          character: '\u{0172}',
          description: "LATIN CAPITAL LETTER U WITH OGONEK",
      },
      Digraph {
          sequence: ['u', ';'],
          character: '\u{0173}',
          description: "LATIN SMALL LETTER U WITH OGONEK",
      },
      Digraph {
          sequence: ['W', '>'],
          character: '\u{0174}',
          description: "LATIN CAPITAL LETTER W WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['w', '>'],
          character: '\u{0175}',
          description: "LATIN SMALL LETTER W WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['Y', '>'],
          character: '\u{0176}',
          description: "LATIN CAPITAL LETTER Y WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['y', '>'],
          character: '\u{0177}',
          description: "LATIN SMALL LETTER Y WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['Y', ':'],
          character: '\u{0178}',
          description: "LATIN CAPITAL LETTER Y WITH DIAERESIS",
      },
      Digraph {
          sequence: ['Z', '\''],
          character: '\u{0179}',
          description: "LATIN CAPITAL LETTER Z WITH ACUTE",
      },
      Digraph {
          sequence: ['z', '\''],
          character: '\u{017a}',
          description: "LATIN SMALL LETTER Z WITH ACUTE",
      },
      Digraph {
          sequence: ['Z', '.'],
          character: '\u{017b}',
          description: "LATIN CAPITAL LETTER Z WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['z', '.'],
          character: '\u{017c}',
          description: "LATIN SMALL LETTER Z WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['Z', '<'],
          character: '\u{017d}',
          description: "LATIN CAPITAL LETTER Z WITH CARON",
      },
      Digraph {
          sequence: ['z', '<'],
          character: '\u{017e}',
          description: "LATIN SMALL LETTER Z WITH CARON",
      },
      Digraph {
          sequence: ['O', '9'],
          character: '\u{01a0}',
          description: "LATIN CAPITAL LETTER O WITH HORN",
      },
      Digraph {
          sequence: ['o', '9'],
          character: '\u{01a1}',
          description: "LATIN SMALL LETTER O WITH HORN",
      },
      Digraph {
          sequence: ['O', 'I'],
          character: '\u{01a2}',
          description: "LATIN CAPITAL LETTER OI",
      },
      Digraph {
          sequence: ['o', 'i'],
          character: '\u{01a3}',
          description: "LATIN SMALL LETTER OI",
      },
      Digraph {
          sequence: ['y', 'r'],
          character: '\u{01a6}',
          description: "LATIN LETTER YR",
      },
      Digraph {
          sequence: ['U', '9'],
          character: '\u{01af}',
          description: "LATIN CAPITAL LETTER U WITH HORN",
      },
      Digraph {
          sequence: ['u', '9'],
          character: '\u{01b0}',
          description: "LATIN SMALL LETTER U WITH HORN",
      },
      Digraph {
          sequence: ['Z', '/'],
          character: '\u{01b5}',
          description: "LATIN CAPITAL LETTER Z WITH STROKE",
      },
      Digraph {
          sequence: ['z', '/'],
          character: '\u{01b6}',
          description: "LATIN SMALL LETTER Z WITH STROKE",
      },
      Digraph {
          sequence: ['E', 'D'],
          character: '\u{01b7}',
          description: "LATIN CAPITAL LETTER EZH",
      },
      Digraph {
          sequence: ['A', '<'],
          character: '\u{01cd}',
          description: "LATIN CAPITAL LETTER A WITH CARON",
      },
      Digraph {
          sequence: ['a', '<'],
          character: '\u{01ce}',
          description: "LATIN SMALL LETTER A WITH CARON",
      },
      Digraph {
          sequence: ['I', '<'],
          character: '\u{01cf}',
          description: "LATIN CAPITAL LETTER I WITH CARON",
      },
      Digraph {
          sequence: ['i', '<'],
          character: '\u{01d0}',
          description: "LATIN SMALL LETTER I WITH CARON",
      },
      Digraph {
          sequence: ['O', '<'],
          character: '\u{01d1}',
          description: "LATIN CAPITAL LETTER O WITH CARON",
      },
      Digraph {
          sequence: ['o', '<'],
          character: '\u{01d2}',
          description: "LATIN SMALL LETTER O WITH CARON",
      },
      Digraph {
          sequence: ['U', '<'],
          character: '\u{01d3}',
          description: "LATIN CAPITAL LETTER U WITH CARON",
      },
      Digraph {
          sequence: ['u', '<'],
          character: '\u{01d4}',
          description: "LATIN SMALL LETTER U WITH CARON",
      },
      Digraph {
          sequence: ['A', '1'],
          character: '\u{01de}',
          description: "LATIN CAPITAL LETTER A WITH DIAERESIS AND MACRON",
      },
      Digraph {
          sequence: ['a', '1'],
          character: '\u{01df}',
          description: "LATIN SMALL LETTER A WITH DIAERESIS AND MACRON",
      },
      Digraph {
          sequence: ['A', '7'],
          character: '\u{01e0}',
          description: "LATIN CAPITAL LETTER A WITH DOT ABOVE AND MACRON",
      },
      Digraph {
          sequence: ['a', '7'],
          character: '\u{01e1}',
          description: "LATIN SMALL LETTER A WITH DOT ABOVE AND MACRON",
      },
      Digraph {
          sequence: ['A', '3'],
          character: '\u{01e2}',
          description: "LATIN CAPITAL LETTER AE WITH MACRON",
      },
      Digraph {
          sequence: ['a', '3'],
          character: '\u{01e3}',
          description: "LATIN SMALL LETTER AE WITH MACRON",
      },
      Digraph {
          sequence: ['G', '/'],
          character: '\u{01e4}',
          description: "LATIN CAPITAL LETTER G WITH STROKE",
      },
      Digraph {
          sequence: ['g', '/'],
          character: '\u{01e5}',
          description: "LATIN SMALL LETTER G WITH STROKE",
      },
      Digraph {
          sequence: ['G', '<'],
          character: '\u{01e6}',
          description: "LATIN CAPITAL LETTER G WITH CARON",
      },
      Digraph {
          sequence: ['g', '<'],
          character: '\u{01e7}',
          description: "LATIN SMALL LETTER G WITH CARON",
      },
      Digraph {
          sequence: ['K', '<'],
          character: '\u{01e8}',
          description: "LATIN CAPITAL LETTER K WITH CARON",
      },
      Digraph {
          sequence: ['k', '<'],
          character: '\u{01e9}',
          description: "LATIN SMALL LETTER K WITH CARON",
      },
      Digraph {
          sequence: ['O', ';'],
          character: '\u{01ea}',
          description: "LATIN CAPITAL LETTER O WITH OGONEK",
      },
      Digraph {
          sequence: ['o', ';'],
          character: '\u{01eb}',
          description: "LATIN SMALL LETTER O WITH OGONEK",
      },
      Digraph {
          sequence: ['O', '1'],
          character: '\u{01ec}',
          description: "LATIN CAPITAL LETTER O WITH OGONEK AND MACRON",
      },
      Digraph {
          sequence: ['o', '1'],
          character: '\u{01ed}',
          description: "LATIN SMALL LETTER O WITH OGONEK AND MACRON",
      },
      Digraph {
          sequence: ['E', 'Z'],
          character: '\u{01ee}',
          description: "LATIN CAPITAL LETTER EZH WITH CARON",
      },
      Digraph {
          sequence: ['e', 'z'],
          character: '\u{01ef}',
          description: "LATIN SMALL LETTER EZH WITH CARON",
      },
      Digraph {
          sequence: ['j', '<'],
          character: '\u{01f0}',
          description: "LATIN SMALL LETTER J WITH CARON",
      },
      Digraph {
          sequence: ['G', '\''],
          character: '\u{01f4}',
          description: "LATIN CAPITAL LETTER G WITH ACUTE",
      },
      Digraph {
          sequence: ['g', '\''],
          character: '\u{01f5}',
          description: "LATIN SMALL LETTER G WITH ACUTE",
      },
      Digraph {
          sequence: [';', 'S'],
          character: '\u{02bf}',
          description: "MODIFIER LETTER LEFT HALF RING",
      },
      Digraph {
          sequence: ['\'', '<'],
          character: '\u{02c7}',
          description: "CARON",
      },
      Digraph {
          sequence: ['\'', '('],
          character: '\u{02d8}',
          description: "BREVE",
      },
      Digraph {
          sequence: ['\'', '.'],
          character: '\u{02d9}',
          description: "DOT ABOVE",
      },
      Digraph {
          sequence: ['\'', '0'],
          character: '\u{02da}',
          description: "RING ABOVE",
      },
      Digraph {
          sequence: ['\'', ';'],
          character: '\u{02db}',
          description: "OGONEK",
      },
      Digraph {
          sequence: ['\'', '"'],
          character: '\u{02dd}',
          description: "DOUBLE ACUTE ACCENT",
      },
      Digraph {
          sequence: ['A', '%'],
          character: '\u{0386}',
          description: "GREEK CAPITAL LETTER ALPHA WITH ACUTE",
      },
      Digraph {
          sequence: ['E', '%'],
          character: '\u{0388}',
          description: "GREEK CAPITAL LETTER EPSILON WITH ACUTE",
      },
      Digraph {
          sequence: ['Y', '%'],
          character: '\u{0389}',
          description: "GREEK CAPITAL LETTER ETA WITH ACUTE",
      },
      Digraph {
          sequence: ['I', '%'],
          character: '\u{038a}',
          description: "GREEK CAPITAL LETTER IOTA WITH ACUTE",
      },
      Digraph {
          sequence: ['O', '%'],
          character: '\u{038c}',
          description: "GREEK CAPITAL LETTER OMICRON WITH ACUTE",
      },
      Digraph {
          sequence: ['U', '%'],
          character: '\u{038e}',
          description: "GREEK CAPITAL LETTER UPSILON WITH ACUTE",
      },
      Digraph {
          sequence: ['W', '%'],
          character: '\u{038f}',
          description: "GREEK CAPITAL LETTER OMEGA WITH ACUTE",
      },
      Digraph {
          sequence: ['i', '3'],
          character: '\u{0390}',
          description: "GREEK SMALL LETTER IOTA WITH ACUTE AND DIAERESIS",
      },
      Digraph {
          sequence: ['A', '*'],
          character: '\u{0391}',
          description: "GREEK CAPITAL LETTER ALPHA",
      },
      Digraph {
          sequence: ['B', '*'],
          character: '\u{0392}',
          description: "GREEK CAPITAL LETTER BETA",
      },
      Digraph {
          sequence: ['G', '*'],
          character: '\u{0393}',
          description: "GREEK CAPITAL LETTER GAMMA",
      },
      Digraph {
          sequence: ['D', '*'],
          character: '\u{0394}',
          description: "GREEK CAPITAL LETTER DELTA",
      },
      Digraph {
          sequence: ['E', '*'],
          character: '\u{0395}',
          description: "GREEK CAPITAL LETTER EPSILON",
      },
      Digraph {
          sequence: ['Z', '*'],
          character: '\u{0396}',
          description: "GREEK CAPITAL LETTER ZETA",
      },
      Digraph {
          sequence: ['Y', '*'],
          character: '\u{0397}',
          description: "GREEK CAPITAL LETTER ETA",
      },
      Digraph {
          sequence: ['H', '*'],
          character: '\u{0398}',
          description: "GREEK CAPITAL LETTER THETA",
      },
      Digraph {
          sequence: ['I', '*'],
          character: '\u{0399}',
          description: "GREEK CAPITAL LETTER IOTA",
      },
      Digraph {
          sequence: ['K', '*'],
          character: '\u{039a}',
          description: "GREEK CAPITAL LETTER KAPPA",
      },
      Digraph {
          sequence: ['L', '*'],
          character: '\u{039b}',
          description: "GREEK CAPITAL LETTER LAMDA",
      },
      Digraph {
          sequence: ['N', '*'],
          character: '\u{039d}',
          description: "GREEK CAPITAL LETTER NU",
      },
      Digraph {
          sequence: ['C', '*'],
          character: '\u{039e}',
          description: "GREEK CAPITAL LETTER XI",
      },
      Digraph {
          sequence: ['O', '*'],
          character: '\u{039f}',
          description: "GREEK CAPITAL LETTER OMICRON",
      },
      Digraph {
          sequence: ['P', '*'],
          character: '\u{03a0}',
          description: "GREEK CAPITAL LETTER PI",
      },
      Digraph {
          sequence: ['R', '*'],
          character: '\u{03a1}',
          description: "GREEK CAPITAL LETTER RHO",
      },
      Digraph {
          sequence: ['S', '*'],
          character: '\u{03a3}',
          description: "GREEK CAPITAL LETTER SIGMA",
      },
      Digraph {
          sequence: ['T', '*'],
          character: '\u{03a4}',
          description: "GREEK CAPITAL LETTER TAU",
      },
      Digraph {
          sequence: ['U', '*'],
          character: '\u{03a5}',
          description: "GREEK CAPITAL LETTER UPSILON",
      },
      Digraph {
          sequence: ['F', '*'],
          character: '\u{03a6}',
          description: "GREEK CAPITAL LETTER PHI",
      },
      Digraph {
          sequence: ['X', '*'],
          character: '\u{03a7}',
          description: "GREEK CAPITAL LETTER CHI",
      },
      Digraph {
          sequence: ['Q', '*'],
          character: '\u{03a8}',
          description: "GREEK CAPITAL LETTER PSI",
      },
      Digraph {
          sequence: ['W', '*'],
          character: '\u{03a9}',
          description: "GREEK CAPITAL LETTER OMEGA",
      },
      Digraph {
          sequence: ['J', '*'],
          character: '\u{03aa}',
          description: "GREEK CAPITAL LETTER IOTA WITH DIAERESIS",
      },
      Digraph {
          sequence: ['V', '*'],
          character: '\u{03ab}',
          description: "GREEK CAPITAL LETTER UPSILON WITH DIAERESIS",
      },
      Digraph {
          sequence: ['a', '%'],
          character: '\u{03ac}',
          description: "GREEK SMALL LETTER ALPHA WITH ACUTE",
      },
      Digraph {
          sequence: ['e', '%'],
          character: '\u{03ad}',
          description: "GREEK SMALL LETTER EPSILON WITH ACUTE",
      },
      Digraph {
          sequence: ['y', '%'],
          character: '\u{03ae}',
          description: "GREEK SMALL LETTER ETA WITH ACUTE",
      },
      Digraph {
          sequence: ['i', '%'],
          character: '\u{03af}',
          description: "GREEK SMALL LETTER IOTA WITH ACUTE",
      },
      Digraph {
          sequence: ['u', '3'],
          character: '\u{03b0}',
          description: "GREEK SMALL LETTER UPSILON WITH ACUTE AND DIAERESIS",
      },
      Digraph {
          sequence: ['a', '*'],
          character: '\u{03b1}',
          description: "GREEK SMALL LETTER ALPHA",
      },
      Digraph {
          sequence: ['b', '*'],
          character: '\u{03b2}',
          description: "GREEK SMALL LETTER BETA",
      },
      Digraph {
          sequence: ['g', '*'],
          character: '\u{03b3}',
          description: "GREEK SMALL LETTER GAMMA",
      },
      Digraph {
          sequence: ['d', '*'],
          character: '\u{03b4}',
          description: "GREEK SMALL LETTER DELTA",
      },
      Digraph {
          sequence: ['e', '*'],
          character: '\u{03b5}',
          description: "GREEK SMALL LETTER EPSILON",
      },
      Digraph {
          sequence: ['z', '*'],
          character: '\u{03b6}',
          description: "GREEK SMALL LETTER ZETA",
      },
      Digraph {
          sequence: ['y', '*'],
          character: '\u{03b7}',
          description: "GREEK SMALL LETTER ETA",
      },
      Digraph {
          sequence: ['h', '*'],
          character: '\u{03b8}',
          description: "GREEK SMALL LETTER THETA",
      },
      Digraph {
          sequence: ['i', '*'],
          character: '\u{03b9}',
          description: "GREEK SMALL LETTER IOTA",
      },
      Digraph {
          sequence: ['k', '*'],
          character: '\u{03ba}',
          description: "GREEK SMALL LETTER KAPPA",
      },
      Digraph {
          sequence: ['l', '*'],
          character: '\u{03bb}',
          description: "GREEK SMALL LETTER LAMDA",
      },
      Digraph {
          sequence: ['m', '*'],
          character: '\u{03bc}',
          description: "GREEK SMALL LETTER MU",
      },
      Digraph {
          sequence: ['n', '*'],
          character: '\u{03bd}',
          description: "GREEK SMALL LETTER NU",
      },
      Digraph {
          sequence: ['o', '*'],
          character: '\u{03bf}',
          description: "GREEK SMALL LETTER OMICRON",
      },
      Digraph {
          sequence: ['p', '*'],
          character: '\u{03c0}',
          description: "GREEK SMALL LETTER PI",
      },
      Digraph {
          sequence: ['*', 's'],
          character: '\u{03c2}',
          description: "GREEK SMALL LETTER FINAL SIGMA",
      },
      Digraph {
          sequence: ['s', '*'],
          character: '\u{03c3}',
          description: "GREEK SMALL LETTER SIGMA",
      },
      Digraph {
          sequence: ['t', '*'],
          character: '\u{03c4}',
          description: "GREEK SMALL LETTER TAU",
      },
      Digraph {
          sequence: ['u', '*'],
          character: '\u{03c5}',
          description: "GREEK SMALL LETTER UPSILON",
      },
      Digraph {
          sequence: ['f', '*'],
          character: '\u{03c6}',
          description: "GREEK SMALL LETTER PHI",
      },
      Digraph {
          sequence: ['x', '*'],
          character: '\u{03c7}',
          description: "GREEK SMALL LETTER CHI",
      },
      Digraph {
          sequence: ['q', '*'],
          character: '\u{03c8}',
          description: "GREEK SMALL LETTER PSI",
      },
      Digraph {
          sequence: ['w', '*'],
          character: '\u{03c9}',
          description: "GREEK SMALL LETTER OMEGA",
      },
      Digraph {
          sequence: ['j', '*'],
          character: '\u{03ca}',
          description: "GREEK SMALL LETTER IOTA WITH DIAERESIS",
      },
      Digraph {
          sequence: ['v', '*'],
          character: '\u{03cb}',
          description: "GREEK SMALL LETTER UPSILON WITH DIAERESIS",
      },
      Digraph {
          sequence: ['o', '%'],
          character: '\u{03cc}',
          description: "GREEK SMALL LETTER OMICRON WITH ACUTE",
      },
      Digraph {
          sequence: ['u', '%'],
          character: '\u{03cd}',
          description: "GREEK SMALL LETTER UPSILON WITH ACUTE",
      },
      Digraph {
          sequence: ['w', '%'],
          character: '\u{03ce}',
          description: "GREEK SMALL LETTER OMEGA WITH ACUTE",
      },
      Digraph {
          sequence: ['\'', 'G'],
          character: '\u{03d8}',
          description: "GREEK NUMERAL SIGN",
      },
      Digraph {
          sequence: [',', 'G'],
          character: '\u{03d9}',
          description: "GREEK LOWER NUMERAL SIGN",
      },
      Digraph {
          sequence: ['T', '3'],
          character: '\u{03da}',
          description: "GREEK CAPITAL LETTER STIGMA",
      },
      Digraph {
          sequence: ['t', '3'],
          character: '\u{03db}',
          description: "GREEK SMALL LETTER STIGMA",
      },
      Digraph {
          sequence: ['M', '3'],
          character: '\u{03dc}',
          description: "GREEK CAPITAL LETTER DIGAMMA",
      },
      Digraph {
          sequence: ['m', '3'],
          character: '\u{03dd}',
          description: "GREEK SMALL LETTER DIGAMMA",
      },
      Digraph {
          sequence: ['K', '3'],
          character: '\u{03de}',
          description: "GREEK CAPITAL LETTER KOPPA",
      },
      Digraph {
          sequence: ['k', '3'],
          character: '\u{03df}',
          description: "GREEK SMALL LETTER KOPPA",
      },
      Digraph {
          sequence: ['P', '3'],
          character: '\u{03e0}',
          description: "GREEK CAPITAL LETTER SAMPI",
      },
      Digraph {
          sequence: ['p', '3'],
          character: '\u{03e1}',
          description: "GREEK SMALL LETTER SAMPI",
      },
      Digraph {
          sequence: ['\'', '%'],
          character: '\u{03f4}',
          description: "ACUTE ACCENT AND DIAERESIS (Tonos and Dialytika)",
      },
      Digraph {
          sequence: ['j', '3'],
          character: '\u{03f5}',
          description: "GREEK IOTA BELOW",
      },
      Digraph {
          sequence: ['I', 'O'],
          character: '\u{0401}',
          description: "CYRILLIC CAPITAL LETTER IO",
      },
      Digraph {
          sequence: ['D', '%'],
          character: '\u{0402}',
          description: "CYRILLIC CAPITAL LETTER DJE (Serbocroatian)",
      },
      Digraph {
          sequence: ['G', '%'],
          character: '\u{0403}',
          description: "CYRILLIC CAPITAL LETTER GJE (Macedonian)",
      },
      Digraph {
          sequence: ['I', 'E'],
          character: '\u{0404}',
          description: "CYRILLIC CAPITAL LETTER UKRAINIAN IE",
      },
      Digraph {
          sequence: ['D', 'S'],
          character: '\u{0405}',
          description: "CYRILLIC CAPITAL LETTER DZE (Macedonian)",
      },
      Digraph {
          sequence: ['I', 'I'],
          character: '\u{0406}',
          description: "CYRILLIC CAPITAL LETTER BYELORUSSIAN-UKRAINIAN I",
      },
      Digraph {
          sequence: ['Y', 'I'],
          character: '\u{0407}',
          description: "CYRILLIC CAPITAL LETTER YI (Ukrainian)",
      },
      Digraph {
          sequence: ['J', '%'],
          character: '\u{0408}',
          description: "CYRILLIC CAPITAL LETTER JE",
      },
      Digraph {
          sequence: ['L', 'J'],
          character: '\u{0409}',
          description: "CYRILLIC CAPITAL LETTER LJE",
      },
      Digraph {
          sequence: ['N', 'J'],
          character: '\u{040a}',
          description: "CYRILLIC CAPITAL LETTER NJE",
      },
      Digraph {
          sequence: ['T', 's'],
          character: '\u{040b}',
          description: "CYRILLIC CAPITAL LETTER TSHE (Serbocroatian)",
      },
      Digraph {
          sequence: ['K', 'J'],
          character: '\u{040c}',
          description: "CYRILLIC CAPITAL LETTER KJE (Macedonian)",
      },
      Digraph {
          sequence: ['V', '%'],
          character: '\u{040e}',
          description: "CYRILLIC CAPITAL LETTER SHORT U (Byelorussian)",
      },
      Digraph {
          sequence: ['D', 'Z'],
          character: '\u{040f}',
          description: "CYRILLIC CAPITAL LETTER DZHE",
      },
      Digraph {
          sequence: ['A', '='],
          character: '\u{0410}',
          description: "CYRILLIC CAPITAL LETTER A",
      },
      Digraph {
          sequence: ['B', '='],
          character: '\u{0411}',
          description: "CYRILLIC CAPITAL LETTER BE",
      },
      Digraph {
          sequence: ['V', '='],
          character: '\u{0412}',
          description: "CYRILLIC CAPITAL LETTER VE",
      },
      Digraph {
          sequence: ['G', '='],
          character: '\u{0413}',
          description: "CYRILLIC CAPITAL LETTER GHE",
      },
      Digraph {
          sequence: ['D', '='],
          character: '\u{0414}',
          description: "CYRILLIC CAPITAL LETTER DE",
      },
      Digraph {
          sequence: ['E', '='],
          character: '\u{0415}',
          description: "CYRILLIC CAPITAL LETTER IE",
      },
      Digraph {
          sequence: ['Z', '%'],
          character: '\u{0416}',
          description: "CYRILLIC CAPITAL LETTER ZHE",
      },
      Digraph {
          sequence: ['Z', '='],
          character: '\u{0417}',
          description: "CYRILLIC CAPITAL LETTER ZE",
      },
      Digraph {
          sequence: ['I', '='],
          character: '\u{0418}',
          description: "CYRILLIC CAPITAL LETTER I",
      },
      Digraph {
          sequence: ['J', '='],
          character: '\u{0419}',
          description: "CYRILLIC CAPITAL LETTER SHORT I",
      },
      Digraph {
          sequence: ['K', '='],
          character: '\u{041a}',
          description: "CYRILLIC CAPITAL LETTER KA",
      },
      Digraph {
          sequence: ['L', '='],
          character: '\u{041b}',
          description: "CYRILLIC CAPITAL LETTER EL",
      },
      Digraph {
          sequence: ['M', '='],
          character: '\u{041c}',
          description: "CYRILLIC CAPITAL LETTER EM",
      },
      Digraph {
          sequence: ['N', '='],
          character: '\u{041d}',
          description: "CYRILLIC CAPITAL LETTER EN",
      },
      Digraph {
          sequence: ['O', '='],
          character: '\u{041e}',
          description: "CYRILLIC CAPITAL LETTER O",
      },
      Digraph {
          sequence: ['P', '='],
          character: '\u{041f}',
          description: "CYRILLIC CAPITAL LETTER PE",
      },
      Digraph {
          sequence: ['R', '='],
          character: '\u{0420}',
          description: "CYRILLIC CAPITAL LETTER ER",
      },
      Digraph {
          sequence: ['S', '='],
          character: '\u{0421}',
          description: "CYRILLIC CAPITAL LETTER ES",
      },
      Digraph {
          sequence: ['T', '='],
          character: '\u{0422}',
          description: "CYRILLIC CAPITAL LETTER TE",
      },
      Digraph {
          sequence: ['U', '='],
          character: '\u{0423}',
          description: "CYRILLIC CAPITAL LETTER U",
      },
      Digraph {
          sequence: ['F', '='],
          character: '\u{0424}',
          description: "CYRILLIC CAPITAL LETTER EF",
      },
      Digraph {
          sequence: ['H', '='],
          character: '\u{0425}',
          description: "CYRILLIC CAPITAL LETTER HA",
      },
      Digraph {
          sequence: ['C', '='],
          character: '\u{0426}',
          description: "CYRILLIC CAPITAL LETTER TSE",
      },
      Digraph {
          sequence: ['C', '%'],
          character: '\u{0427}',
          description: "CYRILLIC CAPITAL LETTER CHE",
      },
      Digraph {
          sequence: ['S', '%'],
          character: '\u{0428}',
          description: "CYRILLIC CAPITAL LETTER SHA",
      },
      Digraph {
          sequence: ['S', 'c'],
          character: '\u{0429}',
          description: "CYRILLIC CAPITAL LETTER SHCHA",
      },
      Digraph {
          sequence: ['=', '"'],
          character: '\u{042a}',
          description: "CYRILLIC CAPITAL LETTER HARD SIGN",
      },
      Digraph {
          sequence: ['Y', '='],
          character: '\u{042b}',
          description: "CYRILLIC CAPITAL LETTER YERU",
      },
      Digraph {
          sequence: ['%', '"'],
          character: '\u{042c}',
          description: "CYRILLIC CAPITAL LETTER SOFT SIGN",
      },
      Digraph {
          sequence: ['J', 'E'],
          character: '\u{042d}',
          description: "CYRILLIC CAPITAL LETTER E",
      },
      Digraph {
          sequence: ['J', 'U'],
          character: '\u{042e}',
          description: "CYRILLIC CAPITAL LETTER YU",
      },
      Digraph {
          sequence: ['J', 'A'],
          character: '\u{042f}',
          description: "CYRILLIC CAPITAL LETTER YA",
      },
      Digraph {
          sequence: ['a', '='],
          character: '\u{0430}',
          description: "CYRILLIC SMALL LETTER A",
      },
      Digraph {
          sequence: ['b', '='],
          character: '\u{0431}',
          description: "CYRILLIC SMALL LETTER BE",
      },
      Digraph {
          sequence: ['v', '='],
          character: '\u{0432}',
          description: "CYRILLIC SMALL LETTER VE",
      },
      Digraph {
          sequence: ['g', '='],
          character: '\u{0433}',
          description: "CYRILLIC SMALL LETTER GHE",
      },
      Digraph {
          sequence: ['d', '='],
          character: '\u{0434}',
          description: "CYRILLIC SMALL LETTER DE",
      },
      Digraph {
          sequence: ['e', '='],
          character: '\u{0435}',
          description: "CYRILLIC SMALL LETTER IE",
      },
      Digraph {
          sequence: ['z', '%'],
          character: '\u{0436}',
          description: "CYRILLIC SMALL LETTER ZHE",
      },
      Digraph {
          sequence: ['z', '='],
          character: '\u{0437}',
          description: "CYRILLIC SMALL LETTER ZE",
      },
      Digraph {
          sequence: ['i', '='],
          character: '\u{0438}',
          description: "CYRILLIC SMALL LETTER I",
      },
      Digraph {
          sequence: ['j', '='],
          character: '\u{0439}',
          description: "CYRILLIC SMALL LETTER SHORT I",
      },
      Digraph {
          sequence: ['k', '='],
          character: '\u{043a}',
          description: "CYRILLIC SMALL LETTER KA",
      },
      Digraph {
          sequence: ['l', '='],
          character: '\u{043b}',
          description: "CYRILLIC SMALL LETTER EL",
      },
      Digraph {
          sequence: ['m', '='],
          character: '\u{043c}',
          description: "CYRILLIC SMALL LETTER EM",
      },
      Digraph {
          sequence: ['n', '='],
          character: '\u{043d}',
          description: "CYRILLIC SMALL LETTER EN",
      },
      Digraph {
          sequence: ['o', '='],
          character: '\u{043e}',
          description: "CYRILLIC SMALL LETTER O",
      },
      Digraph {
          sequence: ['p', '='],
          character: '\u{043f}',
          description: "CYRILLIC SMALL LETTER PE",
      },
      Digraph {
          sequence: ['r', '='],
          character: '\u{0440}',
          description: "CYRILLIC SMALL LETTER ER",
      },
      Digraph {
          sequence: ['s', '='],
          character: '\u{0441}',
          description: "CYRILLIC SMALL LETTER ES",
      },
      Digraph {
          sequence: ['t', '='],
          character: '\u{0442}',
          description: "CYRILLIC SMALL LETTER TE",
      },
      Digraph {
          sequence: ['u', '='],
          character: '\u{0443}',
          description: "CYRILLIC SMALL LETTER U",
      },
      Digraph {
          sequence: ['f', '='],
          character: '\u{0444}',
          description: "CYRILLIC SMALL LETTER EF",
      },
      Digraph {
          sequence: ['h', '='],
          character: '\u{0445}',
          description: "CYRILLIC SMALL LETTER HA",
      },
      Digraph {
          sequence: ['c', '='],
          character: '\u{0446}',
          description: "CYRILLIC SMALL LETTER TSE",
      },
      Digraph {
          sequence: ['c', '%'],
          character: '\u{0447}',
          description: "CYRILLIC SMALL LETTER CHE",
      },
      Digraph {
          sequence: ['s', '%'],
          character: '\u{0448}',
          description: "CYRILLIC SMALL LETTER SHA",
      },
      Digraph {
          sequence: ['s', 'c'],
          character: '\u{0449}',
          description: "CYRILLIC SMALL LETTER SHCHA",
      },
      Digraph {
          sequence: ['=', '\''],
          character: '\u{044a}',
          description: "CYRILLIC SMALL LETTER HARD SIGN",
      },
      Digraph {
          sequence: ['y', '='],
          character: '\u{044b}',
          description: "CYRILLIC SMALL LETTER YERU",
      },
      Digraph {
          sequence: ['%', '\''],
          character: '\u{044c}',
          description: "CYRILLIC SMALL LETTER SOFT SIGN",
      },
      Digraph {
          sequence: ['j', 'e'],
          character: '\u{044d}',
          description: "CYRILLIC SMALL LETTER E",
      },
      Digraph {
          sequence: ['j', 'u'],
          character: '\u{044e}',
          description: "CYRILLIC SMALL LETTER YU",
      },
      Digraph {
          sequence: ['j', 'a'],
          character: '\u{044f}',
          description: "CYRILLIC SMALL LETTER YA",
      },
      Digraph {
          sequence: ['i', 'o'],
          character: '\u{0451}',
          description: "CYRILLIC SMALL LETTER IO",
      },
      Digraph {
          sequence: ['d', '%'],
          character: '\u{0452}',
          description: "CYRILLIC SMALL LETTER DJE (Serbocroatian)",
      },
      Digraph {
          sequence: ['g', '%'],
          character: '\u{0453}',
          description: "CYRILLIC SMALL LETTER GJE (Macedonian)",
      },
      Digraph {
          sequence: ['i', 'e'],
          character: '\u{0454}',
          description: "CYRILLIC SMALL LETTER UKRAINIAN IE",
      },
      Digraph {
          sequence: ['d', 's'],
          character: '\u{0455}',
          description: "CYRILLIC SMALL LETTER DZE (Macedonian)",
      },
      Digraph {
          sequence: ['i', 'i'],
          character: '\u{0456}',
          description: "CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I",
      },
      Digraph {
          sequence: ['y', 'i'],
          character: '\u{0457}',
          description: "CYRILLIC SMALL LETTER YI (Ukrainian)",
      },
      Digraph {
          sequence: ['j', '%'],
          character: '\u{0458}',
          description: "CYRILLIC SMALL LETTER JE",
      },
      Digraph {
          sequence: ['l', 'j'],
          character: '\u{0459}',
          description: "CYRILLIC SMALL LETTER LJE",
      },
      Digraph {
          sequence: ['n', 'j'],
          character: '\u{045a}',
          description: "CYRILLIC SMALL LETTER NJE",
      },
      Digraph {
          sequence: ['t', 's'],
          character: '\u{045b}',
          description: "CYRILLIC SMALL LETTER TSHE (Serbocroatian)",
      },
      Digraph {
          sequence: ['k', 'j'],
          character: '\u{045c}',
          description: "CYRILLIC SMALL LETTER KJE (Macedonian)",
      },
      Digraph {
          sequence: ['v', '%'],
          character: '\u{045e}',
          description: "CYRILLIC SMALL LETTER SHORT U (Byelorussian)",
      },
      Digraph {
          sequence: ['d', 'z'],
          character: '\u{045f}',
          description: "CYRILLIC SMALL LETTER DZHE",
      },
      Digraph {
          sequence: ['Y', '3'],
          character: '\u{0462}',
          description: "CYRILLIC CAPITAL LETTER YAT",
      },
      Digraph {
          sequence: ['y', '3'],
          character: '\u{0463}',
          description: "CYRILLIC SMALL LETTER YAT",
      },
      Digraph {
          sequence: ['O', '3'],
          character: '\u{046a}',
          description: "CYRILLIC CAPITAL LETTER BIG YUS",
      },
      Digraph {
          sequence: ['o', '3'],
          character: '\u{046b}',
          description: "CYRILLIC SMALL LETTER BIG YUS",
      },
      Digraph {
          sequence: ['F', '3'],
          character: '\u{0472}',
          description: "CYRILLIC CAPITAL LETTER FITA",
      },
      Digraph {
          sequence: ['f', '3'],
          character: '\u{0473}',
          description: "CYRILLIC SMALL LETTER FITA",
      },
      Digraph {
          sequence: ['V', '3'],
          character: '\u{0474}',
          description: "CYRILLIC CAPITAL LETTER IZHITSA",
      },
      Digraph {
          sequence: ['v', '3'],
          character: '\u{0475}',
          description: "CYRILLIC SMALL LETTER IZHITSA",
      },
      Digraph {
          sequence: ['C', '3'],
          character: '\u{0480}',
          description: "CYRILLIC CAPITAL LETTER KOPPA",
      },
      Digraph {
          sequence: ['c', '3'],
          character: '\u{0481}',
          description: "CYRILLIC SMALL LETTER KOPPA",
      },
      Digraph {
          sequence: ['G', '3'],
          character: '\u{0490}',
          description: "CYRILLIC CAPITAL LETTER GHE WITH UPTURN",
      },
      Digraph {
          sequence: ['g', '3'],
          character: '\u{0491}',
          description: "CYRILLIC SMALL LETTER GHE WITH UPTURN",
      },
      Digraph {
          sequence: ['A', '+'],
          character: '\u{05d0}',
          description: "HEBREW LETTER ALEF",
      },
      Digraph {
          sequence: ['B', '+'],
          character: '\u{05d1}',
          description: "HEBREW LETTER BET",
      },
      Digraph {
          sequence: ['G', '+'],
          character: '\u{05d2}',
          description: "HEBREW LETTER GIMEL",
      },
      Digraph {
          sequence: ['D', '+'],
          character: '\u{05d3}',
          description: "HEBREW LETTER DALET",
      },
      Digraph {
          sequence: ['H', '+'],
          character: '\u{05d4}',
          description: "HEBREW LETTER HE",
      },
      Digraph {
          sequence: ['W', '+'],
          character: '\u{05d5}',
          description: "HEBREW LETTER VAV",
      },
      Digraph {
          sequence: ['Z', '+'],
          character: '\u{05d6}',
          description: "HEBREW LETTER ZAYIN",
      },
      Digraph {
          sequence: ['X', '+'],
          character: '\u{05d7}',
          description: "HEBREW LETTER HET",
      },
      Digraph {
          sequence: ['T', 'j'],
          character: '\u{05d8}',
          description: "HEBREW LETTER TET",
      },
      Digraph {
          sequence: ['J', '+'],
          character: '\u{05d9}',
          description: "HEBREW LETTER YOD",
      },
      Digraph {
          sequence: ['K', '%'],
          character: '\u{05da}',
          description: "HEBREW LETTER FINAL KAF",
      },
      Digraph {
          sequence: ['K', '+'],
          character: '\u{05db}',
          description: "HEBREW LETTER KAF",
      },
      Digraph {
          sequence: ['L', '+'],
          character: '\u{05dc}',
          description: "HEBREW LETTER LAMED",
      },
      Digraph {
          sequence: ['M', '%'],
          character: '\u{05dd}',
          description: "HEBREW LETTER FINAL MEM",
      },
      Digraph {
          sequence: ['M', '+'],
          character: '\u{05de}',
          description: "HEBREW LETTER MEM",
      },
      Digraph {
          sequence: ['N', '%'],
          character: '\u{05df}',
          description: "HEBREW LETTER FINAL NUN",
      },
      Digraph {
          sequence: ['N', '+'],
          character: '\u{05e0}',
          description: "HEBREW LETTER NUN",
      },
      Digraph {
          sequence: ['S', '+'],
          character: '\u{05e1}',
          description: "HEBREW LETTER SAMEKH",
      },
      Digraph {
          sequence: ['E', '+'],
          character: '\u{05e2}',
          description: "HEBREW LETTER AYIN",
      },
      Digraph {
          sequence: ['P', '%'],
          character: '\u{05e3}',
          description: "HEBREW LETTER FINAL PE",
      },
      Digraph {
          sequence: ['P', '+'],
          character: '\u{05e4}',
          description: "HEBREW LETTER PE",
      },
      Digraph {
          sequence: ['Z', 'j'],
          character: '\u{05e5}',
          description: "HEBREW LETTER FINAL TSADI",
      },
      Digraph {
          sequence: ['Z', 'J'],
          character: '\u{05e6}',
          description: "HEBREW LETTER TSADI",
      },
      Digraph {
          sequence: ['Q', '+'],
          character: '\u{05e7}',
          description: "HEBREW LETTER QOF",
      },
      Digraph {
          sequence: ['R', '+'],
          character: '\u{05e8}',
          description: "HEBREW LETTER RESH",
      },
      Digraph {
          sequence: ['S', 'h'],
          character: '\u{05e9}',
          description: "HEBREW LETTER SHIN",
      },
      Digraph {
          sequence: ['T', '+'],
          character: '\u{05ea}',
          description: "HEBREW LETTER TAV",
      },
      Digraph {
          sequence: [',', '+'],
          character: '\u{060c}',
          description: "ARABIC COMMA",
      },
      Digraph {
          sequence: [';', '+'],
          character: '\u{061b}',
          description: "ARABIC SEMICOLON",
      },
      Digraph {
          sequence: ['?', '+'],
          character: '\u{061f}',
          description: "ARABIC QUESTION MARK",
      },
      Digraph {
          sequence: ['H', '\''],
          character: '\u{0621}',
          description: "ARABIC LETTER HAMZA",
      },
      Digraph {
          sequence: ['a', 'M'],
          character: '\u{0622}',
          description: "ARABIC LETTER ALEF WITH MADDA ABOVE",
      },
      Digraph {
          sequence: ['a', 'H'],
          character: '\u{0623}',
          description: "ARABIC LETTER ALEF WITH HAMZA ABOVE",
      },
      Digraph {
          sequence: ['w', 'H'],
          character: '\u{0624}',
          description: "ARABIC LETTER WAW WITH HAMZA ABOVE",
      },
      Digraph {
          sequence: ['a', 'h'],
          character: '\u{0625}',
          description: "ARABIC LETTER ALEF WITH HAMZA BELOW",
      },
      Digraph {
          sequence: ['y', 'H'],
          character: '\u{0626}',
          description: "ARABIC LETTER YEH WITH HAMZA ABOVE",
      },
      Digraph {
          sequence: ['a', '+'],
          character: '\u{0627}',
          description: "ARABIC LETTER ALEF",
      },
      Digraph {
          sequence: ['b', '+'],
          character: '\u{0628}',
          description: "ARABIC LETTER BEH",
      },
      Digraph {
          sequence: ['t', 'm'],
          character: '\u{0629}',
          description: "ARABIC LETTER TEH MARBUTA",
      },
      Digraph {
          sequence: ['t', '+'],
          character: '\u{062a}',
          description: "ARABIC LETTER TEH",
      },
      Digraph {
          sequence: ['t', 'k'],
          character: '\u{062b}',
          description: "ARABIC LETTER THEH",
      },
      Digraph {
          sequence: ['g', '+'],
          character: '\u{062c}',
          description: "ARABIC LETTER JEEM",
      },
      Digraph {
          sequence: ['h', 'k'],
          character: '\u{062d}',
          description: "ARABIC LETTER HAH",
      },
      Digraph {
          sequence: ['x', '+'],
          character: '\u{062e}',
          description: "ARABIC LETTER KHAH",
      },
      Digraph {
          sequence: ['d', '+'],
          character: '\u{062f}',
          description: "ARABIC LETTER DAL",
      },
      Digraph {
          sequence: ['d', 'k'],
          character: '\u{0630}',
          description: "ARABIC LETTER THAL",
      },
      Digraph {
          sequence: ['r', '+'],
          character: '\u{0631}',
          description: "ARABIC LETTER REH",
      },
      Digraph {
          sequence: ['z', '+'],
          character: '\u{0632}',
          description: "ARABIC LETTER ZAIN",
      },
      Digraph {
          sequence: ['s', '+'],
          character: '\u{0633}',
          description: "ARABIC LETTER SEEN",
      },
      Digraph {
          sequence: ['s', 'n'],
          character: '\u{0634}',
          description: "ARABIC LETTER SHEEN",
      },
      Digraph {
          sequence: ['c', '+'],
          character: '\u{0635}',
          description: "ARABIC LETTER SAD",
      },
      Digraph {
          sequence: ['d', 'd'],
          character: '\u{0636}',
          description: "ARABIC LETTER DAD",
      },
      Digraph {
          sequence: ['t', 'j'],
          character: '\u{0637}',
          description: "ARABIC LETTER TAH",
      },
      Digraph {
          sequence: ['z', 'H'],
          character: '\u{0638}',
          description: "ARABIC LETTER ZAH",
      },
      Digraph {
          sequence: ['e', '+'],
          character: '\u{0639}',
          description: "ARABIC LETTER AIN",
      },
      Digraph {
          sequence: ['i', '+'],
          character: '\u{063a}',
          description: "ARABIC LETTER GHAIN",
      },
      Digraph {
          sequence: ['+', '+'],
          character: '\u{0640}',
          description: "ARABIC TATWEEL",
      },
      Digraph {
          sequence: ['f', '+'],
          character: '\u{0641}',
          description: "ARABIC LETTER FEH",
      },
      Digraph {
          sequence: ['q', '+'],
          character: '\u{0642}',
          description: "ARABIC LETTER QAF",
      },
      Digraph {
          sequence: ['k', '+'],
          character: '\u{0643}',
          description: "ARABIC LETTER KAF",
      },
      Digraph {
          sequence: ['l', '+'],
          character: '\u{0644}',
          description: "ARABIC LETTER LAM",
      },
      Digraph {
          sequence: ['m', '+'],
          character: '\u{0645}',
          description: "ARABIC LETTER MEEM",
      },
      Digraph {
          sequence: ['n', '+'],
          character: '\u{0646}',
          description: "ARABIC LETTER NOON",
      },
      Digraph {
          sequence: ['h', '+'],
          character: '\u{0647}',
          description: "ARABIC LETTER HEH",
      },
      Digraph {
          sequence: ['w', '+'],
          character: '\u{0648}',
          description: "ARABIC LETTER WAW",
      },
      Digraph {
          sequence: ['j', '+'],
          character: '\u{0649}',
          description: "ARABIC LETTER ALEF MAKSURA",
      },
      Digraph {
          sequence: ['y', '+'],
          character: '\u{064a}',
          description: "ARABIC LETTER YEH",
      },
      Digraph {
          sequence: [':', '+'],
          character: '\u{064b}',
          description: "ARABIC FATHATAN",
      },
      Digraph {
          sequence: ['"', '+'],
          character: '\u{064c}',
          description: "ARABIC DAMMATAN",
      },
      Digraph {
          sequence: ['=', '+'],
          character: '\u{064d}',
          description: "ARABIC KASRATAN",
      },
      Digraph {
          sequence: ['/', '+'],
          character: '\u{064e}',
          description: "ARABIC FATHA",
      },
      Digraph {
          sequence: ['\'', '+'],
          character: '\u{064f}',
          description: "ARABIC DAMMA",
      },
      Digraph {
          sequence: ['1', '+'],
          character: '\u{0650}',
          description: "ARABIC KASRA",
      },
      Digraph {
          sequence: ['3', '+'],
          character: '\u{0651}',
          description: "ARABIC SHADDA",
      },
      Digraph {
          sequence: ['0', '+'],
          character: '\u{0652}',
          description: "ARABIC SUKUN",
      },
      Digraph {
          sequence: ['a', 'S'],
          character: '\u{0670}',
          description: "SUPERSCRIPT ARABIC LETTER ALEF",
      },
      Digraph {
          sequence: ['p', '+'],
          character: '\u{067e}',
          description: "ARABIC LETTER PEH",
      },
      Digraph {
          sequence: ['v', '+'],
          character: '\u{06a4}',
          description: "ARABIC LETTER VEH",
      },
      Digraph {
          sequence: ['g', 'f'],
          character: '\u{06af}',
          description: "ARABIC LETTER GAF",
      },
      Digraph {
          sequence: ['0', 'a'],
          character: '\u{06f0}',
          description: "EASTERN ARABIC-INDIC DIGIT ZERO",
      },
      Digraph {
          sequence: ['1', 'a'],
          character: '\u{06f1}',
          description: "EASTERN ARABIC-INDIC DIGIT ONE",
      },
      Digraph {
          sequence: ['2', 'a'],
          character: '\u{06f2}',
          description: "EASTERN ARABIC-INDIC DIGIT TWO",
      },
      Digraph {
          sequence: ['3', 'a'],
          character: '\u{06f3}',
          description: "EASTERN ARABIC-INDIC DIGIT THREE",
      },
      Digraph {
          sequence: ['4', 'a'],
          character: '\u{06f4}',
          description: "EASTERN ARABIC-INDIC DIGIT FOUR",
      },
      Digraph {
          sequence: ['5', 'a'],
          character: '\u{06f5}',
          description: "EASTERN ARABIC-INDIC DIGIT FIVE",
      },
      Digraph {
          sequence: ['6', 'a'],
          character: '\u{06f6}',
          description: "EASTERN ARABIC-INDIC DIGIT SIX",
      },
      Digraph {
          sequence: ['7', 'a'],
          character: '\u{06f7}',
          description: "EASTERN ARABIC-INDIC DIGIT SEVEN",
      },
      Digraph {
          sequence: ['8', 'a'],
          character: '\u{06f8}',
          description: "EASTERN ARABIC-INDIC DIGIT EIGHT",
      },
      Digraph {
          sequence: ['9', 'a'],
          character: '\u{06f9}',
          description: "EASTERN ARABIC-INDIC DIGIT NINE",
      },
      Digraph {
          sequence: ['B', '.'],
          character: '\u{1e02}',
          description: "LATIN CAPITAL LETTER B WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['b', '.'],
          character: '\u{1e03}',
          description: "LATIN SMALL LETTER B WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['B', '_'],
          character: '\u{1e06}',
          description: "LATIN CAPITAL LETTER B WITH LINE BELOW",
      },
      Digraph {
          sequence: ['b', '_'],
          character: '\u{1e07}',
          description: "LATIN SMALL LETTER B WITH LINE BELOW",
      },
      Digraph {
          sequence: ['D', '.'],
          character: '\u{1e0a}',
          description: "LATIN CAPITAL LETTER D WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['d', '.'],
          character: '\u{1e0b}',
          description: "LATIN SMALL LETTER D WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['D', '_'],
          character: '\u{1e0e}',
          description: "LATIN CAPITAL LETTER D WITH LINE BELOW",
      },
      Digraph {
          sequence: ['d', '_'],
          character: '\u{1e0f}',
          description: "LATIN SMALL LETTER D WITH LINE BELOW",
      },
      Digraph {
          sequence: ['D', ','],
          character: '\u{1e10}',
          description: "LATIN CAPITAL LETTER D WITH CEDILLA",
      },
      Digraph {
          sequence: ['d', ','],
          character: '\u{1e11}',
          description: "LATIN SMALL LETTER D WITH CEDILLA",
      },
      Digraph {
          sequence: ['F', '.'],
          character: '\u{1e1e}',
          description: "LATIN CAPITAL LETTER F WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['f', '.'],
          character: '\u{1e1f}',
          description: "LATIN SMALL LETTER F WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['G', '-'],
          character: '\u{1e20}',
          description: "LATIN CAPITAL LETTER G WITH MACRON",
      },
      Digraph {
          sequence: ['g', '-'],
          character: '\u{1e21}',
          description: "LATIN SMALL LETTER G WITH MACRON",
      },
      Digraph {
          sequence: ['H', '.'],
          character: '\u{1e22}',
          description: "LATIN CAPITAL LETTER H WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['h', '.'],
          character: '\u{1e23}',
          description: "LATIN SMALL LETTER H WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['H', ':'],
          character: '\u{1e26}',
          description: "LATIN CAPITAL LETTER H WITH DIAERESIS",
      },
      Digraph {
          sequence: ['h', ':'],
          character: '\u{1e27}',
          description: "LATIN SMALL LETTER H WITH DIAERESIS",
      },
      Digraph {
          sequence: ['H', ','],
          character: '\u{1e28}',
          description: "LATIN CAPITAL LETTER H WITH CEDILLA",
      },
      Digraph {
          sequence: ['h', ','],
          character: '\u{1e29}',
          description: "LATIN SMALL LETTER H WITH CEDILLA",
      },
      Digraph {
          sequence: ['K', '\''],
          character: '\u{1e30}',
          description: "LATIN CAPITAL LETTER K WITH ACUTE",
      },
      Digraph {
          sequence: ['k', '\''],
          character: '\u{1e31}',
          description: "LATIN SMALL LETTER K WITH ACUTE",
      },
      Digraph {
          sequence: ['K', '_'],
          character: '\u{1e34}',
          description: "LATIN CAPITAL LETTER K WITH LINE BELOW",
      },
      Digraph {
          sequence: ['k', '_'],
          character: '\u{1e35}',
          description: "LATIN SMALL LETTER K WITH LINE BELOW",
      },
      Digraph {
          sequence: ['L', '_'],
          character: '\u{1e3a}',
          description: "LATIN CAPITAL LETTER L WITH LINE BELOW",
      },
      Digraph {
          sequence: ['l', '_'],
          character: '\u{1e3b}',
          description: "LATIN SMALL LETTER L WITH LINE BELOW",
      },
      Digraph {
          sequence: ['M', '\''],
          character: '\u{1e3e}',
          description: "LATIN CAPITAL LETTER M WITH ACUTE",
      },
      Digraph {
          sequence: ['m', '\''],
          character: '\u{1e3f}',
          description: "LATIN SMALL LETTER M WITH ACUTE",
      },
      Digraph {
          sequence: ['M', '.'],
          character: '\u{1e40}',
          description: "LATIN CAPITAL LETTER M WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['m', '.'],
          character: '\u{1e41}',
          description: "LATIN SMALL LETTER M WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['N', '.'],
          character: '\u{1e44}',
          description: "LATIN CAPITAL LETTER N WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['n', '.'],
          character: '\u{1e45}',
          description: "LATIN SMALL LETTER N WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['N', '_'],
          character: '\u{1e48}',
          description: "LATIN CAPITAL LETTER N WITH LINE BELOW",
      },
      Digraph {
          sequence: ['n', '_'],
          character: '\u{1e49}',
          description: "LATIN SMALL LETTER N WITH LINE BELOW",
      },
      Digraph {
          sequence: ['P', '\''],
          character: '\u{1e54}',
          description: "LATIN CAPITAL LETTER P WITH ACUTE",
      },
      Digraph {
          sequence: ['p', '\''],
          character: '\u{1e55}',
          description: "LATIN SMALL LETTER P WITH ACUTE",
      },
      Digraph {
          sequence: ['P', '.'],
          character: '\u{1e56}',
          description: "LATIN CAPITAL LETTER P WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['p', '.'],
          character: '\u{1e57}',
          description: "LATIN SMALL LETTER P WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['R', '.'],
          character: '\u{1e58}',
          description: "LATIN CAPITAL LETTER R WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['r', '.'],
          character: '\u{1e59}',
          description: "LATIN SMALL LETTER R WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['R', '_'],
          character: '\u{1e5e}',
          description: "LATIN CAPITAL LETTER R WITH LINE BELOW",
      },
      Digraph {
          sequence: ['r', '_'],
          character: '\u{1e5f}',
          description: "LATIN SMALL LETTER R WITH LINE BELOW",
      },
      Digraph {
          sequence: ['S', '.'],
          character: '\u{1e60}',
          description: "LATIN CAPITAL LETTER S WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['s', '.'],
          character: '\u{1e61}',
          description: "LATIN SMALL LETTER S WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['T', '.'],
          character: '\u{1e6a}',
          description: "LATIN CAPITAL LETTER T WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['t', '.'],
          character: '\u{1e6b}',
          description: "LATIN SMALL LETTER T WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['T', '_'],
          character: '\u{1e6e}',
          description: "LATIN CAPITAL LETTER T WITH LINE BELOW",
      },
      Digraph {
          sequence: ['t', '_'],
          character: '\u{1e6f}',
          description: "LATIN SMALL LETTER T WITH LINE BELOW",
      },
      Digraph {
          sequence: ['V', '?'],
          character: '\u{1e7c}',
          description: "LATIN CAPITAL LETTER V WITH TILDE",
      },
      Digraph {
          sequence: ['v', '?'],
          character: '\u{1e7d}',
          description: "LATIN SMALL LETTER V WITH TILDE",
      },
      Digraph {
          sequence: ['W', '!'],
          character: '\u{1e80}',
          description: "LATIN CAPITAL LETTER W WITH GRAVE",
      },
      Digraph {
          sequence: ['w', '!'],
          character: '\u{1e81}',
          description: "LATIN SMALL LETTER W WITH GRAVE",
      },
      Digraph {
          sequence: ['W', '\''],
          character: '\u{1e82}',
          description: "LATIN CAPITAL LETTER W WITH ACUTE",
      },
      Digraph {
          sequence: ['w', '\''],
          character: '\u{1e83}',
          description: "LATIN SMALL LETTER W WITH ACUTE",
      },
      Digraph {
          sequence: ['W', ':'],
          character: '\u{1e84}',
          description: "LATIN CAPITAL LETTER W WITH DIAERESIS",
      },
      Digraph {
          sequence: ['w', ':'],
          character: '\u{1e85}',
          description: "LATIN SMALL LETTER W WITH DIAERESIS",
      },
      Digraph {
          sequence: ['W', '.'],
          character: '\u{1e86}',
          description: "LATIN CAPITAL LETTER W WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['w', '.'],
          character: '\u{1e87}',
          description: "LATIN SMALL LETTER W WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['X', '.'],
          character: '\u{1e8a}',
          description: "LATIN CAPITAL LETTER X WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['x', '.'],
          character: '\u{1e8b}',
          description: "LATIN SMALL LETTER X WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['X', ':'],
          character: '\u{1e8c}',
          description: "LATIN CAPITAL LETTER X WITH DIAERESIS",
      },
      Digraph {
          sequence: ['x', ':'],
          character: '\u{1e8d}',
          description: "LATIN SMALL LETTER X WITH DIAERESIS",
      },
      Digraph {
          sequence: ['Y', '.'],
          character: '\u{1e8e}',
          description: "LATIN CAPITAL LETTER Y WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['y', '.'],
          character: '\u{1e8f}',
          description: "LATIN SMALL LETTER Y WITH DOT ABOVE",
      },
      Digraph {
          sequence: ['Z', '>'],
          character: '\u{1e90}',
          description: "LATIN CAPITAL LETTER Z WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['z', '>'],
          character: '\u{1e91}',
          description: "LATIN SMALL LETTER Z WITH CIRCUMFLEX",
      },
      Digraph {
          sequence: ['Z', '_'],
          character: '\u{1e94}',
          description: "LATIN CAPITAL LETTER Z WITH LINE BELOW",
      },
      Digraph {
          sequence: ['z', '_'],
          character: '\u{1e95}',
          description: "LATIN SMALL LETTER Z WITH LINE BELOW",
      },
      Digraph {
          sequence: ['h', '_'],
          character: '\u{1e96}',
          description: "LATIN SMALL LETTER H WITH LINE BELOW",
      },
      Digraph {
          sequence: ['t', ':'],
          character: '\u{1e97}',
          description: "LATIN SMALL LETTER T WITH DIAERESIS",
      },
      Digraph {
          sequence: ['w', '0'],
          character: '\u{1e98}',
          description: "LATIN SMALL LETTER W WITH RING ABOVE",
      },
      Digraph {
          sequence: ['y', '0'],
          character: '\u{1e99}',
          description: "LATIN SMALL LETTER Y WITH RING ABOVE",
      },
      Digraph {
          sequence: ['A', '2'],
          character: '\u{1ea2}',
          description: "LATIN CAPITAL LETTER A WITH HOOK ABOVE",
      },
      Digraph {
          sequence: ['a', '2'],
          character: '\u{1ea3}',
          description: "LATIN SMALL LETTER A WITH HOOK ABOVE",
      },
      Digraph {
          sequence: ['E', '2'],
          character: '\u{1eba}',
          description: "LATIN CAPITAL LETTER E WITH HOOK ABOVE",
      },
      Digraph {
          sequence: ['e', '2'],
          character: '\u{1ebb}',
          description: "LATIN SMALL LETTER E WITH HOOK ABOVE",
      },
      Digraph {
          sequence: ['E', '?'],
          character: '\u{1ebc}',
          description: "LATIN CAPITAL LETTER E WITH TILDE",
      },
      Digraph {
          sequence: ['e', '?'],
          character: '\u{1ebd}',
          description: "LATIN SMALL LETTER E WITH TILDE",
      },
      Digraph {
          sequence: ['I', '2'],
          character: '\u{1ec8}',
          description: "LATIN CAPITAL LETTER I WITH HOOK ABOVE",
      },
      Digraph {
          sequence: ['i', '2'],
          character: '\u{1ec9}',
          description: "LATIN SMALL LETTER I WITH HOOK ABOVE",
      },
      Digraph {
          sequence: ['O', '2'],
          character: '\u{1ece}',
          description: "LATIN CAPITAL LETTER O WITH HOOK ABOVE",
      },
      Digraph {
          sequence: ['o', '2'],
          character: '\u{1ecf}',
          description: "LATIN SMALL LETTER O WITH HOOK ABOVE",
      },
      Digraph {
          sequence: ['U', '2'],
          character: '\u{1ee6}',
          description: "LATIN CAPITAL LETTER U WITH HOOK ABOVE",
      },
      Digraph {
          sequence: ['u', '2'],
          character: '\u{1ee7}',
          description: "LATIN SMALL LETTER U WITH HOOK ABOVE",
      },
      Digraph {
          sequence: ['Y', '!'],
          character: '\u{1ef2}',
          description: "LATIN CAPITAL LETTER Y WITH GRAVE",
      },
      Digraph {
          sequence: ['y', '!'],
          character: '\u{1ef3}',
          description: "LATIN SMALL LETTER Y WITH GRAVE",
      },
      Digraph {
          sequence: ['Y', '2'],
          character: '\u{1ef6}',
          description: "LATIN CAPITAL LETTER Y WITH HOOK ABOVE",
      },
      Digraph {
          sequence: ['y', '2'],
          character: '\u{1ef7}',
          description: "LATIN SMALL LETTER Y WITH HOOK ABOVE",
      },
      Digraph {
          sequence: ['Y', '?'],
          character: '\u{1ef8}',
          description: "LATIN CAPITAL LETTER Y WITH TILDE",
      },
      Digraph {
          sequence: ['y', '?'],
          character: '\u{1ef9}',
          description: "LATIN SMALL LETTER Y WITH TILDE",
      },
      Digraph {
          sequence: [';', '\''],
          character: '\u{1f00}',
          description: "GREEK DASIA AND ACUTE ACCENT",
      },
      Digraph {
          sequence: [',', '\''],
          character: '\u{1f01}',
          description: "GREEK PSILI AND ACUTE ACCENT",
      },
      Digraph {
          sequence: [';', '!'],
          character: '\u{1f02}',
          description: "GREEK DASIA AND VARIA",
      },
      Digraph {
          sequence: [',', '!'],
          character: '\u{1f03}',
          description: "GREEK PSILI AND VARIA",
      },
      Digraph {
          sequence: ['?', ';'],
          character: '\u{1f04}',
          description: "GREEK DASIA AND PERISPOMENI",
      },
      Digraph {
          sequence: ['?', ','],
          character: '\u{1f05}',
          description: "GREEK PSILI AND PERISPOMENI",
      },
      Digraph {
          sequence: ['!', ':'],
          character: '\u{1f06}',
          description: "GREEK DIAERESIS AND VARIA",
      },
      Digraph {
          sequence: ['?', ':'],
          character: '\u{1f07}',
          description: "GREEK DIAERESIS AND PERISPOMENI",
      },
      Digraph {
          sequence: ['1', 'N'],
          character: '\u{2002}',
          description: "EN SPACE",
      },
      Digraph {
          sequence: ['1', 'M'],
          character: '\u{2003}',
          description: "EM SPACE",
      },
      Digraph {
          sequence: ['3', 'M'],
          character: '\u{2004}',
          description: "THREE-PER-EM SPACE",
      },
      Digraph {
          sequence: ['4', 'M'],
          character: '\u{2005}',
          description: "FOUR-PER-EM SPACE",
      },
      Digraph {
          sequence: ['6', 'M'],
          character: '\u{2006}',
          description: "SIX-PER-EM SPACE",
      },
      Digraph {
          sequence: ['1', 'T'],
          character: '\u{2009}',
          description: "THIN SPACE",
      },
      Digraph {
          sequence: ['1', 'H'],
          character: '\u{200a}',
          description: "HAIR SPACE",
      },
      Digraph {
          sequence: ['-', '1'],
          character: '\u{2010}',
          description: "HYPHEN",
      },
      Digraph {
          sequence: ['-', 'N'],
          character: '\u{2013}',
          description: "EN DASH",
      },
      Digraph {
          sequence: ['-', 'M'],
          character: '\u{2014}',
          description: "EM DASH",
      },
      Digraph {
          sequence: ['-', '3'],
          character: '\u{2015}',
          description: "HORIZONTAL BAR",
      },
      Digraph {
          sequence: ['!', '2'],
          character: '\u{2016}',
          description: "DOUBLE VERTICAL LINE",
      },
      Digraph {
          sequence: ['=', '2'],
          character: '\u{2017}',
          description: "DOUBLE LOW LINE",
      },
      Digraph {
          sequence: ['\'', '6'],
          character: '\u{2018}',
          description: "LEFT SINGLE QUOTATION MARK",
      },
      Digraph {
          sequence: ['\'', '9'],
          character: '\u{2019}',
          description: "RIGHT SINGLE QUOTATION MARK",
      },
      Digraph {
          sequence: ['.', '9'],
          character: '\u{201a}',
          description: "SINGLE LOW-9 QUOTATION MARK",
      },
      Digraph {
          sequence: ['9', '\''],
          character: '\u{201b}',
          description: "SINGLE HIGH-REVERSED-9 QUOTATION MARK",
      },
      Digraph {
          sequence: ['"', '6'],
          character: '\u{201c}',
          description: "LEFT DOUBLE QUOTATION MARK",
      },
      Digraph {
          sequence: ['"', '9'],
          character: '\u{201d}',
          description: "RIGHT DOUBLE QUOTATION MARK",
      },
      Digraph {
          sequence: [':', '9'],
          character: '\u{201e}',
          description: "DOUBLE LOW-9 QUOTATION MARK",
      },
      Digraph {
          sequence: ['9', '"'],
          character: '\u{201f}',
          description: "DOUBLE HIGH-REVERSED-9 QUOTATION MARK",
      },
      Digraph {
          sequence: ['/', '-'],
          character: '\u{2020}',
          description: "DAGGER",
      },
      Digraph {
          sequence: ['/', '='],
          character: '\u{2021}',
          description: "DOUBLE DAGGER",
      },
      Digraph {
          sequence: ['.', '.'],
          character: '\u{2025}',
          description: "TWO DOT LEADER",
      },
      Digraph {
          sequence: ['%', '0'],
          character: '\u{2030}',
          description: "PER MILLE SIGN",
      },
      Digraph {
          sequence: ['1', '\''],
          character: '\u{2032}',
          description: "PRIME",
      },
      Digraph {
          sequence: ['2', '\''],
          character: '\u{2033}',
          description: "DOUBLE PRIME",
      },
      Digraph {
          sequence: ['3', '\''],
          character: '\u{2034}',
          description: "TRIPLE PRIME",
      },
      Digraph {
          sequence: ['1', '"'],
          character: '\u{2035}',
          description: "REVERSED PRIME",
      },
      Digraph {
          sequence: ['2', '"'],
          character: '\u{2036}',
          description: "REVERSED DOUBLE PRIME",
      },
      Digraph {
          sequence: ['3', '"'],
          character: '\u{2037}',
          description: "REVERSED TRIPLE PRIME",
      },
      Digraph {
          sequence: ['C', 'a'],
          character: '\u{2038}',
          description: "CARET",
      },
      Digraph {
          sequence: ['<', '1'],
          character: '\u{2039}',
          description: "SINGLE LEFT-POINTING ANGLE QUOTATION MARK",
      },
      Digraph {
          sequence: ['>', '1'],
          character: '\u{203a}',
          description: "SINGLE RIGHT-POINTING ANGLE QUOTATION MARK",
      },
      Digraph {
          sequence: [':', 'X'],
          character: '\u{203b}',
          description: "REFERENCE MARK",
      },
      Digraph {
          sequence: ['\'', '-'],
          character: '\u{203e}',
          description: "OVERLINE",
      },
      Digraph {
          sequence: ['/', 'f'],
          character: '\u{2044}',
          description: "FRACTION SLASH",
      },
      Digraph {
          sequence: ['0', 'S'],
          character: '\u{2070}',
          description: "SUPERSCRIPT DIGIT ZERO",
      },
      Digraph {
          sequence: ['4', 'S'],
          character: '\u{2074}',
          description: "SUPERSCRIPT DIGIT FOUR",
      },
      Digraph {
          sequence: ['5', 'S'],
          character: '\u{2075}',
          description: "SUPERSCRIPT DIGIT FIVE",
      },
      Digraph {
          sequence: ['6', 'S'],
          character: '\u{2076}',
          description: "SUPERSCRIPT DIGIT SIX",
      },
      Digraph {
          sequence: ['7', 'S'],
          character: '\u{2077}',
          description: "SUPERSCRIPT DIGIT SEVEN",
      },
      Digraph {
          sequence: ['8', 'S'],
          character: '\u{2078}',
          description: "SUPERSCRIPT DIGIT EIGHT",
      },
      Digraph {
          sequence: ['9', 'S'],
          character: '\u{2079}',
          description: "SUPERSCRIPT DIGIT NINE",
      },
      Digraph {
          sequence: ['+', 'S'],
          character: '\u{207a}',
          description: "SUPERSCRIPT PLUS SIGN",
      },
      Digraph {
          sequence: ['-', 'S'],
          character: '\u{207b}',
          description: "SUPERSCRIPT MINUS",
      },
      Digraph {
          sequence: ['=', 'S'],
          character: '\u{207c}',
          description: "SUPERSCRIPT EQUALS SIGN",
      },
      Digraph {
          sequence: ['(', 'S'],
          character: '\u{207d}',
          description: "SUPERSCRIPT LEFT PARENTHESIS",
      },
      Digraph {
          sequence: [')', 'S'],
          character: '\u{207e}',
          description: "SUPERSCRIPT RIGHT PARENTHESIS",
      },
      Digraph {
          sequence: ['n', 'S'],
          character: '\u{207f}',
          description: "SUPERSCRIPT LATIN SMALL LETTER N",
      },
      Digraph {
          sequence: ['0', 's'],
          character: '\u{2080}',
          description: "SUBSCRIPT DIGIT ZERO",
      },
      Digraph {
          sequence: ['1', 's'],
          character: '\u{2081}',
          description: "SUBSCRIPT DIGIT ONE",
      },
      Digraph {
          sequence: ['2', 's'],
          character: '\u{2082}',
          description: "SUBSCRIPT DIGIT TWO",
      },
      Digraph {
          sequence: ['3', 's'],
          character: '\u{2083}',
          description: "SUBSCRIPT DIGIT THREE",
      },
      Digraph {
          sequence: ['4', 's'],
          character: '\u{2084}',
          description: "SUBSCRIPT DIGIT FOUR",
      },
      Digraph {
          sequence: ['5', 's'],
          character: '\u{2085}',
          description: "SUBSCRIPT DIGIT FIVE",
      },
      Digraph {
          sequence: ['6', 's'],
          character: '\u{2086}',
          description: "SUBSCRIPT DIGIT SIX",
      },
      Digraph {
          sequence: ['7', 's'],
          character: '\u{2087}',
          description: "SUBSCRIPT DIGIT SEVEN",
      },
      Digraph {
          sequence: ['8', 's'],
          character: '\u{2088}',
          description: "SUBSCRIPT DIGIT EIGHT",
      },
      Digraph {
          sequence: ['9', 's'],
          character: '\u{2089}',
          description: "SUBSCRIPT DIGIT NINE",
      },
      Digraph {
          sequence: ['+', 's'],
          character: '\u{208a}',
          description: "SUBSCRIPT PLUS SIGN",
      },
      Digraph {
          sequence: ['-', 's'],
          character: '\u{208b}',
          description: "SUBSCRIPT MINUS",
      },
      Digraph {
          sequence: ['=', 's'],
          character: '\u{208c}',
          description: "SUBSCRIPT EQUALS SIGN",
      },
      Digraph {
          sequence: ['(', 's'],
          character: '\u{208d}',
          description: "SUBSCRIPT LEFT PARENTHESIS",
      },
      Digraph {
          sequence: [')', 's'],
          character: '\u{208e}',
          description: "SUBSCRIPT RIGHT PARENTHESIS",
      },
      Digraph {
          sequence: ['L', 'i'],
          character: '\u{20a4}',
          description: "LIRA SIGN",
      },
      Digraph {
          sequence: ['P', 't'],
          character: '\u{20a7}',
          description: "PESETA SIGN",
      },
      Digraph {
          sequence: ['W', '='],
          character: '\u{20a9}',
          description: "WON SIGN",
      },
      Digraph {
          sequence: ['o', 'C'],
          character: '\u{2103}',
          description: "DEGREE CENTIGRADE",
      },
      Digraph {
          sequence: ['c', 'o'],
          character: '\u{2105}',
          description: "CARE OF",
      },
      Digraph {
          sequence: ['o', 'F'],
          character: '\u{2109}',
          description: "DEGREE FAHRENHEIT",
      },
      Digraph {
          sequence: ['N', '0'],
          character: '\u{2116}',
          description: "NUMERO SIGN",
      },
      Digraph {
          sequence: ['P', 'O'],
          character: '\u{2117}',
          description: "SOUND RECORDING COPYRIGHT",
      },
      Digraph {
          sequence: ['R', 'x'],
          character: '\u{211e}',
          description: "PRESCRIPTION TAKE",
      },
      Digraph {
          sequence: ['S', 'M'],
          character: '\u{2120}',
          description: "SERVICE MARK",
      },
      Digraph {
          sequence: ['T', 'M'],
          character: '\u{2122}',
          description: "TRADE MARK SIGN",
      },
      Digraph {
          sequence: ['O', 'm'],
          character: '\u{2126}',
          description: "OHM SIGN",
      },
      Digraph {
          sequence: ['A', 'O'],
          character: '\u{212b}',
          description: "ANGSTROEM SIGN",
      },
      Digraph {
          sequence: ['1', '3'],
          character: '\u{2153}',
          description: "VULGAR FRACTION ONE THIRD",
      },
      Digraph {
          sequence: ['2', '3'],
          character: '\u{2154}',
          description: "VULGAR FRACTION TWO THIRDS",
      },
      Digraph {
          sequence: ['1', '5'],
          character: '\u{2155}',
          description: "VULGAR FRACTION ONE FIFTH",
      },
      Digraph {
          sequence: ['2', '5'],
          character: '\u{2156}',
          description: "VULGAR FRACTION TWO FIFTHS",
      },
      Digraph {
          sequence: ['3', '5'],
          character: '\u{2157}',
          description: "VULGAR FRACTION THREE FIFTHS",
      },
      Digraph {
          sequence: ['4', '5'],
          character: '\u{2158}',
          description: "VULGAR FRACTION FOUR FIFTHS",
      },
      Digraph {
          sequence: ['1', '6'],
          character: '\u{2159}',
          description: "VULGAR FRACTION ONE SIXTH",
      },
      Digraph {
          sequence: ['5', '6'],
          character: '\u{215a}',
          description: "VULGAR FRACTION FIVE SIXTHS",
      },
      Digraph {
          sequence: ['1', '8'],
          character: '\u{215b}',
          description: "VULGAR FRACTION ONE EIGHTH",
      },
      Digraph {
          sequence: ['3', '8'],
          character: '\u{215c}',
          description: "VULGAR FRACTION THREE EIGHTHS",
      },
      Digraph {
          sequence: ['5', '8'],
          character: '\u{215d}',
          description: "VULGAR FRACTION FIVE EIGHTHS",
      },
      Digraph {
          sequence: ['7', '8'],
          character: '\u{215e}',
          description: "VULGAR FRACTION SEVEN EIGHTHS",
      },
      Digraph {
          sequence: ['1', 'R'],
          character: '\u{2160}',
          description: "ROMAN NUMERAL ONE",
      },
      Digraph {
          sequence: ['2', 'R'],
          character: '\u{2161}',
          description: "ROMAN NUMERAL TWO",
      },
      Digraph {
          sequence: ['3', 'R'],
          character: '\u{2162}',
          description: "ROMAN NUMERAL THREE",
      },
      Digraph {
          sequence: ['4', 'R'],
          character: '\u{2163}',
          description: "ROMAN NUMERAL FOUR",
      },
      Digraph {
          sequence: ['5', 'R'],
          character: '\u{2164}',
          description: "ROMAN NUMERAL FIVE",
      },
      Digraph {
          sequence: ['6', 'R'],
          character: '\u{2165}',
          description: "ROMAN NUMERAL SIX",
      },
      Digraph {
          sequence: ['7', 'R'],
          character: '\u{2166}',
          description: "ROMAN NUMERAL SEVEN",
      },
      Digraph {
          sequence: ['8', 'R'],
          character: '\u{2167}',
          description: "ROMAN NUMERAL EIGHT",
      },
      Digraph {
          sequence: ['9', 'R'],
          character: '\u{2168}',
          description: "ROMAN NUMERAL NINE",
      },
      Digraph {
          sequence: ['a', 'R'],
          character: '\u{2169}',
          description: "ROMAN NUMERAL TEN",
      },
      Digraph {
          sequence: ['b', 'R'],
          character: '\u{216a}',
          description: "ROMAN NUMERAL ELEVEN",
      },
      Digraph {
          sequence: ['c', 'R'],
          character: '\u{216b}',
          description: "ROMAN NUMERAL TWELVE",
      },
      Digraph {
          sequence: ['1', 'r'],
          character: '\u{2170}',
          description: "SMALL ROMAN NUMERAL ONE",
      },
      Digraph {
          sequence: ['2', 'r'],
          character: '\u{2171}',
          description: "SMALL ROMAN NUMERAL TWO",
      },
      Digraph {
          sequence: ['3', 'r'],
          character: '\u{2172}',
          description: "SMALL ROMAN NUMERAL THREE",
      },
      Digraph {
          sequence: ['4', 'r'],
          character: '\u{2173}',
          description: "SMALL ROMAN NUMERAL FOUR",
      },
      Digraph {
          sequence: ['5', 'r'],
          character: '\u{2174}',
          description: "SMALL ROMAN NUMERAL FIVE",
      },
      Digraph {
          sequence: ['6', 'r'],
          character: '\u{2175}',
          description: "SMALL ROMAN NUMERAL SIX",
      },
      Digraph {
          sequence: ['7', 'r'],
          character: '\u{2176}',
          description: "SMALL ROMAN NUMERAL SEVEN",
      },
      Digraph {
          sequence: ['8', 'r'],
          character: '\u{2177}',
          description: "SMALL ROMAN NUMERAL EIGHT",
      },
      Digraph {
          sequence: ['9', 'r'],
          character: '\u{2178}',
          description: "SMALL ROMAN NUMERAL NINE",
      },
      Digraph {
          sequence: ['a', 'r'],
          character: '\u{2179}',
          description: "SMALL ROMAN NUMERAL TEN",
      },
      Digraph {
          sequence: ['b', 'r'],
          character: '\u{217a}',
          description: "SMALL ROMAN NUMERAL ELEVEN",
      },
      Digraph {
          sequence: ['c', 'r'],
          character: '\u{217b}',
          description: "SMALL ROMAN NUMERAL TWELVE",
      },
      Digraph {
          sequence: ['<', '-'],
          character: '\u{2190}',
          description: "LEFTWARDS ARROW",
      },
      Digraph {
          sequence: ['-', '!'],
          character: '\u{2191}',
          description: "UPWARDS ARROW",
      },
      Digraph {
          sequence: ['-', '>'],
          character: '\u{2192}',
          description: "RIGHTWARDS ARROW",
      },
      Digraph {
          sequence: ['-', 'v'],
          character: '\u{2193}',
          description: "DOWNWARDS ARROW",
      },
      Digraph {
          sequence: ['<', '>'],
          character: '\u{2194}',
          description: "LEFT RIGHT ARROW",
      },
      Digraph {
          sequence: ['U', 'D'],
          character: '\u{2195}',
          description: "UP DOWN ARROW",
      },
      Digraph {
          sequence: ['<', '='],
          character: '\u{21d0}',
          description: "LEFTWARDS DOUBLE ARROW",
      },
      Digraph {
          sequence: ['=', '>'],
          character: '\u{21d2}',
          description: "RIGHTWARDS DOUBLE ARROW",
      },
      Digraph {
          sequence: ['=', '='],
          character: '\u{21d4}',
          description: "LEFT RIGHT DOUBLE ARROW",
      },
      Digraph {
          sequence: ['F', 'A'],
          character: '\u{2200}',
          description: "FOR ALL",
      },
      Digraph {
          sequence: ['d', 'P'],
          character: '\u{2202}',
          description: "PARTIAL DIFFERENTIAL",
      },
      Digraph {
          sequence: ['T', 'E'],
          character: '\u{2203}',
          description: "THERE EXISTS",
      },
      Digraph {
          sequence: ['/', '0'],
          character: '\u{2205}',
          description: "EMPTY SET",
      },
      Digraph {
          sequence: ['D', 'E'],
          character: '\u{2206}',
          description: "INCREMENT",
      },
      Digraph {
          sequence: ['N', 'B'],
          character: '\u{2207}',
          description: "NABLA",
      },
      Digraph {
          sequence: ['(', '-'],
          character: '\u{2208}',
          description: "ELEMENT OF",
      },
      Digraph {
          sequence: ['-', ')'],
          character: '\u{220b}',
          description: "CONTAINS AS MEMBER",
      },
      Digraph {
          sequence: ['*', 'P'],
          character: '\u{220f}',
          description: "N-ARY PRODUCT",
      },
      Digraph {
          sequence: ['+', 'Z'],
          character: '\u{2211}',
          description: "N-ARY SUMMATION",
      },
      Digraph {
          sequence: ['-', '2'],
          character: '\u{2212}',
          description: "MINUS SIGN",
      },
      Digraph {
          sequence: ['-', '+'],
          character: '\u{2213}',
          description: "MINUS-OR-PLUS SIGN",
      },
      Digraph {
          sequence: ['*', '-'],
          character: '\u{2217}',
          description: "ASTERISK OPERATOR",
      },
      Digraph {
          sequence: ['O', 'b'],
          character: '\u{2218}',
          description: "RING OPERATOR",
      },
      Digraph {
          sequence: ['S', 'b'],
          character: '\u{2219}',
          description: "BULLET OPERATOR",
      },
      Digraph {
          sequence: ['R', 'T'],
          character: '\u{221a}',
          description: "SQUARE ROOT",
      },
      Digraph {
          sequence: ['0', '('],
          character: '\u{221d}',
          description: "PROPORTIONAL TO",
      },
      Digraph {
          sequence: ['0', '0'],
          character: '\u{221e}',
          description: "INFINITY",
      },
      Digraph {
          sequence: ['-', 'L'],
          character: '\u{221f}',
          description: "RIGHT ANGLE",
      },
      Digraph {
          sequence: ['-', 'V'],
          character: '\u{2220}',
          description: "ANGLE",
      },
      Digraph {
          sequence: ['P', 'P'],
          character: '\u{2225}',
          description: "PARALLEL TO",
      },
      Digraph {
          sequence: ['A', 'N'],
          character: '\u{2227}',
          description: "LOGICAL AND",
      },
      Digraph {
          sequence: ['O', 'R'],
          character: '\u{2228}',
          description: "LOGICAL OR",
      },
      Digraph {
          sequence: ['(', 'U'],
          character: '\u{2229}',
          description: "INTERSECTION",
      },
      Digraph {
          sequence: [')', 'U'],
          character: '\u{222a}',
          description: "UNION",
      },
      Digraph {
          sequence: ['I', 'n'],
          character: '\u{222b}',
          description: "INTEGRAL",
      },
      Digraph {
          sequence: ['D', 'I'],
          character: '\u{222c}',
          description: "DOUBLE INTEGRAL",
      },
      Digraph {
          sequence: ['I', 'o'],
          character: '\u{222e}',
          description: "CONTOUR INTEGRAL",
      },
      Digraph {
          sequence: ['.', ':'],
          character: '\u{2234}',
          description: "THEREFORE",
      },
      Digraph {
          sequence: [':', '.'],
          character: '\u{2235}',
          description: "BECAUSE",
      },
      Digraph {
          sequence: [':', 'R'],
          character: '\u{2236}',
          description: "RATIO",
      },
      Digraph {
          sequence: [':', ':'],
          character: '\u{2237}',
          description: "PROPORTION",
      },
      Digraph {
          sequence: ['?', '1'],
          character: '\u{223c}',
          description: "TILDE OPERATOR",
      },
      Digraph {
          sequence: ['C', 'G'],
          character: '\u{223e}',
          description: "INVERTED LAZY S",
      },
      Digraph {
          sequence: ['?', '-'],
          character: '\u{2243}',
          description: "ASYMPTOTICALLY EQUAL TO",
      },
      Digraph {
          sequence: ['?', '='],
          character: '\u{2245}',
          description: "APPROXIMATELY EQUAL TO",
      },
      Digraph {
          sequence: ['?', '2'],
          character: '\u{2248}',
          description: "ALMOST EQUAL TO",
      },
      Digraph {
          sequence: ['=', '?'],
          character: '\u{224c}',
          description: "ALL EQUAL TO",
      },
      Digraph {
          sequence: ['H', 'I'],
          character: '\u{2253}',
          description: "IMAGE OF OR APPROXIMATELY EQUAL TO",
      },
      Digraph {
          sequence: ['!', '='],
          character: '\u{2260}',
          description: "NOT EQUAL TO",
      },
      Digraph {
          sequence: ['=', '3'],
          character: '\u{2261}',
          description: "IDENTICAL TO",
      },
      Digraph {
          sequence: ['=', '<'],
          character: '\u{2264}',
          description: "LESS-THAN OR EQUAL TO",
      },
      Digraph {
          sequence: ['>', '='],
          character: '\u{2265}',
          description: "GREATER-THAN OR EQUAL TO",
      },
      Digraph {
          sequence: ['<', '*'],
          character: '\u{226a}',
          description: "MUCH LESS-THAN",
      },
      Digraph {
          sequence: ['*', '>'],
          character: '\u{226b}',
          description: "MUCH GREATER-THAN",
      },
      Digraph {
          sequence: ['!', '<'],
          character: '\u{226e}',
          description: "NOT LESS-THAN",
      },
      Digraph {
          sequence: ['!', '>'],
          character: '\u{226f}',
          description: "NOT GREATER-THAN",
      },
      Digraph {
          sequence: ['(', 'C'],
          character: '\u{2282}',
          description: "SUBSET OF",
      },
      Digraph {
          sequence: [')', 'C'],
          character: '\u{2283}',
          description: "SUPERSET OF",
      },
      Digraph {
          sequence: ['(', '_'],
          character: '\u{2286}',
          description: "SUBSET OF OR EQUAL TO",
      },
      Digraph {
          sequence: [')', '_'],
          character: '\u{2287}',
          description: "SUPERSET OF OR EQUAL TO",
      },
      Digraph {
          sequence: ['0', '.'],
          character: '\u{2299}',
          description: "CIRCLED DOT OPERATOR",
      },
      Digraph {
          sequence: ['0', '2'],
          character: '\u{229a}',
          description: "CIRCLED RING OPERATOR",
      },
      Digraph {
          sequence: ['-', 'T'],
          character: '\u{22a5}',
          description: "UP TACK",
      },
      Digraph {
          sequence: ['.', 'P'],
          character: '\u{22c5}',
          description: "DOT OPERATOR",
      },
      Digraph {
          sequence: [':', '3'],
          character: '\u{22ee}',
          description: "VERTICAL ELLIPSIS",
      },
      Digraph {
          sequence: ['.', '3'],
          character: '\u{22ef}',
          description: "MIDLINE HORIZONTAL ELLIPSIS",
      },
      Digraph {
          sequence: ['E', 'h'],
          character: '\u{2302}',
          description: "HOUSE",
      },
      Digraph {
          sequence: ['<', '7'],
          character: '\u{2308}',
          description: "LEFT CEILING",
      },
      Digraph {
          sequence: ['>', '7'],
          character: '\u{2309}',
          description: "RIGHT CEILING",
      },
      Digraph {
          sequence: ['7', '<'],
          character: '\u{230a}',
          description: "LEFT FLOOR",
      },
      Digraph {
          sequence: ['7', '>'],
          character: '\u{230b}',
          description: "RIGHT FLOOR",
      },
      Digraph {
          sequence: ['N', 'I'],
          character: '\u{2310}',
          description: "REVERSED NOT SIGN",
      },
      Digraph {
          sequence: ['(', 'A'],
          character: '\u{2312}',
          description: "ARC",
      },
      Digraph {
          sequence: ['T', 'R'],
          character: '\u{2315}',
          description: "TELEPHONE RECORDER",
      },
      Digraph {
          sequence: ['I', 'u'],
          character: '\u{2320}',
          description: "TOP HALF INTEGRAL",
      },
      Digraph {
          sequence: ['I', 'l'],
          character: '\u{2321}',
          description: "BOTTOM HALF INTEGRAL",
      },
      Digraph {
          sequence: ['<', '/'],
          character: '\u{2329}',
          description: "LEFT-POINTING ANGLE BRACKET",
      },
      Digraph {
          sequence: ['/', '>'],
          character: '\u{232a}',
          description: "RIGHT-POINTING ANGLE BRACKET",
      },
      Digraph {
          sequence: ['V', 's'],
          character: '\u{2423}',
          description: "OPEN BOX",
      },
      Digraph {
          sequence: ['1', 'h'],
          character: '\u{2440}',
          description: "OCR HOOK",
      },
      Digraph {
          sequence: ['3', 'h'],
          character: '\u{2441}',
          description: "OCR CHAIR",
      },
      Digraph {
          sequence: ['2', 'h'],
          character: '\u{2442}',
          description: "OCR FORK",
      },
      Digraph {
          sequence: ['4', 'h'],
          character: '\u{2443}',
          description: "OCR INVERTED FORK",
      },
      Digraph {
          sequence: ['1', 'j'],
          character: '\u{2446}',
          description: "OCR BRANCH BANK IDENTIFICATION",
      },
      Digraph {
          sequence: ['2', 'j'],
          character: '\u{2447}',
          description: "OCR AMOUNT OF CHECK",
      },
      Digraph {
          sequence: ['3', 'j'],
          character: '\u{2448}',
          description: "OCR DASH",
      },
      Digraph {
          sequence: ['4', 'j'],
          character: '\u{2449}',
          description: "OCR CUSTOMER ACCOUNT NUMBER",
      },
      Digraph {
          sequence: ['1', '.'],
          character: '\u{2488}',
          description: "DIGIT ONE FULL STOP",
      },
      Digraph {
          sequence: ['2', '.'],
          character: '\u{2489}',
          description: "DIGIT TWO FULL STOP",
      },
      Digraph {
          sequence: ['3', '.'],
          character: '\u{248a}',
          description: "DIGIT THREE FULL STOP",
      },
      Digraph {
          sequence: ['4', '.'],
          character: '\u{248b}',
          description: "DIGIT FOUR FULL STOP",
      },
      Digraph {
          sequence: ['5', '.'],
          character: '\u{248c}',
          description: "DIGIT FIVE FULL STOP",
      },
      Digraph {
          sequence: ['6', '.'],
          character: '\u{248d}',
          description: "DIGIT SIX FULL STOP",
      },
      Digraph {
          sequence: ['7', '.'],
          character: '\u{248e}',
          description: "DIGIT SEVEN FULL STOP",
      },
      Digraph {
          sequence: ['8', '.'],
          character: '\u{248f}',
          description: "DIGIT EIGHT FULL STOP",
      },
      Digraph {
          sequence: ['9', '.'],
          character: '\u{2490}',
          description: "DIGIT NINE FULL STOP",
      },
      Digraph {
          sequence: ['h', 'h'],
          character: '\u{2500}',
          description: "BOX DRAWINGS LIGHT HORIZONTAL",
      },
      Digraph {
          sequence: ['H', 'H'],
          character: '\u{2501}',
          description: "BOX DRAWINGS HEAVY HORIZONTAL",
      },
      Digraph {
          sequence: ['v', 'v'],
          character: '\u{2502}',
          description: "BOX DRAWINGS LIGHT VERTICAL",
      },
      Digraph {
          sequence: ['V', 'V'],
          character: '\u{2503}',
          description: "BOX DRAWINGS HEAVY VERTICAL",
      },
      Digraph {
          sequence: ['3', '-'],
          character: '\u{2504}',
          description: "BOX DRAWINGS LIGHT TRIPLE DASH HORIZONTAL",
      },
      Digraph {
          sequence: ['3', '_'],
          character: '\u{2505}',
          description: "BOX DRAWINGS HEAVY TRIPLE DASH HORIZONTAL",
      },
      Digraph {
          sequence: ['3', '!'],
          character: '\u{2506}',
          description: "BOX DRAWINGS LIGHT TRIPLE DASH VERTICAL",
      },
      Digraph {
          sequence: ['3', '/'],
          character: '\u{2507}',
          description: "BOX DRAWINGS HEAVY TRIPLE DASH VERTICAL",
      },
      Digraph {
          sequence: ['4', '-'],
          character: '\u{2508}',
          description: "BOX DRAWINGS LIGHT QUADRUPLE DASH HORIZONTAL",
      },
      Digraph {
          sequence: ['4', '_'],
          character: '\u{2509}',
          description: "BOX DRAWINGS HEAVY QUADRUPLE DASH HORIZONTAL",
      },
      Digraph {
          sequence: ['4', '!'],
          character: '\u{250a}',
          description: "BOX DRAWINGS LIGHT QUADRUPLE DASH VERTICAL",
      },
      Digraph {
          sequence: ['4', '/'],
          character: '\u{250b}',
          description: "BOX DRAWINGS HEAVY QUADRUPLE DASH VERTICAL",
      },
      Digraph {
          sequence: ['d', 'r'],
          character: '\u{250c}',
          description: "BOX DRAWINGS LIGHT DOWN AND RIGHT",
      },
      Digraph {
          sequence: ['d', 'R'],
          character: '\u{250d}',
          description: "BOX DRAWINGS DOWN LIGHT AND RIGHT HEAVY",
      },
      Digraph {
          sequence: ['D', 'r'],
          character: '\u{250e}',
          description: "BOX DRAWINGS DOWN HEAVY AND RIGHT LIGHT",
      },
      Digraph {
          sequence: ['D', 'R'],
          character: '\u{250f}',
          description: "BOX DRAWINGS HEAVY DOWN AND RIGHT",
      },
      Digraph {
          sequence: ['d', 'l'],
          character: '\u{2510}',
          description: "BOX DRAWINGS LIGHT DOWN AND LEFT",
      },
      Digraph {
          sequence: ['d', 'L'],
          character: '\u{2511}',
          description: "BOX DRAWINGS DOWN LIGHT AND LEFT HEAVY",
      },
      Digraph {
          sequence: ['D', 'l'],
          character: '\u{2512}',
          description: "BOX DRAWINGS DOWN HEAVY AND LEFT LIGHT",
      },
      Digraph {
          sequence: ['L', 'D'],
          character: '\u{2513}',
          description: "BOX DRAWINGS HEAVY DOWN AND LEFT",
      },
      Digraph {
          sequence: ['u', 'r'],
          character: '\u{2514}',
          description: "BOX DRAWINGS LIGHT UP AND RIGHT",
      },
      Digraph {
          sequence: ['u', 'R'],
          character: '\u{2515}',
          description: "BOX DRAWINGS UP LIGHT AND RIGHT HEAVY",
      },
      Digraph {
          sequence: ['U', 'r'],
          character: '\u{2516}',
          description: "BOX DRAWINGS UP HEAVY AND RIGHT LIGHT",
      },
      Digraph {
          sequence: ['U', 'R'],
          character: '\u{2517}',
          description: "BOX DRAWINGS HEAVY UP AND RIGHT",
      },
      Digraph {
          sequence: ['u', 'l'],
          character: '\u{2518}',
          description: "BOX DRAWINGS LIGHT UP AND LEFT",
      },
      Digraph {
          sequence: ['u', 'L'],
          character: '\u{2519}',
          description: "BOX DRAWINGS UP LIGHT AND LEFT HEAVY",
      },
      Digraph {
          sequence: ['U', 'l'],
          character: '\u{251a}',
          description: "BOX DRAWINGS UP HEAVY AND LEFT LIGHT",
      },
      Digraph {
          sequence: ['U', 'L'],
          character: '\u{251b}',
          description: "BOX DRAWINGS HEAVY UP AND LEFT",
      },
      Digraph {
          sequence: ['v', 'r'],
          character: '\u{251c}',
          description: "BOX DRAWINGS LIGHT VERTICAL AND RIGHT",
      },
      Digraph {
          sequence: ['v', 'R'],
          character: '\u{251d}',
          description: "BOX DRAWINGS VERTICAL LIGHT AND RIGHT HEAVY",
      },
      Digraph {
          sequence: ['V', 'r'],
          character: '\u{2520}',
          description: "BOX DRAWINGS VERTICAL HEAVY AND RIGHT LIGHT",
      },
      Digraph {
          sequence: ['V', 'R'],
          character: '\u{2523}',
          description: "BOX DRAWINGS HEAVY VERTICAL AND RIGHT",
      },
      Digraph {
          sequence: ['v', 'l'],
          character: '\u{2524}',
          description: "BOX DRAWINGS LIGHT VERTICAL AND LEFT",
      },
      Digraph {
          sequence: ['v', 'L'],
          character: '\u{2525}',
          description: "BOX DRAWINGS VERTICAL LIGHT AND LEFT HEAVY",
      },
      Digraph {
          sequence: ['V', 'l'],
          character: '\u{2528}',
          description: "BOX DRAWINGS VERTICAL HEAVY AND LEFT LIGHT",
      },
      Digraph {
          sequence: ['V', 'L'],
          character: '\u{252b}',
          description: "BOX DRAWINGS HEAVY VERTICAL AND LEFT",
      },
      Digraph {
          sequence: ['d', 'h'],
          character: '\u{252c}',
          description: "BOX DRAWINGS LIGHT DOWN AND HORIZONTAL",
      },
      Digraph {
          sequence: ['d', 'H'],
          character: '\u{252f}',
          description: "BOX DRAWINGS DOWN LIGHT AND HORIZONTAL HEAVY",
      },
      Digraph {
          sequence: ['D', 'h'],
          character: '\u{2530}',
          description: "BOX DRAWINGS DOWN HEAVY AND HORIZONTAL LIGHT",
      },
      Digraph {
          sequence: ['D', 'H'],
          character: '\u{2533}',
          description: "BOX DRAWINGS HEAVY DOWN AND HORIZONTAL",
      },
      Digraph {
          sequence: ['u', 'h'],
          character: '\u{2534}',
          description: "BOX DRAWINGS LIGHT UP AND HORIZONTAL",
      },
      Digraph {
          sequence: ['u', 'H'],
          character: '\u{2537}',
          description: "BOX DRAWINGS UP LIGHT AND HORIZONTAL HEAVY",
      },
      Digraph {
          sequence: ['U', 'h'],
          character: '\u{2538}',
          description: "BOX DRAWINGS UP HEAVY AND HORIZONTAL LIGHT",
      },
      Digraph {
          sequence: ['U', 'H'],
          character: '\u{253b}',
          description: "BOX DRAWINGS HEAVY UP AND HORIZONTAL",
      },
      Digraph {
          sequence: ['v', 'h'],
          character: '\u{253c}',
          description: "BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL",
      },
      Digraph {
          sequence: ['v', 'H'],
          character: '\u{253f}',
          description: "BOX DRAWINGS VERTICAL LIGHT AND HORIZONTAL HEAVY",
      },
      Digraph {
          sequence: ['V', 'h'],
          character: '\u{2542}',
          description: "BOX DRAWINGS VERTICAL HEAVY AND HORIZONTAL LIGHT",
      },
      Digraph {
          sequence: ['V', 'H'],
          character: '\u{254b}',
          description: "BOX DRAWINGS HEAVY VERTICAL AND HORIZONTAL",
      },
      Digraph {
          sequence: ['F', 'D'],
          character: '\u{2571}',
          description: "BOX DRAWINGS LIGHT DIAGONAL UPPER RIGHT TO LOWER LEFT",
      },
      Digraph {
          sequence: ['B', 'D'],
          character: '\u{2572}',
          description: "BOX DRAWINGS LIGHT DIAGONAL UPPER LEFT TO LOWER RIGHT",
      },
      Digraph {
          sequence: ['T', 'B'],
          character: '\u{2580}',
          description: "UPPER HALF BLOCK",
      },
      Digraph {
          sequence: ['L', 'B'],
          character: '\u{2584}',
          description: "LOWER HALF BLOCK",
      },
      Digraph {
          sequence: ['F', 'B'],
          character: '\u{2588}',
          description: "FULL BLOCK",
      },
      Digraph {
          sequence: ['l', 'B'],
          character: '\u{258c}',
          description: "LEFT HALF BLOCK",
      },
      Digraph {
          sequence: ['R', 'B'],
          character: '\u{2590}',
          description: "RIGHT HALF BLOCK",
      },
      Digraph {
          sequence: ['.', 'S'],
          character: '\u{2591}',
          description: "LIGHT SHADE",
      },
      Digraph {
          sequence: [':', 'S'],
          character: '\u{2592}',
          description: "MEDIUM SHADE",
      },
      Digraph {
          sequence: ['?', 'S'],
          character: '\u{2593}',
          description: "DARK SHADE",
      },
      Digraph {
          sequence: ['f', 'S'],
          character: '\u{25a0}',
          description: "BLACK SQUARE",
      },
      Digraph {
          sequence: ['O', 'S'],
          character: '\u{25a1}',
          description: "WHITE SQUARE",
      },
      Digraph {
          sequence: ['R', 'O'],
          character: '\u{25a2}',
          description: "WHITE SQUARE WITH ROUNDED CORNERS",
      },
      Digraph {
          sequence: ['R', 'r'],
          character: '\u{25a3}',
          description: "WHITE SQUARE CONTAINING BLACK SMALL SQUARE",
      },
      Digraph {
          sequence: ['R', 'F'],
          character: '\u{25a4}',
          description: "SQUARE WITH HORIZONTAL FILL",
      },
      Digraph {
          sequence: ['R', 'Y'],
          character: '\u{25a5}',
          description: "SQUARE WITH VERTICAL FILL",
      },
      Digraph {
          sequence: ['R', 'H'],
          character: '\u{25a6}',
          description: "SQUARE WITH ORTHOGONAL CROSSHATCH FILL",
      },
      Digraph {
          sequence: ['R', 'Z'],
          character: '\u{25a7}',
          description: "SQUARE WITH UPPER LEFT TO LOWER RIGHT FILL",
      },
      Digraph {
          sequence: ['R', 'K'],
          character: '\u{25a8}',
          description: "SQUARE WITH UPPER RIGHT TO LOWER LEFT FILL",
      },
      Digraph {
          sequence: ['R', 'X'],
          character: '\u{25a9}',
          description: "SQUARE WITH DIAGONAL CROSSHATCH FILL",
      },
      Digraph {
          sequence: ['s', 'B'],
          character: '\u{25aa}',
          description: "BLACK SMALL SQUARE",
      },
      Digraph {
          sequence: ['S', 'R'],
          character: '\u{25ac}',
          description: "BLACK RECTANGLE",
      },
      Digraph {
          sequence: ['O', 'r'],
          character: '\u{25ad}',
          description: "WHITE RECTANGLE",
      },
      Digraph {
          sequence: ['U', 'T'],
          character: '\u{25b2}',
          description: "BLACK UP-POINTING TRIANGLE",
      },
      Digraph {
          sequence: ['u', 'T'],
          character: '\u{25b3}',
          description: "WHITE UP-POINTING TRIANGLE",
      },
      Digraph {
          sequence: ['P', 'R'],
          character: '\u{25b6}',
          description: "BLACK RIGHT-POINTING TRIANGLE",
      },
      Digraph {
          sequence: ['T', 'r'],
          character: '\u{25b7}',
          description: "WHITE RIGHT-POINTING TRIANGLE",
      },
      Digraph {
          sequence: ['D', 't'],
          character: '\u{25bc}',
          description: "BLACK DOWN-POINTING TRIANGLE",
      },
      Digraph {
          sequence: ['d', 'T'],
          character: '\u{25bd}',
          description: "WHITE DOWN-POINTING TRIANGLE",
      },
      Digraph {
          sequence: ['P', 'L'],
          character: '\u{25c0}',
          description: "BLACK LEFT-POINTING TRIANGLE",
      },
      Digraph {
          sequence: ['T', 'l'],
          character: '\u{25c1}',
          description: "WHITE LEFT-POINTING TRIANGLE",
      },
      Digraph {
          sequence: ['D', 'b'],
          character: '\u{25c6}',
          description: "BLACK DIAMOND",
      },
      Digraph {
          sequence: ['D', 'w'],
          character: '\u{25c7}',
          description: "WHITE DIAMOND",
      },
      Digraph {
          sequence: ['L', 'Z'],
          character: '\u{25ca}',
          description: "LOZENGE",
      },
      Digraph {
          sequence: ['0', 'm'],
          character: '\u{25cb}',
          description: "WHITE CIRCLE",
      },
      Digraph {
          sequence: ['0', 'o'],
          character: '\u{25ce}',
          description: "BULLSEYE",
      },
      Digraph {
          sequence: ['0', 'M'],
          character: '\u{25cf}',
          description: "BLACK CIRCLE",
      },
      Digraph {
          sequence: ['0', 'L'],
          character: '\u{25d0}',
          description: "CIRCLE WITH LEFT HALF BLACK",
      },
      Digraph {
          sequence: ['0', 'R'],
          character: '\u{25d1}',
          description: "CIRCLE WITH RIGHT HALF BLACK",
      },
      Digraph {
          sequence: ['S', 'n'],
          character: '\u{25d8}',
          description: "INVERSE BULLET",
      },
      Digraph {
          sequence: ['I', 'c'],
          character: '\u{25d9}',
          description: "INVERSE WHITE CIRCLE",
      },
      Digraph {
          sequence: ['F', 'd'],
          character: '\u{25e2}',
          description: "BLACK LOWER RIGHT TRIANGLE",
      },
      Digraph {
          sequence: ['B', 'd'],
          character: '\u{25e3}',
          description: "BLACK LOWER LEFT TRIANGLE",
      },
      Digraph {
          sequence: ['*', '2'],
          character: '\u{2605}',
          description: "BLACK STAR",
      },
      Digraph {
          sequence: ['*', '1'],
          character: '\u{2606}',
          description: "WHITE STAR",
      },
      Digraph {
          sequence: ['<', 'H'],
          character: '\u{261c}',
          description: "WHITE LEFT POINTING INDEX",
      },
      Digraph {
          sequence: ['>', 'H'],
          character: '\u{261e}',
          description: "WHITE RIGHT POINTING INDEX",
      },
      Digraph {
          sequence: ['0', 'u'],
          character: '\u{263a}',
          description: "WHITE SMILING FACE",
      },
      Digraph {
          sequence: ['0', 'U'],
          character: '\u{263b}',
          description: "BLACK SMILING FACE",
      },
      Digraph {
          sequence: ['S', 'U'],
          character: '\u{263c}',
          description: "WHITE SUN WITH RAYS",
      },
      Digraph {
          sequence: ['F', 'm'],
          character: '\u{2640}',
          description: "FEMALE SIGN",
      },
      Digraph {
          sequence: ['M', 'l'],
          character: '\u{2642}',
          description: "MALE SIGN",
      },
      Digraph {
          sequence: ['c', 'S'],
          character: '\u{2660}',
          description: "BLACK SPADE SUIT",
      },
      Digraph {
          sequence: ['c', 'H'],
          character: '\u{2661}',
          description: "WHITE HEART SUIT",
      },
      Digraph {
          sequence: ['c', 'D'],
          character: '\u{2662}',
          description: "WHITE DIAMOND SUIT",
      },
      Digraph {
          sequence: ['c', 'C'],
          character: '\u{2663}',
          description: "BLACK CLUB SUIT",
      },
      Digraph {
          sequence: ['M', 'd'],
          character: '\u{2669}',
          description: "QUARTER NOTE",
      },
      Digraph {
          sequence: ['M', '8'],
          character: '\u{266a}',
          description: "EIGHTH NOTE",
      },
      Digraph {
          sequence: ['M', '2'],
          character: '\u{266b}',
          description: "BARRED EIGHTH NOTES",
      },
      Digraph {
          sequence: ['M', 'b'],
          character: '\u{266d}',
          description: "MUSIC FLAT SIGN",
      },
      Digraph {
          sequence: ['M', 'x'],
          character: '\u{266e}',
          description: "MUSIC NATURAL SIGN",
      },
      Digraph {
          sequence: ['M', 'X'],
          character: '\u{266f}',
          description: "MUSIC SHARP SIGN",
      },
      Digraph {
          sequence: ['O', 'K'],
          character: '\u{2713}',
          description: "CHECK MARK",
      },
      Digraph {
          sequence: ['X', 'X'],
          character: '\u{2717}',
          description: "BALLOT X",
      },
      Digraph {
          sequence: ['-', 'X'],
          character: '\u{2720}',
          description: "MALTESE CROSS",
      },
      Digraph {
          sequence: ['I', 'S'],
          character: '\u{3000}',
          description: "IDEOGRAPHIC SPACE",
      },
      Digraph {
          sequence: [',', '_'],
          character: '\u{3001}',
          description: "IDEOGRAPHIC COMMA",
      },
      Digraph {
          sequence: ['.', '_'],
          character: '\u{3002}',
          description: "IDEOGRAPHIC PERIOD",
      },
      Digraph {
          sequence: ['+', '"'],
          character: '\u{3003}',
          description: "DITTO MARK",
      },
      Digraph {
          sequence: ['+', '_'],
          character: '\u{3004}',
          description: "IDEOGRAPHIC DITTO MARK",
      },
      Digraph {
          sequence: ['*', '_'],
          character: '\u{3005}',
          description: "IDEOGRAPHIC ITERATION MARK",
      },
      Digraph {
          sequence: [';', '_'],
          character: '\u{3006}',
          description: "IDEOGRAPHIC CLOSING MARK",
      },
      Digraph {
          sequence: ['0', '_'],
          character: '\u{3007}',
          description: "IDEOGRAPHIC NUMBER ZERO",
      },
      Digraph {
          sequence: ['<', '+'],
          character: '\u{300a}',
          description: "LEFT DOUBLE ANGLE BRACKET",
      },
      Digraph {
          sequence: ['>', '+'],
          character: '\u{300b}',
          description: "RIGHT DOUBLE ANGLE BRACKET",
      },
      Digraph {
          sequence: ['<', '\''],
          character: '\u{300c}',
          description: "LEFT CORNER BRACKET",
      },
      Digraph {
          sequence: ['>', '\''],
          character: '\u{300d}',
          description: "RIGHT CORNER BRACKET",
      },
      Digraph {
          sequence: ['<', '"'],
          character: '\u{300e}',
          description: "LEFT WHITE CORNER BRACKET",
      },
      Digraph {
          sequence: ['>', '"'],
          character: '\u{300f}',
          description: "RIGHT WHITE CORNER BRACKET",
      },
      Digraph {
          sequence: ['(', '"'],
          character: '\u{3010}',
          description: "LEFT BLACK LENTICULAR BRACKET",
      },
      Digraph {
          sequence: [')', '"'],
          character: '\u{3011}',
          description: "RIGHT BLACK LENTICULAR BRACKET",
      },
      Digraph {
          sequence: ['=', 'T'],
          character: '\u{3012}',
          description: "POSTAL MARK",
      },
      Digraph {
          sequence: ['=', '_'],
          character: '\u{3013}',
          description: "GETA MARK",
      },
      Digraph {
          sequence: ['(', '\''],
          character: '\u{3014}',
          description: "LEFT TORTOISE SHELL BRACKET",
      },
      Digraph {
          sequence: [')', '\''],
          character: '\u{3015}',
          description: "RIGHT TORTOISE SHELL BRACKET",
      },
      Digraph {
          sequence: ['(', 'I'],
          character: '\u{3016}',
          description: "LEFT WHITE LENTICULAR BRACKET",
      },
      Digraph {
          sequence: [')', 'I'],
          character: '\u{3017}',
          description: "RIGHT WHITE LENTICULAR BRACKET",
      },
      Digraph {
          sequence: ['-', '?'],
          character: '\u{301c}',
          description: "WAVE DASH",
      },
      Digraph {
          sequence: ['A', '5'],
          character: '\u{3041}',
          description: "HIRAGANA LETTER SMALL A",
      },
      Digraph {
          sequence: ['a', '5'],
          character: '\u{3042}',
          description: "HIRAGANA LETTER A",
      },
      Digraph {
          sequence: ['I', '5'],
          character: '\u{3043}',
          description: "HIRAGANA LETTER SMALL I",
      },
      Digraph {
          sequence: ['i', '5'],
          character: '\u{3044}',
          description: "HIRAGANA LETTER I",
      },
      Digraph {
          sequence: ['U', '5'],
          character: '\u{3045}',
          description: "HIRAGANA LETTER SMALL U",
      },
      Digraph {
          sequence: ['u', '5'],
          character: '\u{3046}',
          description: "HIRAGANA LETTER U",
      },
      Digraph {
          sequence: ['E', '5'],
          character: '\u{3047}',
          description: "HIRAGANA LETTER SMALL E",
      },
      Digraph {
          sequence: ['e', '5'],
          character: '\u{3048}',
          description: "HIRAGANA LETTER E",
      },
      Digraph {
          sequence: ['O', '5'],
          character: '\u{3049}',
          description: "HIRAGANA LETTER SMALL O",
      },
      Digraph {
          sequence: ['o', '5'],
          character: '\u{304a}',
          description: "HIRAGANA LETTER O",
      },
      Digraph {
          sequence: ['k', 'a'],
          character: '\u{304b}',
          description: "HIRAGANA LETTER KA",
      },
      Digraph {
          sequence: ['g', 'a'],
          character: '\u{304c}',
          description: "HIRAGANA LETTER GA",
      },
      Digraph {
          sequence: ['k', 'i'],
          character: '\u{304d}',
          description: "HIRAGANA LETTER KI",
      },
      Digraph {
          sequence: ['g', 'i'],
          character: '\u{304e}',
          description: "HIRAGANA LETTER GI",
      },
      Digraph {
          sequence: ['k', 'u'],
          character: '\u{304f}',
          description: "HIRAGANA LETTER KU",
      },
      Digraph {
          sequence: ['g', 'u'],
          character: '\u{3050}',
          description: "HIRAGANA LETTER GU",
      },
      Digraph {
          sequence: ['k', 'e'],
          character: '\u{3051}',
          description: "HIRAGANA LETTER KE",
      },
      Digraph {
          sequence: ['g', 'e'],
          character: '\u{3052}',
          description: "HIRAGANA LETTER GE",
      },
      Digraph {
          sequence: ['k', 'o'],
          character: '\u{3053}',
          description: "HIRAGANA LETTER KO",
      },
      Digraph {
          sequence: ['g', 'o'],
          character: '\u{3054}',
          description: "HIRAGANA LETTER GO",
      },
      Digraph {
          sequence: ['s', 'a'],
          character: '\u{3055}',
          description: "HIRAGANA LETTER SA",
      },
      Digraph {
          sequence: ['z', 'a'],
          character: '\u{3056}',
          description: "HIRAGANA LETTER ZA",
      },
      Digraph {
          sequence: ['s', 'i'],
          character: '\u{3057}',
          description: "HIRAGANA LETTER SI",
      },
      Digraph {
          sequence: ['z', 'i'],
          character: '\u{3058}',
          description: "HIRAGANA LETTER ZI",
      },
      Digraph {
          sequence: ['s', 'u'],
          character: '\u{3059}',
          description: "HIRAGANA LETTER SU",
      },
      Digraph {
          sequence: ['z', 'u'],
          character: '\u{305a}',
          description: "HIRAGANA LETTER ZU",
      },
      Digraph {
          sequence: ['s', 'e'],
          character: '\u{305b}',
          description: "HIRAGANA LETTER SE",
      },
      Digraph {
          sequence: ['z', 'e'],
          character: '\u{305c}',
          description: "HIRAGANA LETTER ZE",
      },
      Digraph {
          sequence: ['s', 'o'],
          character: '\u{305d}',
          description: "HIRAGANA LETTER SO",
      },
      Digraph {
          sequence: ['z', 'o'],
          character: '\u{305e}',
          description: "HIRAGANA LETTER ZO",
      },
      Digraph {
          sequence: ['t', 'a'],
          character: '\u{305f}',
          description: "HIRAGANA LETTER TA",
      },
      Digraph {
          sequence: ['d', 'a'],
          character: '\u{3060}',
          description: "HIRAGANA LETTER DA",
      },
      Digraph {
          sequence: ['t', 'i'],
          character: '\u{3061}',
          description: "HIRAGANA LETTER TI",
      },
      Digraph {
          sequence: ['d', 'i'],
          character: '\u{3062}',
          description: "HIRAGANA LETTER DI",
      },
      Digraph {
          sequence: ['t', 'U'],
          character: '\u{3063}',
          description: "HIRAGANA LETTER SMALL TU",
      },
      Digraph {
          sequence: ['t', 'u'],
          character: '\u{3064}',
          description: "HIRAGANA LETTER TU",
      },
      Digraph {
          sequence: ['d', 'u'],
          character: '\u{3065}',
          description: "HIRAGANA LETTER DU",
      },
      Digraph {
          sequence: ['t', 'e'],
          character: '\u{3066}',
          description: "HIRAGANA LETTER TE",
      },
      Digraph {
          sequence: ['d', 'e'],
          character: '\u{3067}',
          description: "HIRAGANA LETTER DE",
      },
      Digraph {
          sequence: ['t', 'o'],
          character: '\u{3068}',
          description: "HIRAGANA LETTER TO",
      },
      Digraph {
          sequence: ['d', 'o'],
          character: '\u{3069}',
          description: "HIRAGANA LETTER DO",
      },
      Digraph {
          sequence: ['n', 'a'],
          character: '\u{306a}',
          description: "HIRAGANA LETTER NA",
      },
      Digraph {
          sequence: ['n', 'i'],
          character: '\u{306b}',
          description: "HIRAGANA LETTER NI",
      },
      Digraph {
          sequence: ['n', 'u'],
          character: '\u{306c}',
          description: "HIRAGANA LETTER NU",
      },
      Digraph {
          sequence: ['n', 'e'],
          character: '\u{306d}',
          description: "HIRAGANA LETTER NE",
      },
      Digraph {
          sequence: ['n', 'o'],
          character: '\u{306e}',
          description: "HIRAGANA LETTER NO",
      },
      Digraph {
          sequence: ['h', 'a'],
          character: '\u{306f}',
          description: "HIRAGANA LETTER HA",
      },
      Digraph {
          sequence: ['b', 'a'],
          character: '\u{3070}',
          description: "HIRAGANA LETTER BA",
      },
      Digraph {
          sequence: ['p', 'a'],
          character: '\u{3071}',
          description: "HIRAGANA LETTER PA",
      },
      Digraph {
          sequence: ['h', 'i'],
          character: '\u{3072}',
          description: "HIRAGANA LETTER HI",
      },
      Digraph {
          sequence: ['b', 'i'],
          character: '\u{3073}',
          description: "HIRAGANA LETTER BI",
      },
      Digraph {
          sequence: ['p', 'i'],
          character: '\u{3074}',
          description: "HIRAGANA LETTER PI",
      },
      Digraph {
          sequence: ['h', 'u'],
          character: '\u{3075}',
          description: "HIRAGANA LETTER HU",
      },
      Digraph {
          sequence: ['b', 'u'],
          character: '\u{3076}',
          description: "HIRAGANA LETTER BU",
      },
      Digraph {
          sequence: ['p', 'u'],
          character: '\u{3077}',
          description: "HIRAGANA LETTER PU",
      },
      Digraph {
          sequence: ['h', 'e'],
          character: '\u{3078}',
          description: "HIRAGANA LETTER HE",
      },
      Digraph {
          sequence: ['b', 'e'],
          character: '\u{3079}',
          description: "HIRAGANA LETTER BE",
      },
      Digraph {
          sequence: ['p', 'e'],
          character: '\u{307a}',
          description: "HIRAGANA LETTER PE",
      },
      Digraph {
          sequence: ['h', 'o'],
          character: '\u{307b}',
          description: "HIRAGANA LETTER HO",
      },
      Digraph {
          sequence: ['b', 'o'],
          character: '\u{307c}',
          description: "HIRAGANA LETTER BO",
      },
      Digraph {
          sequence: ['p', 'o'],
          character: '\u{307d}',
          description: "HIRAGANA LETTER PO",
      },
      Digraph {
          sequence: ['m', 'a'],
          character: '\u{307e}',
          description: "HIRAGANA LETTER MA",
      },
      Digraph {
          sequence: ['m', 'i'],
          character: '\u{307f}',
          description: "HIRAGANA LETTER MI",
      },
      Digraph {
          sequence: ['m', 'u'],
          character: '\u{3080}',
          description: "HIRAGANA LETTER MU",
      },
      Digraph {
          sequence: ['m', 'e'],
          character: '\u{3081}',
          description: "HIRAGANA LETTER ME",
      },
      Digraph {
          sequence: ['m', 'o'],
          character: '\u{3082}',
          description: "HIRAGANA LETTER MO",
      },
      Digraph {
          sequence: ['y', 'A'],
          character: '\u{3083}',
          description: "HIRAGANA LETTER SMALL YA",
      },
      Digraph {
          sequence: ['y', 'a'],
          character: '\u{3084}',
          description: "HIRAGANA LETTER YA",
      },
      Digraph {
          sequence: ['y', 'U'],
          character: '\u{3085}',
          description: "HIRAGANA LETTER SMALL YU",
      },
      Digraph {
          sequence: ['y', 'u'],
          character: '\u{3086}',
          description: "HIRAGANA LETTER YU",
      },
      Digraph {
          sequence: ['y', 'O'],
          character: '\u{3087}',
          description: "HIRAGANA LETTER SMALL YO",
      },
      Digraph {
          sequence: ['y', 'o'],
          character: '\u{3088}',
          description: "HIRAGANA LETTER YO",
      },
      Digraph {
          sequence: ['r', 'a'],
          character: '\u{3089}',
          description: "HIRAGANA LETTER RA",
      },
      Digraph {
          sequence: ['r', 'i'],
          character: '\u{308a}',
          description: "HIRAGANA LETTER RI",
      },
      Digraph {
          sequence: ['r', 'u'],
          character: '\u{308b}',
          description: "HIRAGANA LETTER RU",
      },
      Digraph {
          sequence: ['r', 'e'],
          character: '\u{308c}',
          description: "HIRAGANA LETTER RE",
      },
      Digraph {
          sequence: ['r', 'o'],
          character: '\u{308d}',
          description: "HIRAGANA LETTER RO",
      },
      Digraph {
          sequence: ['w', 'A'],
          character: '\u{308e}',
          description: "HIRAGANA LETTER SMALL WA",
      },
      Digraph {
          sequence: ['w', 'a'],
          character: '\u{308f}',
          description: "HIRAGANA LETTER WA",
      },
      Digraph {
          sequence: ['w', 'i'],
          character: '\u{3090}',
          description: "HIRAGANA LETTER WI",
      },
      Digraph {
          sequence: ['w', 'e'],
          character: '\u{3091}',
          description: "HIRAGANA LETTER WE",
      },
      Digraph {
          sequence: ['w', 'o'],
          character: '\u{3092}',
          description: "HIRAGANA LETTER WO",
      },
      Digraph {
          sequence: ['n', '5'],
          character: '\u{3093}',
          description: "HIRAGANA LETTER N",
      },
      Digraph {
          sequence: ['v', 'u'],
          character: '\u{3094}',
          description: "HIRAGANA LETTER VU",
      },
      Digraph {
          sequence: ['"', '5'],
          character: '\u{309b}',
          description: "KATAKANA-HIRAGANA VOICED SOUND MARK",
      },
      Digraph {
          sequence: ['0', '5'],
          character: '\u{309c}',
          description: "KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK",
      },
      Digraph {
          sequence: ['*', '5'],
          character: '\u{309d}',
          description: "HIRAGANA ITERATION MARK",
      },
      Digraph {
          sequence: ['+', '5'],
          character: '\u{309e}',
          description: "HIRAGANA VOICED ITERATION MARK",
      },
      Digraph {
          sequence: ['a', '6'],
          character: '\u{30a1}',
          description: "KATAKANA LETTER SMALL A",
      },
      Digraph {
          sequence: ['A', '6'],
          character: '\u{30a2}',
          description: "KATAKANA LETTER A",
      },
      Digraph {
          sequence: ['i', '6'],
          character: '\u{30a3}',
          description: "KATAKANA LETTER SMALL I",
      },
      Digraph {
          sequence: ['I', '6'],
          character: '\u{30a4}',
          description: "KATAKANA LETTER I",
      },
      Digraph {
          sequence: ['u', '6'],
          character: '\u{30a5}',
          description: "KATAKANA LETTER SMALL U",
      },
      Digraph {
          sequence: ['U', '6'],
          character: '\u{30a6}',
          description: "KATAKANA LETTER U",
      },
      Digraph {
          sequence: ['e', '6'],
          character: '\u{30a7}',
          description: "KATAKANA LETTER SMALL E",
      },
      Digraph {
          sequence: ['E', '6'],
          character: '\u{30a8}',
          description: "KATAKANA LETTER E",
      },
      Digraph {
          sequence: ['o', '6'],
          character: '\u{30a9}',
          description: "KATAKANA LETTER SMALL O",
      },
      Digraph {
          sequence: ['O', '6'],
          character: '\u{30aa}',
          description: "KATAKANA LETTER O",
      },
      Digraph {
          sequence: ['K', 'a'],
          character: '\u{30ab}',
          description: "KATAKANA LETTER KA",
      },
      Digraph {
          sequence: ['G', 'a'],
          character: '\u{30ac}',
          description: "KATAKANA LETTER GA",
      },
      Digraph {
          sequence: ['K', 'i'],
          character: '\u{30ad}',
          description: "KATAKANA LETTER KI",
      },
      Digraph {
          sequence: ['G', 'i'],
          character: '\u{30ae}',
          description: "KATAKANA LETTER GI",
      },
      Digraph {
          sequence: ['K', 'u'],
          character: '\u{30af}',
          description: "KATAKANA LETTER KU",
      },
      Digraph {
          sequence: ['G', 'u'],
          character: '\u{30b0}',
          description: "KATAKANA LETTER GU",
      },
      Digraph {
          sequence: ['K', 'e'],
          character: '\u{30b1}',
          description: "KATAKANA LETTER KE",
      },
      Digraph {
          sequence: ['G', 'e'],
          character: '\u{30b2}',
          description: "KATAKANA LETTER GE",
      },
      Digraph {
          sequence: ['K', 'o'],
          character: '\u{30b3}',
          description: "KATAKANA LETTER KO",
      },
      Digraph {
          sequence: ['G', 'o'],
          character: '\u{30b4}',
          description: "KATAKANA LETTER GO",
      },
      Digraph {
          sequence: ['S', 'a'],
          character: '\u{30b5}',
          description: "KATAKANA LETTER SA",
      },
      Digraph {
          sequence: ['Z', 'a'],
          character: '\u{30b6}',
          description: "KATAKANA LETTER ZA",
      },
      Digraph {
          sequence: ['S', 'i'],
          character: '\u{30b7}',
          description: "KATAKANA LETTER SI",
      },
      Digraph {
          sequence: ['Z', 'i'],
          character: '\u{30b8}',
          description: "KATAKANA LETTER ZI",
      },
      Digraph {
          sequence: ['S', 'u'],
          character: '\u{30b9}',
          description: "KATAKANA LETTER SU",
      },
      Digraph {
          sequence: ['Z', 'u'],
          character: '\u{30ba}',
          description: "KATAKANA LETTER ZU",
      },
      Digraph {
          sequence: ['S', 'e'],
          character: '\u{30bb}',
          description: "KATAKANA LETTER SE",
      },
      Digraph {
          sequence: ['Z', 'e'],
          character: '\u{30bc}',
          description: "KATAKANA LETTER ZE",
      },
      Digraph {
          sequence: ['S', 'o'],
          character: '\u{30bd}',
          description: "KATAKANA LETTER SO",
      },
      Digraph {
          sequence: ['Z', 'o'],
          character: '\u{30be}',
          description: "KATAKANA LETTER ZO",
      },
      Digraph {
          sequence: ['T', 'a'],
          character: '\u{30bf}',
          description: "KATAKANA LETTER TA",
      },
      Digraph {
          sequence: ['D', 'a'],
          character: '\u{30c0}',
          description: "KATAKANA LETTER DA",
      },
      Digraph {
          sequence: ['T', 'i'],
          character: '\u{30c1}',
          description: "KATAKANA LETTER TI",
      },
      Digraph {
          sequence: ['D', 'i'],
          character: '\u{30c2}',
          description: "KATAKANA LETTER DI",
      },
      Digraph {
          sequence: ['T', 'U'],
          character: '\u{30c3}',
          description: "KATAKANA LETTER SMALL TU",
      },
      Digraph {
          sequence: ['T', 'u'],
          character: '\u{30c4}',
          description: "KATAKANA LETTER TU",
      },
      Digraph {
          sequence: ['D', 'u'],
          character: '\u{30c5}',
          description: "KATAKANA LETTER DU",
      },
      Digraph {
          sequence: ['T', 'e'],
          character: '\u{30c6}',
          description: "KATAKANA LETTER TE",
      },
      Digraph {
          sequence: ['D', 'e'],
          character: '\u{30c7}',
          description: "KATAKANA LETTER DE",
      },
      Digraph {
          sequence: ['T', 'o'],
          character: '\u{30c8}',
          description: "KATAKANA LETTER TO",
      },
      Digraph {
          sequence: ['D', 'o'],
          character: '\u{30c9}',
          description: "KATAKANA LETTER DO",
      },
      Digraph {
          sequence: ['N', 'a'],
          character: '\u{30ca}',
          description: "KATAKANA LETTER NA",
      },
      Digraph {
          sequence: ['N', 'i'],
          character: '\u{30cb}',
          description: "KATAKANA LETTER NI",
      },
      Digraph {
          sequence: ['N', 'u'],
          character: '\u{30cc}',
          description: "KATAKANA LETTER NU",
      },
      Digraph {
          sequence: ['N', 'e'],
          character: '\u{30cd}',
          description: "KATAKANA LETTER NE",
      },
      Digraph {
          sequence: ['N', 'o'],
          character: '\u{30ce}',
          description: "KATAKANA LETTER NO",
      },
      Digraph {
          sequence: ['H', 'a'],
          character: '\u{30cf}',
          description: "KATAKANA LETTER HA",
      },
      Digraph {
          sequence: ['B', 'a'],
          character: '\u{30d0}',
          description: "KATAKANA LETTER BA",
      },
      Digraph {
          sequence: ['P', 'a'],
          character: '\u{30d1}',
          description: "KATAKANA LETTER PA",
      },
      Digraph {
          sequence: ['H', 'i'],
          character: '\u{30d2}',
          description: "KATAKANA LETTER HI",
      },
      Digraph {
          sequence: ['B', 'i'],
          character: '\u{30d3}',
          description: "KATAKANA LETTER BI",
      },
      Digraph {
          sequence: ['P', 'i'],
          character: '\u{30d4}',
          description: "KATAKANA LETTER PI",
      },
      Digraph {
          sequence: ['H', 'u'],
          character: '\u{30d5}',
          description: "KATAKANA LETTER HU",
      },
      Digraph {
          sequence: ['B', 'u'],
          character: '\u{30d6}',
          description: "KATAKANA LETTER BU",
      },
      Digraph {
          sequence: ['P', 'u'],
          character: '\u{30d7}',
          description: "KATAKANA LETTER PU",
      },
      Digraph {
          sequence: ['H', 'e'],
          character: '\u{30d8}',
          description: "KATAKANA LETTER HE",
      },
      Digraph {
          sequence: ['B', 'e'],
          character: '\u{30d9}',
          description: "KATAKANA LETTER BE",
      },
      Digraph {
          sequence: ['P', 'e'],
          character: '\u{30da}',
          description: "KATAKANA LETTER PE",
      },
      Digraph {
          sequence: ['H', 'o'],
          character: '\u{30db}',
          description: "KATAKANA LETTER HO",
      },
      Digraph {
          sequence: ['B', 'o'],
          character: '\u{30dc}',
          description: "KATAKANA LETTER BO",
      },
      Digraph {
          sequence: ['P', 'o'],
          character: '\u{30dd}',
          description: "KATAKANA LETTER PO",
      },
      Digraph {
          sequence: ['M', 'a'],
          character: '\u{30de}',
          description: "KATAKANA LETTER MA",
      },
      Digraph {
          sequence: ['M', 'i'],
          character: '\u{30df}',
          description: "KATAKANA LETTER MI",
      },
      Digraph {
          sequence: ['M', 'u'],
          character: '\u{30e0}',
          description: "KATAKANA LETTER MU",
      },
      Digraph {
          sequence: ['M', 'e'],
          character: '\u{30e1}',
          description: "KATAKANA LETTER ME",
      },
      Digraph {
          sequence: ['M', 'o'],
          character: '\u{30e2}',
          description: "KATAKANA LETTER MO",
      },
      Digraph {
          sequence: ['Y', 'A'],
          character: '\u{30e3}',
          description: "KATAKANA LETTER SMALL YA",
      },
      Digraph {
          sequence: ['Y', 'a'],
          character: '\u{30e4}',
          description: "KATAKANA LETTER YA",
      },
      Digraph {
          sequence: ['Y', 'U'],
          character: '\u{30e5}',
          description: "KATAKANA LETTER SMALL YU",
      },
      Digraph {
          sequence: ['Y', 'u'],
          character: '\u{30e6}',
          description: "KATAKANA LETTER YU",
      },
      Digraph {
          sequence: ['Y', 'O'],
          character: '\u{30e7}',
          description: "KATAKANA LETTER SMALL YO",
      },
      Digraph {
          sequence: ['Y', 'o'],
          character: '\u{30e8}',
          description: "KATAKANA LETTER YO",
      },
      Digraph {
          sequence: ['R', 'a'],
          character: '\u{30e9}',
          description: "KATAKANA LETTER RA",
      },
      Digraph {
          sequence: ['R', 'i'],
          character: '\u{30ea}',
          description: "KATAKANA LETTER RI",
      },
      Digraph {
          sequence: ['R', 'u'],
          character: '\u{30eb}',
          description: "KATAKANA LETTER RU",
      },
      Digraph {
          sequence: ['R', 'e'],
          character: '\u{30ec}',
          description: "KATAKANA LETTER RE",
      },
      Digraph {
          sequence: ['R', 'o'],
          character: '\u{30ed}',
          description: "KATAKANA LETTER RO",
      },
      Digraph {
          sequence: ['W', 'A'],
          character: '\u{30ee}',
          description: "KATAKANA LETTER SMALL WA",
      },
      Digraph {
          sequence: ['W', 'a'],
          character: '\u{30ef}',
          description: "KATAKANA LETTER WA",
      },
      Digraph {
          sequence: ['W', 'i'],
          character: '\u{30f0}',
          description: "KATAKANA LETTER WI",
      },
      Digraph {
          sequence: ['W', 'e'],
          character: '\u{30f1}',
          description: "KATAKANA LETTER WE",
      },
      Digraph {
          sequence: ['W', 'o'],
          character: '\u{30f2}',
          description: "KATAKANA LETTER WO",
      },
      Digraph {
          sequence: ['N', '6'],
          character: '\u{30f3}',
          description: "KATAKANA LETTER N",
      },
      Digraph {
          sequence: ['V', 'u'],
          character: '\u{30f4}',
          description: "KATAKANA LETTER VU",
      },
      Digraph {
          sequence: ['K', 'A'],
          character: '\u{30f5}',
          description: "KATAKANA LETTER SMALL KA",
      },
      Digraph {
          sequence: ['K', 'E'],
          character: '\u{30f6}',
          description: "KATAKANA LETTER SMALL KE",
      },
      Digraph {
          sequence: ['V', 'a'],
          character: '\u{30f7}',
          description: "KATAKANA LETTER VA",
      },
      Digraph {
          sequence: ['V', 'i'],
          character: '\u{30f8}',
          description: "KATAKANA LETTER VI",
      },
      Digraph {
          sequence: ['V', 'e'],
          character: '\u{30f9}',
          description: "KATAKANA LETTER VE",
      },
      Digraph {
          sequence: ['V', 'o'],
          character: '\u{30fa}',
          description: "KATAKANA LETTER VO",
      },
      Digraph {
          sequence: ['.', '6'],
          character: '\u{30fb}',
          description: "KATAKANA MIDDLE DOT",
      },
      Digraph {
          sequence: ['-', '6'],
          character: '\u{30fc}',
          description: "KATAKANA-HIRAGANA PROLONGED SOUND MARK",
      },
      Digraph {
          sequence: ['*', '6'],
          character: '\u{30fd}',
          description: "KATAKANA ITERATION MARK",
      },
      Digraph {
          sequence: ['+', '6'],
          character: '\u{30fe}',
          description: "KATAKANA VOICED ITERATION MARK",
      },
      Digraph {
          sequence: ['b', '4'],
          character: '\u{3105}',
          description: "BOPOMOFO LETTER B",
      },
      Digraph {
          sequence: ['p', '4'],
          character: '\u{3106}',
          description: "BOPOMOFO LETTER P",
      },
      Digraph {
          sequence: ['m', '4'],
          character: '\u{3107}',
          description: "BOPOMOFO LETTER M",
      },
      Digraph {
          sequence: ['f', '4'],
          character: '\u{3108}',
          description: "BOPOMOFO LETTER F",
      },
      Digraph {
          sequence: ['d', '4'],
          character: '\u{3109}',
          description: "BOPOMOFO LETTER D",
      },
      Digraph {
          sequence: ['t', '4'],
          character: '\u{310a}',
          description: "BOPOMOFO LETTER T",
      },
      Digraph {
          sequence: ['n', '4'],
          character: '\u{310b}',
          description: "BOPOMOFO LETTER N",
      },
      Digraph {
          sequence: ['l', '4'],
          character: '\u{310c}',
          description: "BOPOMOFO LETTER L",
      },
      Digraph {
          sequence: ['g', '4'],
          character: '\u{310d}',
          description: "BOPOMOFO LETTER G",
      },
      Digraph {
          sequence: ['k', '4'],
          character: '\u{310e}',
          description: "BOPOMOFO LETTER K",
      },
      Digraph {
          sequence: ['h', '4'],
          character: '\u{310f}',
          description: "BOPOMOFO LETTER H",
      },
      Digraph {
          sequence: ['j', '4'],
          character: '\u{3110}',
          description: "BOPOMOFO LETTER J",
      },
      Digraph {
          sequence: ['q', '4'],
          character: '\u{3111}',
          description: "BOPOMOFO LETTER Q",
      },
      Digraph {
          sequence: ['x', '4'],
          character: '\u{3112}',
          description: "BOPOMOFO LETTER X",
      },
      Digraph {
          sequence: ['z', 'h'],
          character: '\u{3113}',
          description: "BOPOMOFO LETTER ZH",
      },
      Digraph {
          sequence: ['c', 'h'],
          character: '\u{3114}',
          description: "BOPOMOFO LETTER CH",
      },
      Digraph {
          sequence: ['s', 'h'],
          character: '\u{3115}',
          description: "BOPOMOFO LETTER SH",
      },
      Digraph {
          sequence: ['r', '4'],
          character: '\u{3116}',
          description: "BOPOMOFO LETTER R",
      },
      Digraph {
          sequence: ['z', '4'],
          character: '\u{3117}',
          description: "BOPOMOFO LETTER Z",
      },
      Digraph {
          sequence: ['c', '4'],
          character: '\u{3118}',
          description: "BOPOMOFO LETTER C",
      },
      Digraph {
          sequence: ['s', '4'],
          character: '\u{3119}',
          description: "BOPOMOFO LETTER S",
      },
      Digraph {
          sequence: ['a', '4'],
          character: '\u{311a}',
          description: "BOPOMOFO LETTER A",
      },
      Digraph {
          sequence: ['o', '4'],
          character: '\u{311b}',
          description: "BOPOMOFO LETTER O",
      },
      Digraph {
          sequence: ['e', '4'],
          character: '\u{311c}',
          description: "BOPOMOFO LETTER E",
      },
      Digraph {
          sequence: ['a', 'i'],
          character: '\u{311e}',
          description: "BOPOMOFO LETTER AI",
      },
      Digraph {
          sequence: ['e', 'i'],
          character: '\u{311f}',
          description: "BOPOMOFO LETTER EI",
      },
      Digraph {
          sequence: ['a', 'u'],
          character: '\u{3120}',
          description: "BOPOMOFO LETTER AU",
      },
      Digraph {
          sequence: ['o', 'u'],
          character: '\u{3121}',
          description: "BOPOMOFO LETTER OU",
      },
      Digraph {
          sequence: ['a', 'n'],
          character: '\u{3122}',
          description: "BOPOMOFO LETTER AN",
      },
      Digraph {
          sequence: ['e', 'n'],
          character: '\u{3123}',
          description: "BOPOMOFO LETTER EN",
      },
      Digraph {
          sequence: ['a', 'N'],
          character: '\u{3124}',
          description: "BOPOMOFO LETTER ANG",
      },
      Digraph {
          sequence: ['e', 'N'],
          character: '\u{3125}',
          description: "BOPOMOFO LETTER ENG",
      },
      Digraph {
          sequence: ['e', 'r'],
          character: '\u{3126}',
          description: "BOPOMOFO LETTER ER",
      },
      Digraph {
          sequence: ['i', '4'],
          character: '\u{3127}',
          description: "BOPOMOFO LETTER I",
      },
      Digraph {
          sequence: ['u', '4'],
          character: '\u{3128}',
          description: "BOPOMOFO LETTER U",
      },
      Digraph {
          sequence: ['i', 'u'],
          character: '\u{3129}',
          description: "BOPOMOFO LETTER IU",
      },
      Digraph {
          sequence: ['v', '4'],
          character: '\u{312a}',
          description: "BOPOMOFO LETTER V",
      },
      Digraph {
          sequence: ['n', 'G'],
          character: '\u{312b}',
          description: "BOPOMOFO LETTER NG",
      },
      Digraph {
          sequence: ['g', 'n'],
          character: '\u{312c}',
          description: "BOPOMOFO LETTER GN",
      },
      Digraph {
          sequence: ['1', 'c'],
          character: '\u{3220}',
          description: "PARENTHESIZED IDEOGRAPH ONE",
      },
      Digraph {
          sequence: ['2', 'c'],
          character: '\u{3221}',
          description: "PARENTHESIZED IDEOGRAPH TWO",
      },
      Digraph {
          sequence: ['3', 'c'],
          character: '\u{3222}',
          description: "PARENTHESIZED IDEOGRAPH THREE",
      },
      Digraph {
          sequence: ['4', 'c'],
          character: '\u{3223}',
          description: "PARENTHESIZED IDEOGRAPH FOUR",
      },
      Digraph {
          sequence: ['5', 'c'],
          character: '\u{3224}',
          description: "PARENTHESIZED IDEOGRAPH FIVE",
      },
      Digraph {
          sequence: ['6', 'c'],
          character: '\u{3225}',
          description: "PARENTHESIZED IDEOGRAPH SIX",
      },
      Digraph {
          sequence: ['7', 'c'],
          character: '\u{3226}',
          description: "PARENTHESIZED IDEOGRAPH SEVEN",
      },
      Digraph {
          sequence: ['8', 'c'],
          character: '\u{3227}',
          description: "PARENTHESIZED IDEOGRAPH EIGHT",
      },
      Digraph {
          sequence: ['9', 'c'],
          character: '\u{3228}',
          description: "PARENTHESIZED IDEOGRAPH NINE",
      },
      Digraph {
          sequence: ['f', 'f'],
          character: '\u{fb00}',
          description: "LATIN SMALL LIGATURE FF",
      },
      Digraph {
          sequence: ['f', 'i'],
          character: '\u{fb01}',
          description: "LATIN SMALL LIGATURE FI",
      },
      Digraph {
          sequence: ['f', 'l'],
          character: '\u{fb02}',
          description: "LATIN SMALL LIGATURE FL",
      },
      Digraph {
          sequence: ['f', 't'],
          character: '\u{fb05}',
          description: "LATIN SMALL LIGATURE FT",
      },
      Digraph {
          sequence: ['s', 't'],
          character: '\u{fb06}',
          description: "LATIN SMALL LIGATURE ST",
      },
      Digraph {
          sequence: ['N', 'U'],
          character: '\u{0000}',
          description: "NULL (NUL)",
      },
      Digraph {
          sequence: ['S', 'H'],
          character: '\u{0001}',
          description: "START OF HEADING (SOH)",
      },
      Digraph {
          sequence: ['S', 'X'],
          character: '\u{0002}',
          description: "START OF TEXT (STX)",
      },
      Digraph {
          sequence: ['E', 'X'],
          character: '\u{0003}',
          description: "END OF TEXT (ETX)",
      },
      Digraph {
          sequence: ['E', 'T'],
          character: '\u{0004}',
          description: "END OF TRANSMISSION (EOT)",
      },
      Digraph {
          sequence: ['E', 'Q'],
          character: '\u{0005}',
          description: "ENQUIRY (ENQ)",
      },
      Digraph {
          sequence: ['A', 'K'],
          character: '\u{0006}',
          description: "ACKNOWLEDGE (ACK)",
      },
      Digraph {
          sequence: ['B', 'L'],
          character: '\u{0007}',
          description: "BELL (BEL)",
      },
      Digraph {
          sequence: ['B', 'S'],
          character: '\u{0008}',
          description: "BACKSPACE (BS)",
      },
      Digraph {
          sequence: ['H', 'T'],
          character: '\u{0009}',
          description: "CHARACTER TABULATION (HT)",
      },
      Digraph {
          sequence: ['L', 'F'],
          character: '\u{000a}',
          description: "LINE FEED (LF)",
      },
      Digraph {
          sequence: ['V', 'T'],
          character: '\u{000b}',
          description: "LINE TABULATION (VT)",
      },
      Digraph {
          sequence: ['F', 'F'],
          character: '\u{000c}',
          description: "FORM FEED (FF)",
      },
      Digraph {
          sequence: ['C', 'R'],
          character: '\u{000d}',
          description: "CARRIAGE RETURN (CR)",
      },
      Digraph {
          sequence: ['S', 'O'],
          character: '\u{000e}',
          description: "SHIFT OUT (SO)",
      },
      Digraph {
          sequence: ['S', 'I'],
          character: '\u{000f}',
          description: "SHIFT IN (SI)",
      },
      Digraph {
          sequence: ['D', 'L'],
          character: '\u{0010}',
          description: "DATALINK ESCAPE (DLE)",
      },
      Digraph {
          sequence: ['D', '1'],
          character: '\u{0011}',
          description: "DEVICE CONTROL ONE (DC1)",
      },
      Digraph {
          sequence: ['D', '2'],
          character: '\u{0012}',
          description: "DEVICE CONTROL TWO (DC2)",
      },
      Digraph {
          sequence: ['D', '3'],
          character: '\u{0013}',
          description: "DEVICE CONTROL THREE (DC3)",
      },
      Digraph {
          sequence: ['D', '4'],
          character: '\u{0014}',
          description: "DEVICE CONTROL FOUR (DC4)",
      },
      Digraph {
          sequence: ['N', 'K'],
          character: '\u{0015}',
          description: "NEGATIVE ACKNOWLEDGE (NAK)",
      },
      Digraph {
          sequence: ['S', 'Y'],
          character: '\u{0016}',
          description: "SYNCRONOUS IDLE (SYN)",
      },
      Digraph {
          sequence: ['E', 'B'],
          character: '\u{0017}',
          description: "END OF TRANSMISSION BLOCK (ETB)",
      },
      Digraph {
          sequence: ['C', 'N'],
          character: '\u{0018}',
          description: "CANCEL (CAN)",
      },
      Digraph {
          sequence: ['E', 'M'],
          character: '\u{0019}',
          description: "END OF MEDIUM (EM)",
      },
      Digraph {
          sequence: ['S', 'B'],
          character: '\u{001a}',
          description: "SUBSTITUTE (SUB)",
      },
      Digraph {
          sequence: ['E', 'C'],
          character: '\u{001b}',
          description: "ESCAPE (ESC)",
      },
      Digraph {
          sequence: ['F', 'S'],
          character: '\u{001c}',
          description: "FILE SEPARATOR (IS4)",
      },
      Digraph {
          sequence: ['G', 'S'],
          character: '\u{001d}',
          description: "GROUP SEPARATOR (IS3)",
      },
      Digraph {
          sequence: ['R', 'S'],
          character: '\u{001e}',
          description: "RECORD SEPARATOR (IS2)",
      },
      Digraph {
          sequence: ['U', 'S'],
          character: '\u{001f}',
          description: "UNIT SEPARATOR (IS1)",
      },
      Digraph {
          sequence: ['D', 'T'],
          character: '\u{007f}',
          description: "DELETE (DEL)",
      },
      Digraph {
          sequence: ['P', 'A'],
          character: '\u{0080}',
          description: "PADDING CHARACTER (PAD)",
      },
      Digraph {
          sequence: ['H', 'O'],
          character: '\u{0081}',
          description: "HIGH OCTET PRESET (HOP)",
      },
      Digraph {
          sequence: ['B', 'H'],
          character: '\u{0082}',
          description: "BREAK PERMITTED HERE (BPH)",
      },
      Digraph {
          sequence: ['N', 'H'],
          character: '\u{0083}',
          description: "NO BREAK HERE (NBH)",
      },
      Digraph {
          sequence: ['I', 'N'],
          character: '\u{0084}',
          description: "INDEX (IND)",
      },
      Digraph {
          sequence: ['N', 'L'],
          character: '\u{0085}',
          description: "NEXT LINE (NEL)",
      },
      Digraph {
          sequence: ['S', 'A'],
          character: '\u{0086}',
          description: "START OF SELECTED AREA (SSA)",
      },
      Digraph {
          sequence: ['E', 'S'],
          character: '\u{0087}',
          description: "END OF SELECTED AREA (ESA)",
      },
      Digraph {
          sequence: ['H', 'S'],
          character: '\u{0088}',
          description: "CHARACTER TABULATION SET (HTS)",
      },
      Digraph {
          sequence: ['H', 'J'],
          character: '\u{0089}',
          description: "CHARACTER TABULATION WITH JUSTIFICATION (HTJ)",
      },
      Digraph {
          sequence: ['V', 'S'],
          character: '\u{008a}',
          description: "LINE TABULATION SET (VTS)",
      },
      Digraph {
          sequence: ['P', 'D'],
          character: '\u{008b}',
          description: "PARTIAL LINE FORWARD (PLD)",
      },
      Digraph {
          sequence: ['P', 'U'],
          character: '\u{008c}',
          description: "PARTIAL LINE BACKWARD (PLU)",
      },
      Digraph {
          sequence: ['R', 'I'],
          character: '\u{008d}',
          description: "REVERSE LINE FEED (RI)",
      },
      Digraph {
          sequence: ['S', '2'],
          character: '\u{008e}',
          description: "SINGLE-SHIFT TWO (SS2)",
      },
      Digraph {
          sequence: ['S', '3'],
          character: '\u{008f}',
          description: "SINGLE-SHIFT THREE (SS3)",
      },
      Digraph {
          sequence: ['D', 'C'],
          character: '\u{0090}',
          description: "DEVICE CONTROL STRING (DCS)",
      },
      Digraph {
          sequence: ['P', '1'],
          character: '\u{0091}',
          description: "PRIVATE USE ONE (PU1)",
      },
      Digraph {
          sequence: ['P', '2'],
          character: '\u{0092}',
          description: "PRIVATE USE TWO (PU2)",
      },
      Digraph {
          sequence: ['T', 'S'],
          character: '\u{0093}',
          description: "SET TRANSMIT STATE (STS)",
      },
      Digraph {
          sequence: ['C', 'C'],
          character: '\u{0094}',
          description: "CANCEL CHARACTER (CCH)",
      },
      Digraph {
          sequence: ['M', 'W'],
          character: '\u{0095}',
          description: "MESSAGE WAITING (MW)",
      },
      Digraph {
          sequence: ['S', 'G'],
          character: '\u{0096}',
          description: "START OF GUARDED AREA (SPA)",
      },
      Digraph {
          sequence: ['E', 'G'],
          character: '\u{0097}',
          description: "END OF GUARDED AREA (EPA)",
      },
      Digraph {
          sequence: ['S', 'S'],
          character: '\u{0098}',
          description: "START OF STRING (SOS)",
      },
      Digraph {
          sequence: ['G', 'C'],
          character: '\u{0099}',
          description: "SINGLE GRAPHIC CHARACTER INTRODUCER (SGCI)",
      },
      Digraph {
          sequence: ['S', 'C'],
          character: '\u{009a}',
          description: "SINGLE CHARACTER INTRODUCER (SCI)",
      },
      Digraph {
          sequence: ['C', 'I'],
          character: '\u{009b}',
          description: "CONTROL SEQUENCE INTRODUCER (CSI)",
      },
      Digraph {
          sequence: ['S', 'T'],
          character: '\u{009c}',
          description: "STRING TERMINATOR (ST)",
      },
      Digraph {
          sequence: ['O', 'C'],
          character: '\u{009d}',
          description: "OPERATING SYSTEM COMMAND (OSC)",
      },
      Digraph {
          sequence: ['P', 'M'],
          character: '\u{009e}',
          description: "PRIVACY MESSAGE (PM)",
      },
      Digraph {
          sequence: ['A', 'C'],
          character: '\u{009f}',
          description: "APPLICATION PROGRAM COMMAND (APC)",
      },
      Digraph {
          sequence: ['/', 'c'],
          character: '\u{e001}',
          description: "JOIN THIS LINE WITH NEXT LINE (Mnemonic)",
      },
      Digraph {
          sequence: ['U', 'A'],
          character: '\u{e002}',
          description: "Unit space A (ISO-IR-8-1 064)",
      },
      Digraph {
          sequence: ['U', 'B'],
          character: '\u{e003}',
          description: "Unit space B (ISO-IR-8-1 096)",
      },
      Digraph {
          sequence: ['"', '3'],
          character: '\u{e004}',
          description: "NON-SPACING UMLAUT (ISO-IR-38 201) (character part)",
      },
      Digraph {
          sequence: ['"', '1'],
          character: '\u{e005}',
          description: "NON-SPACING DIAERESIS WITH ACCENT (ISO-IR-70 192) (character part)",
      },
      Digraph {
          sequence: ['"', '!'],
          character: '\u{e006}',
          description: "NON-SPACING GRAVE ACCENT (ISO-IR-103 193) (character part)",
      },
      Digraph {
          sequence: ['"', '\''],
          character: '\u{e007}',
          description: "NON-SPACING ACUTE ACCENT (ISO-IR-103 194) (character part)",
      },
      Digraph {
          sequence: ['"', '>'],
          character: '\u{e008}',
          description: "NON-SPACING CIRCUMFLEX ACCENT (ISO-IR-103 195) (character part)",
      },
      Digraph {
          sequence: ['"', '?'],
          character: '\u{e009}',
          description: "NON-SPACING TILDE (ISO-IR-103 196) (character part)",
      },
      Digraph {
          sequence: ['"', '-'],
          character: '\u{e00a}',
          description: "NON-SPACING MACRON (ISO-IR-103 197) (character part)",
      },
      Digraph {
          sequence: ['"', '('],
          character: '\u{e00b}',
          description: "NON-SPACING BREVE (ISO-IR-103 198) (character part)",
      },
      Digraph {
          sequence: ['"', '.'],
          character: '\u{e00c}',
          description: "NON-SPACING DOT ABOVE (ISO-IR-103 199) (character part)",
      },
      Digraph {
          sequence: ['"', ':'],
          character: '\u{e00d}',
          description: "NON-SPACING DIAERESIS (ISO-IR-103 200) (character part)",
      },
      Digraph {
          sequence: ['"', '0'],
          character: '\u{e00e}',
          description: "NON-SPACING RING ABOVE (ISO-IR-103 202) (character part)",
      },
      Digraph {
          sequence: ['"', '"'],
          character: '\u{e00f}',
          description: "NON-SPACING DOUBLE ACCUTE (ISO-IR-103 204) (character part)",
      },
      Digraph {
          sequence: ['"', '<'],
          character: '\u{e010}',
          description: "NON-SPACING CARON (ISO-IR-103 206) (character part)",
      },
      Digraph {
          sequence: ['"', ','],
          character: '\u{e011}',
          description: "NON-SPACING CEDILLA (ISO-IR-103 203) (character part)",
      },
      Digraph {
          sequence: ['"', ';'],
          character: '\u{e012}',
          description: "NON-SPACING OGONEK (ISO-IR-103 206) (character part)",
      },
      Digraph {
          sequence: ['"', '_'],
          character: '\u{e013}',
          description: "NON-SPACING LOW LINE (ISO-IR-103 204) (character part)",
      },
      Digraph {
          sequence: ['"', '='],
          character: '\u{e014}',
          description: "NON-SPACING DOUBLE LOW LINE (ISO-IR-38 217) (character part)",
      },
      Digraph {
          sequence: ['"', '/'],
          character: '\u{e015}',
          description: "NON-SPACING LONG SOLIDUS (ISO-IR-128 201) (character part)",
      },
      Digraph {
          sequence: ['"', 'i'],
          character: '\u{e016}',
          description: "GREEK NON-SPACING IOTA BELOW (ISO-IR-55 39) (character part)",
      },
      Digraph {
          sequence: ['"', 'd'],
          character: '\u{e017}',
          description: "GREEK NON-SPACING DASIA PNEUMATA (ISO-IR-55 38) (character part)",
      },
      Digraph {
          sequence: ['"', 'p'],
          character: '\u{e018}',
          description: "GREEK NON-SPACING PSILI PNEUMATA (ISO-IR-55 37) (character part)",
      },
      Digraph {
          sequence: [';', ';'],
          character: '\u{e019}',
          description: "GREEK DASIA PNEUMATA (ISO-IR-18 92)",
      },
      Digraph {
          sequence: [',', ','],
          character: '\u{e01a}',
          description: "GREEK PSILI PNEUMATA (ISO-IR-18 124)",
      },
      Digraph {
          sequence: ['b', '3'],
          character: '\u{e01b}',
          description: "GREEK SMALL LETTER MIDDLE BETA (ISO-IR-18 99)",
      },
      Digraph {
          sequence: ['C', 'i'],
          character: '\u{e01c}',
          description: "CIRCLE (ISO-IR-83 0294)",
      },
      Digraph {
          sequence: ['f', '('],
          character: '\u{e01d}',
          description: "FUNCTION SIGN (ISO-IR-143 221)",
      },
      Digraph {
          sequence: ['e', 'd'],
          character: '\u{e01e}',
          description: "LATIN SMALL LETTER EZH (ISO-IR-158 142)",
      },
      Digraph {
          sequence: ['a', 'm'],
          character: '\u{e01f}',
          description: "ANTE MERIDIAM SIGN (ISO-IR-149 0267)",
      },
      Digraph {
          sequence: ['p', 'm'],
          character: '\u{e020}',
          description: "POST MERIDIAM SIGN (ISO-IR-149 0268)",
      },
      Digraph {
          sequence: ['F', 'l'],
          character: '\u{e023}',
          description: "DUTCH GUILDER SIGN (IBM437 159)",
      },
      Digraph {
          sequence: ['G', 'F'],
          character: '\u{e024}',
          description: "GAMMA FUNCTION SIGN (ISO-10646-1DIS 032/032/037/122)",
      },
      Digraph {
          sequence: ['>', 'V'],
          character: '\u{e025}',
          description: "RIGHTWARDS VECTOR ABOVE (ISO-10646-1DIS 032/032/038/046)",
      },
      Digraph {
          sequence: ['!', '*'],
          character: '\u{e026}',
          description: "GREEK VARIA (ISO-10646-1DIS 032/032/042/164)",
      },
      Digraph {
          sequence: ['J', '<'],
          character: '\u{e028}',
          description: "LATIN CAPITAL LETTER J WITH CARON (lowercase: 000/000/001/240)",
      }];
