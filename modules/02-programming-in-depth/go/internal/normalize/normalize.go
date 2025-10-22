package normalize

import (
	"regexp"
	"strings"
)

var reWS = regexp.MustCompile(`\s+`)

// NormalizeUserInput trims and collapses all whitespace to a single space.
func NormalizeUserInput(s string) string {
	trimmed := strings.TrimSpace(s)
	return reWS.ReplaceAllString(trimmed, " ")
}
