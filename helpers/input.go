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
	result := []int{}
	for _, l := range splitter {
		i, err := strconv.Atoi(l)
		if err != nil {
			return nil, err
		}
		result = append(result, i)
	}
	return result, nil
}

// ScanFile read and return file line by line
func ScanFile(filepath string, f func(s string) error) error {
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
