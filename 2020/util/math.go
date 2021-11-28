package util

import (
	"strconv"
)


func GetInt(in string) int {
	res, err := strconv.Atoi(in)
	if err != nil {
		panic(err)
	}
	return res
}

func Sum(ints ...int) (out int) {
	for _, v := range ints {
		out += v
	}
	return
}