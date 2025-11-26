package day14

import (
	"fmt"
	"image"
	"image/color"
	"image/png"
	"os"
	"path/filepath"
)

func generateImage(robots []Robot, width, height int, time int, outputDir string) error {
	img := image.NewRGBA(image.Rect(0, 0, width, height))

	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			img.Set(x, y, color.Black)
		}
	}

	for _, robot := range robots {
		img.Set(robot.position.x, robot.position.y, color.White)
	}

	if err := os.MkdirAll(outputDir, 0o755); err != nil {
		return err
	}

	filename := filepath.Join(outputDir, fmt.Sprintf("frame_%05d.png", time))
	f, err := os.Create(filename)
	if err != nil {
		return err
	}
	defer f.Close()

	if err := png.Encode(f, img); err != nil {
		return err
	}

	return nil
}

func GenerateAllImages(content string, outputDir string) error {
	width, height := 101, 103
	originalRobots := parseInput(content)

	for time := 0; time <= 10403; time++ {
		currentRobots := make([]Robot, len(originalRobots))
		copy(currentRobots, originalRobots)

		for i := 0; i < time; i++ {
			for j := range currentRobots {
				moveRobot(&currentRobots[j], width, height)
			}
		}

		if err := generateImage(currentRobots, width, height, time, outputDir); err != nil {
			return fmt.Errorf("error generating frame %d: %v", time, err)
		}

		if time%100 == 0 {
			fmt.Printf("Generated frame %d/10403\n", time)
		}
	}

	return nil
}

func Part2(content string) (string, error) {
	if err := GenerateAllImages(content, "frames"); err != nil {
		return "", err
	}
	return "Generated all frames", nil
}
