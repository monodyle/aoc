package main

import (
	"testing"
)

func TestSolution(t *testing.T) {
	var partOne, partTwo = solution("./inputTest")
	const correctOne, correctTwo = 514579, 241861950

	if correctOne != partOne {
		t.Fatalf("✖ Failed! Want %d, given %d", correctOne, partOne)
	}
	if correctTwo != partTwo {
		t.Fatalf("✖ Failed! Want %d, given %d", correctTwo, partTwo)
	}
}
