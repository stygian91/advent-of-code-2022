package entry

type Entry struct {
	value    uint8
	isStart  bool
	isFinish bool
}

func (this Entry) Walkable(other Entry) bool {
  // TODO:
  return false
}


func FromRune(rune rune) Entry {
  var result Entry = Entry{
  	value:    0,
  	isStart:  false,
  	isFinish: false,
  }

  if rune == 'S' {
    result.value = 0
    result.isStart = true
  } else if rune == 'E' {
    result.value = 25
    result.isFinish = true
  } else {
    result.value = uint8(rune) - uint8('a')
  }

  return result
}
