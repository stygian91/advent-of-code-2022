package common

var (
	aValue        byte = byte("a"[0])
	zValue        byte = byte("z"[0])
	capitalAValue byte = byte("A"[0])
)

func GetValue(letter byte) byte {
	if letter <= zValue && letter >= aValue {
		return (letter - aValue) + 1
	}

	return (letter - capitalAValue) + 27
}

// This can be sped up with a precomputed value table
func GetLetterU64(letter string) uint64 {
	value := GetValue(byte(letter[0]))

	return 1 << (value - 1)
}

func GetBackpackAsU64(backpack string) uint64 {
	var result uint64 = 0

	for _, letter := range backpack {
		result = result | GetLetterU64(string(letter))
	}

	return result
}

// same as GetLetterU64
func GetValueFromU64(encoded uint64) uint64 {
	var work uint64 = encoded

	for i := 0; i < 64; i++ {
		if work&1 == 1 {
			return uint64(i) + 1
		}

		work = work >> 1
	}

	return 0
}

func FindChunkCommon(chunk []string) uint64 {
	if len(chunk) == 0 {
		panic("Attempted to find common on empty chunk")
	}

	var result uint64 = GetBackpackAsU64(chunk[0])

	for _, backpack := range chunk[1:] {
		result = result & GetBackpackAsU64(backpack)
	}

	return result
}
