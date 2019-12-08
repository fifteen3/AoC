package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"

	"github.com/yieldbot/gocli"
)

var (
	cli          gocli.Cli
	usageFlag    bool
	versionFlag  bool
	filenameFlag bool
)

func init() {
	//Init flags
	flag.BoolVar(&usageFlag, "h", false, "Display usage")
	flag.BoolVar(&usageFlag, "help", false, "Display usage")
	flag.BoolVar(&versionFlag, "version", false, "Display version")
	flag.BoolVar(&versionFlag, "v", false, "Display version")
	flag.BoolVar(&filenameFlag, "filename", false, "File containing data set to load")
}

func main() {
	cli = gocli.Cli{
		Name:        "puzzle1",
		Version:     "0.0.1",
		Description: "AoC puzzle1",
		Commands: map[string]string{
			"solveA": "Solve the puzzle",
			"solveB": "Solve the puzzle",
		},
	}
	cli.Init()

	if cli.SubCommand == "solveA" {
		fmt.Println(solveA(cli.SubCommandArgsMap["filename"]))
	} else if cli.SubCommand == "solveB" {
		fmt.Println(solveB(cli.SubCommandArgsMap["filename"]))
	} else if versionFlag {
		cli.PrintVersion(true)
	} else {
		cli.PrintUsage()
	}
}
func readInstructionSet(start int, opcodes []string) []string {
	var tot_opcodes = len(opcodes)
	var end = start + 4
	if tot_opcodes-start < 4 {
		end = tot_opcodes
	}
	fmt.Println("len: ", tot_opcodes, " end: ", end)
	fmt.Println(start, ": ", end)
	return opcodes[start:end]
}

func addOpcodes(a int, b int, store_loc int, opcodes []string) {
	operand1, err := strconv.Atoi(opcodes[a])
	if err != nil {
		panic(err)
	}
	operand2, err := strconv.Atoi(opcodes[b])
	if err != nil {
		panic(err)
	}
	value := strconv.Itoa(operand1 + operand2)
	fmt.Println("adding: ", operand1, " to ", operand2, " = ", value)
	opcodes[store_loc] = value
}

func multiplyOpcodes(a int, b int, store_loc int, opcodes []string) {
	operand1, err := strconv.Atoi(opcodes[a])
	if err != nil {
		panic(err)
	}
	operand2, err := strconv.Atoi(opcodes[b])
	if err != nil {
		panic(err)
	}
	value := strconv.Itoa(operand1 * operand2)
	opcodes[store_loc] = value
}

func solveA(filename string) string {
	opcodes := readInput(filename)
	start := 0
	for {
		fmt.Println("reading instructions...", start)
		var instructions = readInstructionSet(start, opcodes[:])
		if instructions[0] == "99" {
			fmt.Println("halting...", instructions[0])
			break
		}
		if instructions[0] == "1" {
			fmt.Println("adding...", instructions[0])
			operand1, err := strconv.Atoi(instructions[1])
			operand2, err := strconv.Atoi(instructions[2])
			store, err := strconv.Atoi(instructions[3])
			fmt.Println("%d to %d", opcodes[operand1], opcodes[operand2])
			if err != nil {
				panic(err)
			}
			addOpcodes(operand1, operand2, store, opcodes[:])
		}
		if instructions[0] == "2" {
			fmt.Println("mutiplying...", instructions[0])
			operand1, err := strconv.Atoi(instructions[1])
			operand2, err := strconv.Atoi(instructions[2])
			store, err := strconv.Atoi(instructions[3])
			fmt.Println("%d x %d", opcodes[operand1], opcodes[operand2])
			if err != nil {
				panic(err)
			}
			multiplyOpcodes(operand1, operand2, store, opcodes[:])
		}
		fmt.Println("incrementing...")
		start += 4
	}
	return opcodes[0]
}

func getOpcode(opcodes []string, opcode string) int {
	pos, err := strconv.Atoi(opcode)
	if err != nil {
		panic(err)
	}
	value, err := strconv.Atoi(opcodes[pos])
	if err != nil {
		panic(err)
	}
	return value
}

func solveB(filename string) int {
	changes := readInput(filename)
	acc := 0
	for i := 0; i < len(changes); i++ {
		number, err := strconv.Atoi(changes[i])
		if err != nil {
			panic(err)
		}
		acc += number
	}
	return acc
}

func readInput(filename string) []string {
	data, err := ioutil.ReadFile(filename)
	if err != nil {
		fmt.Println("Trouble reading file.")
		panic(err)
	}
	n := len(data)
	stringData := string(data[:n])
	clean_data := strings.TrimSuffix(stringData, "\n")
	return strings.Split(clean_data, ",")
}
