package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"math"
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
		fmt.Printf("%.1f", solveA(cli.SubCommandArgsMap["filename"]))
	} else if cli.SubCommand == "solveB" {
		fmt.Printf("%.1f", solveB(cli.SubCommandArgsMap["filename"]))
	} else if versionFlag {
		cli.PrintVersion(true)
	} else {
		cli.PrintUsage()
	}
}

func solveA(filename string) float64 {
	changes := readInput(filename)
	acc := 0.0
	for i := 0; i < len(changes); i++ {
		number, err := strconv.ParseFloat(changes[i], 64)
		if err != nil {
			panic(err)
		}
		fuel := math.Floor(number/3) - 2
		acc += fuel
	}
	return acc
}

func solveB(filename string) float64 {
	changes := readInput(filename)
	acc := 0.0
	for i := 0; i < len(changes); i++ {
		number, err := strconv.ParseFloat(changes[i], 64)
		if err != nil {
			panic(err)
		}
		acc += calcFuel(number)
	}
	return acc
}

func calcFuel(fuel float64) float64 {
	calcdFuel := math.Floor(fuel/3) - 2
	if calcdFuel < 1 {
		return 0
	} else {
		calcdFuel += calcFuel(calcdFuel)
	}
	return calcdFuel
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
