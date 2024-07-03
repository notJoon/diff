package myers

import (
	"fmt"
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
