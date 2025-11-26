package main

import (
	"flag"
	"fmt"
	"os"
	"strings"
	"time"
)

import (
	"aoc2024/days/day01"
	"aoc2024/days/day02"
	"aoc2024/days/day03"
	"aoc2024/days/day04"
	"aoc2024/days/day05"
	"aoc2024/days/day06"
	"aoc2024/days/day07"
	"aoc2024/days/day08"
	"aoc2024/days/day09"
	"aoc2024/days/day10"
	"aoc2024/days/day11"
	"aoc2024/days/day12"
	"aoc2024/days/day13"
	"aoc2024/days/day14"
	"aoc2024/days/day15"
	"aoc2024/days/day16"
	"aoc2024/days/day17"
	"aoc2024/days/day18"
	"aoc2024/days/day19"
	"aoc2024/days/day20"
	"aoc2024/days/day21"
	"aoc2024/days/day22"
	"aoc2024/days/day23"
	"aoc2024/days/day24"
	"aoc2024/days/day25"
)

func main() {
	day := flag.Int("day", 1, "Day to run")
	test := flag.Bool("test", false, "Run tests")
	part := flag.Int("part", 1, "Part to run")

	flag.Parse()

	var filePath string
	if *test {
		filePath = fmt.Sprintf("days/day%02d/test_input.txt", *day)
	} else {
		filePath = fmt.Sprintf("days/day%02d/input.txt", *day)
	}

	content, err := os.ReadFile(filePath)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	if *test {
		fmt.Printf("Testing day %d part %d\n", *day, *part)
	} else {
		fmt.Printf("Running day %d part %d\n", *day, *part)
	}

	start := time.Now()
	answer, err := runDay(*day, *part, strings.TrimSpace(string(content)))
	elapsed := time.Since(start)

	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	if *test {
		testAnswerPath := fmt.Sprintf("days/day%02d/answer_part%d.txt", *day, *part)
		testAnswerBytes, err := os.ReadFile(testAnswerPath)
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		testAnswer := strings.TrimSpace(string(testAnswerBytes))
		answer = strings.TrimSpace(answer)
		if testAnswer != answer {
			fmt.Printf("Test failed. Expected '%s', got '%s'\n", testAnswer, answer)
			os.Exit(1)
		}
		fmt.Printf("Test passed in %s\n", elapsed)
		os.Exit(0)
		return
	}

	fmt.Printf("Answer: %s\n", answer)
	fmt.Printf("Time: %s\n", elapsed)
}

func runDay(day int, part int, content string) (string, error) {
	index := day*10 + part
	switch index {
	case 1_1:
		return day01.Part1(content)
	case 1_2:
		return day01.Part2(content)
	case 2_1:
		return day02.Part1(content)
	case 2_2:
		return day02.Part2(content)
	case 3_1:
		return day03.Part1(content)
	case 3_2:
		return day03.Part2(content)
	case 4_1:
		return day04.Part1(content)
	case 4_2:
		return day04.Part2(content)
	case 5_1:
		return day05.Part1(content)
	case 5_2:
		return day05.Part2(content)
	case 6_1:
		return day06.Part1(content)
	case 6_2:
		return day06.Part2(content)
	case 7_1:
		return day07.Part1(content)
	case 7_2:
		return day07.Part2(content)
	case 8_1:
		return day08.Part1(content)
	case 8_2:
		return day08.Part2(content)
	case 9_1:
		return day09.Part1(content)
	case 9_2:
		return day09.Part2(content)
	case 10_1:
		return day10.Part1(content)
	case 10_2:
		return day10.Part2(content)
	case 11_1:
		return day11.Part1(content)
	case 11_2:
		return day11.Part2(content)
	case 12_1:
		return day12.Part1(content)
	case 12_2:
		return day12.Part2(content)
	case 13_1:
		return day13.Part1(content)
	case 13_2:
		return day13.Part2(content)
	case 14_1:
		return day14.Part1(content)
	case 14_2:
		return day14.Part2(content)
	case 15_1:
		return day15.Part1(content)
	case 15_2:
		return day15.Part2(content)
	case 16_1:
		return day16.Part1(content)
	case 16_2:
		return day16.Part2(content)
	case 17_1:
		return day17.Part1(content)
	case 17_2:
		return day17.Part2(content)
	case 18_1:
		return day18.Part1(content)
	case 18_2:
		return day18.Part2(content)
	case 19_1:
		return day19.Part1(content)
	case 19_2:
		return day19.Part2(content)
	case 20_1:
		return day20.Part1(content)
	case 20_2:
		return day20.Part2(content)
	case 21_1:
		return day21.Part1(content)
	case 21_2:
		return day21.Part2(content)
	case 22_1:
		return day22.Part1(content)
	case 22_2:
		return day22.Part2(content)
	case 23_1:
		return day23.Part1(content)
	case 23_2:
		return day23.Part2(content)
	case 24_1:
		return day24.Part1(content)
	case 24_2:
		return day24.Part2(content)
	case 25_1:
		return day25.Part1(content)
	case 25_2:
		return day25.Part2(content)

	default:
		return "", fmt.Errorf("Day %d part %d not implemented", day, part)
	}
}
