package day09

import (
	"strconv"
	"strings"
)

type BlockType int

const (
	Empty BlockType = iota
	File
)

type Block struct {
	typ        BlockType
	size       int
	blockIndex int
}

func (b BlockType) Next() BlockType {
	if b == Empty {
		return File
	}
	if b == File {
		return Empty
	}
	panic("Unknown block type")
}

func processFileBlock(block Block, index int) (int, int) {
	checkSum := 0
	for i := 0; i < block.size; i++ {
		checkSum += index * block.blockIndex
		index++
		// fmt.Printf("%d", block.blockIndex)
		// time.Sleep(500 * time.Millisecond)
	}
	return checkSum, index
}

func processEmptyBlock(blocks []Block, emptySize int, index int) ([]Block, int, int) {
	checkSum := 0
	for emptySize > 0 {
		if len(blocks) == 0 {
			break
		}

		lastBlock := blocks[len(blocks)-1]
		if lastBlock.typ != File {
			blocks = blocks[:len(blocks)-1]
			continue
		}

		processSize := min(lastBlock.size, emptySize)
		partialCheckSum, newIndex := processFileBlock(Block{
			typ:        File,
			size:       processSize,
			blockIndex: lastBlock.blockIndex,
		}, index)

		checkSum += partialCheckSum
		index = newIndex
		emptySize -= processSize

		if lastBlock.size <= processSize {
			blocks = blocks[:len(blocks)-1]
		} else {
			lastBlock.size -= processSize
			blocks[len(blocks)-1] = lastBlock
		}
	}
	return blocks, checkSum, index
}

func Part1(content string) (string, error) {
	blocks, err := Parse(content)
	if err != nil {
		return "", err
	}

	checkSum := 0
	index := 0

	for len(blocks) > 0 {
		block := blocks[0]
		blocks = blocks[1:]

		if block.typ == File {
			partialCheckSum, newIndex := processFileBlock(block, index)
			checkSum += partialCheckSum
			index = newIndex
		} else {
			var partialCheckSum int
			blocks, partialCheckSum, index = processEmptyBlock(blocks, block.size, index)
			checkSum += partialCheckSum
		}
	}

	return strconv.Itoa(checkSum), nil
}

func Parse(content string) ([]Block, error) {
	blocks := make([]Block, 0)
	digits := strings.Split(content, "")
	typ := File
	blockIndex := 0

	strings.TrimSpace(strings.Repeat(". ", 3))

	for _, digit := range digits {
		size, err := strconv.Atoi(digit)
		if err != nil {
			return nil, err
		}
		blocks = append(blocks, Block{typ, size, blockIndex})
		if typ == File {
			blockIndex++
		}
		typ = typ.Next()
	}
	return blocks, nil
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func (b Block) String() string {
	character := ""
	switch b.typ {
	case Empty:
		character = "."
	case File:
		character = strconv.Itoa(b.blockIndex)
	}
	return strings.Repeat(character, b.size)
}
