package myers

import (
	"fmt"
	"strings"
	"testing"
)

func TestMyersDiff(t *testing.T) {
	t.Parallel()
	tests := []struct {
		name     string
		old      string
		new      string
		expected string
	}{
		{
			name:     "No difference",
			old:      "abc",
			new:      "abc",
			expected: "abc",
		},
		{
			name:     "Simple insertion",
			old:      "ac",
			new:      "abc",
			expected: "a[+b]c",
		},
		{
			name:     "Simple deletion",
			old:      "abc",
			new:      "ac",
			expected: "a[-b]c",
		},
		{
			name:     "Simple substitution",
			old:      "abc",
			new:      "abd",
			expected: "ab[-c][+d]",
		},
		{
			name:     "Multiple changes",
			old:      "The quick brown fox jumps over the lazy dog",
			new:      "The quick brown cat jumps over the lazy dog",
			expected: "The quick brown [-fox][+cat] jumps over the lazy dog",
		},
		{
			name:     "Prefix and suffix",
			old:      "Hello, world!",
			new:      "Hello, beautiful world!",
			expected: "Hello, [+beautiful ]world!",
		},
		{
			name:     "Complete change",
			old:      "abcdef",
			new:      "ghijkl",
			expected: "[-abcdef][+ghijkl]",
		},
		{
			name:     "Empty strings",
			old:      "",
			new:      "",
			expected: "",
		},
		{
			name:     "Old empty",
			old:      "",
			new:      "abc",
			expected: "[+abc]",
		},
		{
			name:     "New empty",
			old:      "abc",
			new:      "",
			expected: "[-abc]",
		},
		{
			name:     "Emoji diff",
			old:      "Hello 👋 World 🌍",
			new:      "Hello 👋 Beautiful 🌸 World 🌍",
			expected: "Hello 👋 [+Beautiful 🌸 ]World 🌍",
		},
		{
			name:     "Mixed multibyte and ASCII",
			old:      "こんにちは World",
			new:      "こんばんは World",
			expected: "こん[-にち][+ばん]は World",
		},
		{
			name:     "Chinese characters",
			old:      "我喜欢编程",
			new:      "我喜欢看书和编程",
			expected: "我喜欢[+看书和]编程",
		},
		{
			name:     "Combining characters",
			old:      "e\u0301", // é (e + ´)
			new:      "e\u0300", // è (e + `)
			expected: "e[-\u0301][+\u0300]",
		},
		{
			name:     "Right-to-Left languages",
			old:      "שלום",
			new:      "שלום עולם",
			expected: "שלום[+ עולם]",
		},
		{
			name:     "Normalization NFC and NFD",
			old:      "e\u0301", // NFD (decomposed)
			new:      "\u00e9",  // NFC (precomposed)
			expected: "[-e\u0301][+\u00e9]",
		},
		{
			name:     "Case sensitivity",
			old:      "abc",
			new:      "Abc",
			expected: "[-a][+A]bc",
		},
		{
			name:     "Surrogate pairs",
			old:      "Hello 🌍",
			new:      "Hello 🌎",
			expected: "Hello [-🌍][+🌎]",
		},
		{
			name:     "Control characters",
			old:      "Line1\nLine2",
			new:      "Line1\r\nLine2",
			expected: "Line1[+\r]\nLine2",
		},
		{
			name:     "Mixed scripts",
			old:      "Hello नमस्ते こんにちは",
			new:      "Hello สวัสดี こんにちは",
			expected: "Hello [-नमस्ते][+สวัสดี] こんにちは",
		},
		{
			name:     "Unicode normalization",
			old:      "é",       // U+00E9 (precomposed)
			new:      "e\u0301", // U+0065 U+0301 (decomposed)
			expected: "[-é][+e\u0301]",
		},
		{
			name:     "Directional marks",
			old:      "Hello\u200Eworld", // LTR mark
			new:      "Hello\u200Fworld", // RTL mark
			expected: "Hello[-\u200E][+\u200F]world",
		},
		{
			name:     "Zero-width characters",
			old:      "ab\u200Bc", // Zero-width space
			new:      "abc",
			expected: "ab[-\u200B]c",
		},
		{
			name:     "Worst-case scenario (completely different strings)",
			old:      strings.Repeat("a", 1000),
			new:      strings.Repeat("b", 1000),
			expected: "[-" + strings.Repeat("a", 1000) + "][+" + strings.Repeat("b", 1000) + "]",
		},
		{
			name:     "Very long strings",
			old:      strings.Repeat("a", 10000) + "b" + strings.Repeat("a", 10000),
			new:      strings.Repeat("a", 10000) + "c" + strings.Repeat("a", 10000),
			expected: strings.Repeat("a", 10000) + "[-b][+c]" + strings.Repeat("a", 10000),
		},
	}

	for _, tt := range tests {
		tt := tt
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()
			diff := MyersDiff(tt.old, tt.new)
			result := FormatDiff(diff)
			if result != tt.expected {
				t.Errorf("Expected: %s, got: %s", tt.expected, result)
			}
		})
	}
}

func generateTestEdits(size int) []Edit {
	edits := make([]Edit, size)
	for i := 0; i < size; i++ {
		edits[i] = Edit{
			Type: EditType(i % 3),
			Char: rune('a' + i%26),
		}
	}
	return edits
}

func BenchmarkFormatDiff(b *testing.B) {
	sizes := []int{10, 100, 1000, 10000}

	for _, size := range sizes {
		testEdits := generateTestEdits(size)

		b.Run(fmt.Sprintf("Original-Size%d", size), func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				FormatDiff(testEdits)
			}
		})
	}
}
