package helpers

import (
	"bufio"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

// ParseInputInts read and return list of int
func ParseInputInts(filepath string) ([]int, error) {
	content, err := ioutil.ReadFile(filepath)
	if err != nil {
		return nil, err
	}
	splitter := strings.Split(strings.TrimSpace(string(content)), "\n")
	result := make([]int, 0, len(splitter))
	for _, l := range splitter {
		i, err := strconv.Atoi(l)
		if err != nil {
			return nil, err
		}
		result = append(result, i)
	}
	return result, nil
}

// ScanLines read and return file line by line
func ScanLines(filepath string, f func(line string) error) error {
	file, err := os.Open(filepath)
	if err != nil {
		return err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		if err := f(scanner.Text()); err != nil {
			return err
		}
	}
	return scanner.Err()
}

// ScanGroups read and return file line by line
func ScanGroups(filepath string, f func([]string) error) error {
	content, err := ioutil.ReadFile(filepath)
	if err != nil {
		return err
	}

	groups := strings.Split(strings.TrimSpace(string(content)), "\n\n")
	for _, group := range groups {
		lines := strings.Split(strings.TrimSpace(string(group)), "\n")
		if err := f(lines); err != nil {
			return err
		}
	}

	return nil
}
