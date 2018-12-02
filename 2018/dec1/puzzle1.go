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

func solveA(filename string) int {
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

func solveB(filename string) int {
	changes := readInput(filename)
	frequencies := make(map[string]int)
	firstDouble := 0
	acc := 0
	total_changes := len(changes)
	frequencies["0"] = 1
	for i := 0; i < total_changes; i++ {
		number, err := strconv.Atoi(changes[i])
		if err != nil {
			panic(err)
		}
		acc = acc + number
		freq_index := strconv.Itoa(acc)
		frequencies[freq_index] += 1
		if frequencies[freq_index] == 2 {
			firstDouble = acc
			break
		}
		if total_changes == i+1 {
			i = -1
		}
	}
	return firstDouble
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
	return strings.Split(clean_data, "\n")
}
