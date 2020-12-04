package main

func partOne(checker map[string]string) bool {
	return (checker["cid"] == "" && len(checker) == 7) || (checker["cid"] != "" && len(checker) == 8)
}
