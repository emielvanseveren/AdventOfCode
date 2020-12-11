package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"regexp"
)

func main() {
	file, err := os.Open("./04/input")

	if err != nil { log.Fatalf("ERROR: %s", err)}
	defer file.Close()

	s := bufio.NewScanner(file)
	validPassports := 0
	amountOfPassports := 0
	passport := make(map[string]string)

	for s.Scan() {
		if s.Text() !="" {
			fields := strings.Split(s.Text(), " ")
			for i:=0;i<len(fields);i++ {
				f :=strings.Split(fields[i], ":")
				passport[f[0]] = f[1]
			}
		} else {
			amountOfPassports++
			validPassports += isValid(passport)	 // isValid returns 1 if the passport is valid
			passport = make(map[string]string) // clear map
		}
	}
	// ATTENTION: last empty line is not scanned by scanner.
		amountOfPassports++
		validPassports	+= isValid(passport)

	fmt.Printf("Amount of passports: %d\n", amountOfPassports)
	fmt.Printf("There are %d valid passports.\n", validPassports)
}

func isValid(passport map[string]string) int {
	if len(passport) < 7 { return 0 } // if it has less than 7 no need to even check if it has all the required keys.

	keys := []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"} // cid is not required so don't bother checking

	for i:=0; i<len(keys);i++ {
			val, ok := passport[keys[i]]
			if !ok {return 0}

			// part 2: additional validation per key
			switch keys[i] {
			case "byr":
				v,_ := strconv.Atoi(val)
				if len(val) != 4 || v < 1920 || v > 2002 {return 0}
			case "iyr":
				v,_ := strconv.Atoi(val)
				if len(val) != 4 || v < 2010 || v > 2020 {return 0}
			case "eyr":
				v,_ := strconv.Atoi(val)
				if len(val) != 4 || v < 2020 || v > 2030 {return 0}
			case "hgt":
				if strings.Contains(val, "cm"){
					s := strings.Split(val, "c")
					num,_ := strconv.Atoi(s[0])
					if num < 150 || num > 193 { return 0}
				} else if strings.Contains(val, "in"){
					s := strings.Split(val, "i")
					num,_ := strconv.Atoi(s[0])
					if num < 59 || num > 76{ return 0}
				} else {
					return 0 // does not contain cm or in (inch)
				}
			case "hcl":
				if match, _ := regexp.MatchString(`^#[0-9a-f]{6}$`, val); !match {
					return 0
				} // check numbers of chars.
			case "ecl":
				ecl := []string {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}
				if _, ok := Find(ecl, val); !ok {
					return 0
				}
			case "pid":
				if match, _ := regexp.MatchString(`^[0-9]{9}$`, val); !match { // should be exactly 6 => ^ and $
					return 0
				}
			}
	}
	return 1
}

func Find(slice []string, val string) (int, bool) {
    for i, item := range slice {
        if item == val {
            return i, true
        }
    }
    return -1, false
}

