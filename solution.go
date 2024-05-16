package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println(isPalindrome(1234321))
}

func isPalindrome(x int) bool {

	if x < 0 {
		return false
	}

	if x < 10 {
		return true
	}

	var nrOfDigits = int(math.Floor(math.Log10(float64(x))))

	for nrOfDigits > 0 {

		left := x / int(math.Pow10(nrOfDigits))
		right := x % 10

		if left != right {
			return false
		}

		x = x / 10
		x = x % int(math.Pow10(nrOfDigits-1))

		nrOfDigits -= 2
	}
	return true
}
