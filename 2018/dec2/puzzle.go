package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/texttheater/golang-levenshtein/levenshtein"
	"github.com/yieldbot/gocli"
)

var (
	cli          gocli.Cli
	usageFlag    bool
	versionFlag  bool
	filenameFlag bool
)

type idCount struct {
	two   int
	three int
}

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
		Name:        "puzzle2",
		Version:     "0.0.1",
		Description: "AoC puzzle2",
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

func extractCounts(boxId string) idCount {

	//iterate through each character increment counter for each character

	acc := make(map[string]int)
	characters := strings.Split(boxId, "")
	countofTwo := 0
	countofThree := 0
	for i := 0; i < len(characters); i++ {
		acc[characters[i]]++
	}
	for _, v := range acc {
		if v == 2 && countofTwo == 0 {
			countofTwo += 1
		}
		if v == 3 && countofThree == 0 {
			countofThree += 1
		}
	}
	return idCount{two: countofTwo, three: countofThree}
}

func solveA(filename string) int {
	boxIds := readInput(filename)
	accTwo := 0
	accThree := 0
	for i := 0; i < len(boxIds); i++ {
		counts := extractCounts(boxIds[i])
		fmt.Println("two: ", counts.two)
		fmt.Println("three: ", counts.three)
		accTwo += counts.two
		accThree += counts.three
		fmt.Println("Total two: ", accTwo)
		fmt.Println("Total three: ", accThree)
	}
	return accTwo * accThree
}

var PuzzleOptions levenshtein.Options = levenshtein.Options{
	InsCost: 2,
	DelCost: 2,
	SubCost: 1,
	Matches: func(sourceCharacter rune, targetCharacter rune) bool {
		return sourceCharacter == targetCharacter
	},
}

func solveB(filename string) string {
	//iterate through box ids
	//determine Levenshtien Distance(LD) for all
	//when LD
	boxIds := readInput(filename)
	source := ""
	target := ""
	distance := 0
	for i := 0; i < len(boxIds); i++ {
		for j := i + 1; j < len(boxIds); j++ {
			source = boxIds[i]
			target = boxIds[j]
			distance = levenshtein.DistanceForStrings([]rune(source), []rune(target), PuzzleOptions)
			if distance == 1 {
				break
			}
		}
		if distance == 1 {
			break
		}
	}
	var strs []string
	strs = append(strs, source)
	strs = append(strs, target)
	return strings.Join(strs, "--")
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
