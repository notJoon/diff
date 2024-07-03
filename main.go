package main

import (
	"fmt"

	"github.com/notJoon/diff/myers"
)

func main() {
	str1 := "아스키 아닌 것도 되나?"
	str2 := "아스키 아닌 것도 됨."

	diff := myers.MyersDiff(str1, str2)
	formattedDiff := myers.FormatDiff(diff)

	fmt.Println("String 1:", str1)
	fmt.Println("String 2:", str2)
	fmt.Println("Diff    :", formattedDiff)
}
