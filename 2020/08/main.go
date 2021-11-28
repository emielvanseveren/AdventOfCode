package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type InstructionType string
const (
	NOP InstructionType = "nop"
	ACC = "acc"
	JMP = "jmp"
)

type Instruction struct {
	instruction InstructionType
	amount int
	executed bool
}

func main() {
	input, _ := ioutil.ReadFile("./08/input")
	instructionSet := make([]Instruction, 0)

	for _, l := range strings.Split(strings.TrimSpace(string(input)), "\n"){
		parts := strings.Split(l, " ")
		instructionType, num:= parts[0], parts[1]

		// remove unnecessary plus
		if strings.Contains(num, "+") { num = strings.ReplaceAll(num, "+", "")}
		amount, _ := strconv.Atoi(num)
		instructionSet = append(instructionSet,Instruction { InstructionType(instructionType) , amount,  false })
	}

	for i:=0; i<len(instructionSet);i++ {
		c :=append([]Instruction(nil), instructionSet...)

		if c[i].instruction == NOP {
			c[i].instruction = JMP
		} else if c[i].instruction == JMP {
			c[i].instruction = NOP
		} else {
			continue // was ACC so no need to check.
		}
		corr, acc := execute(c)
		if corr { fmt.Println(acc)}
	}
}

func execute(instructions []Instruction) (corr bool, acc int){
	i:=0
	for !instructions[i].executed {
		instructions[i].executed = true

		switch instructions[i].instruction{
		case ACC:
			acc += instructions[i].amount
			i++
			break
		case NOP:
			i++
			break
		case JMP:
			i += instructions[i].amount
			break
		}
		if i == len(instructions) {
			corr = true
			return
		}
	}
	return
}