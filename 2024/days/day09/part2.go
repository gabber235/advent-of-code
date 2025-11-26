package day09

import (
	"strconv"
)

func removeFragmentation(blocks []Block) []Block {
	index := len(blocks) - 1
	for index >= 0 {
		block := blocks[index]
		if block.typ != File {
			index--
			continue
		}
		foundSpace := false
	searchLoop:
		for i := 0; i < min(index, len(blocks)); i++ {
			lookingBlock := blocks[i]
			if lookingBlock.typ != Empty {
				continue searchLoop
			}

			if block.size > lookingBlock.size {
				continue searchLoop
			}

			if block.size == lookingBlock.size {
				blocks[index] = lookingBlock
				blocks[i] = block
				index--
				foundSpace = true
				break searchLoop
			}

			blocks[index] = Block{
				typ:        Empty,
				size:       block.size,
				blockIndex: block.blockIndex,
			}

			lookingBlock.size -= block.size
			blocks[i] = lookingBlock
			// Insert the block before the looking block
			copy(blocks[i+1:], blocks[i:])
			blocks[i] = block

			foundSpace = true

			break searchLoop
		}

		if !foundSpace {
			index--
		}
	}
	return blocks
}

func Part2(content string) (string, error) {
	blocks, err := Parse(content)
	if err != nil {
		return "", err
	}

	blocks = removeFragmentation(blocks)

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
			index += block.size
		}
	}

	return strconv.Itoa(checkSum), nil
}
