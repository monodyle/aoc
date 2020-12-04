package main

import (
	"regexp"
	"strconv"
	"strings"
	"unicode"
)

func partTwo(checker map[string]string) bool {
	byr, err := strconv.Atoi(checker["byr"])
	if err != nil || len(checker["byr"]) != 4 || (byr < 1920 || byr > 2002) {
		return false
	}

	iyr, err := strconv.Atoi(checker["iyr"])
	if err != nil || len(checker["iyr"]) != 4 || (iyr < 2010 || iyr > 2020) {
		return false
	}

	eyr, err := strconv.Atoi(checker["eyr"])
	if err != nil || len(checker["eyr"]) != 4 || (eyr < 2020 || eyr > 2030) {
		return false
	}

	var hgt int
	if strings.HasSuffix(checker["hgt"], "cm") {
		hgt, err = strconv.Atoi(strings.TrimSuffix(checker["hgt"], "cm"))
		if err != nil || (hgt < 150 || hgt > 193) {
			return false
		}
	} else if strings.HasSuffix(checker["hgt"], "in") {
		hgt, err = strconv.Atoi(strings.TrimSuffix(checker["hgt"], "in"))
		if err != nil || (hgt < 59 || hgt > 76) {
			return false
		}
	} else {
		return false
	}

	if !regexp.MustCompile("^#[a-f0-9]{6}$").MatchString(checker["hcl"]) {
		return false
	}

	var colors = map[string]bool{"amb": true, "blu": true, "brn": true, "gry": true, "grn": true, "hzl": true, "oth": true}
	if !colors[checker["ecl"]] {
		return false
	}

	var pid string = checker["pid"]
	if len(pid) != 9 {
		return false
	}
	for _, c := range pid {
		if !unicode.IsDigit(c) {
			return false
		}
	}

	return true
}
