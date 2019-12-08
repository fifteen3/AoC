package main

import (
	"errors"
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
		fmt.Println(solveA(cli.SubCommandArgsMap["filename"]))
	} else if cli.SubCommand == "solveB" {
		fmt.Printf("%.1f", solveB(cli.SubCommandArgsMap["filename"]))
	} else if versionFlag {
		cli.PrintVersion(true)
	} else {
		cli.PrintUsage()
	}
}

type Vertex struct {
	x float64
	y float64
}

type LineSegment struct {
	start Vertex
	end   Vertex
}

func buildPath(input string) []Vertex {
	path := []Vertex{Vertex{0, 0}}
	instructions := strings.Split(input, ",")
	for i := 1; i < len(instructions); i++ {
		fmt.Println("path size: ", len(path))
		path = append(path, calcXY(instructions[i], path[i-1]))
	}
	return path
}

func buildLineSegments(points []Vertex) []LineSegment {
	var segments []LineSegment
	for p := range points {
		if p+1 < len(points) {
			segments = append(segments, createLineSegment(points[p], points[p+1]))
		}
	}
	return segments
}
func createLineSegment(a, b Vertex) LineSegment {
	return LineSegment{a, b}
}
func calcXY(instruction string, lastvertex Vertex) Vertex {
	dir := instruction[0:1]
	magnitude, _ := strconv.ParseFloat(instruction[1:], 64)
	x := 0.0
	y := 0.0
	if dir == "U" {
		y = lastvertex.y + magnitude
		x = lastvertex.x
	}
	if dir == "D" {
		y = lastvertex.y + magnitude*-1
		x = lastvertex.x
	}
	if dir == "L" {
		x = lastvertex.x + magnitude*-1
		y = lastvertex.y
	}
	if dir == "R" {
		x = lastvertex.x + magnitude
		y = lastvertex.y
	}
	return Vertex{x, y}
}

func findIntersectingPoints(segA, segB []LineSegment) []Vertex {
	var points []Vertex
	for a := range segA {
		for b := range segB {
			point, err := findIntersection(segA[a], segB[b])
			if err != nil {

			} else {
				points = append(points, point)
			}
		}
	}
	return points
}

func findIntersection(a, b LineSegment) (Vertex, error) {
	return Vertex{}, errors.New("no intersection")
}

func solveA(filename string) float64 {
	changes := readInput(filename)
	path := make([][]Vertex, 0)
	for i := 0; i < len(changes); i++ {
		path = append(path, buildPath(changes[i]))
	}
	fmt.Println("paths...")
	fmt.Println(path[1:2][0])
	fmt.Println(path[2:][0])
	origin := Vertex{0, 0}
	segmentsA := buildLineSegments(path[1:2][0])
	segmentsB := buildLineSegments(path[2:][0])
	common := findIntersectingPoints(segmentsA, segmentsB)
	smallest_distance := 10000.0
	fmt.Println("common points")
	fmt.Println(common)
	dist := 1000000.0
	for c := range common {
		dist = calcDistance(origin, common[c])
		fmt.Println("dist: %.1f", dist)
		if dist > 0 && dist < smallest_distance {
			fmt.Println("dist: %.1f", dist)
			smallest_distance = dist
		}
	}
	return smallest_distance
}

func calcDistance(point1 Vertex, point2 Vertex) float64 {
	return math.Abs(point2.x-point1.x) + math.Abs(point2.y-point1.y)
}

func solveB(filename string) float64 {
	return 0.0
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
