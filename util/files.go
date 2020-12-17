package util

import "io/ioutil"

func ReadFile(name string) string {
	res, err := ioutil.ReadFile(name)
	if err != nil {
		panic(err)
	}
	return string(res)
}

func GetInts(strings []string) (out []int) {
	for _, s := range strings {
		out = append(out, GetInt(s))
	}
	return
}