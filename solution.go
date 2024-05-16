package  main

import "fmt"

func main() {
  response :=romanToInt("MCMXCIV")

  fmt.Println(response)

}

func romanToInt(s string) int {

	romanMap := map[string]int{"I": 1, "V": 5, "X": 10, "L": 50, "C": 100, "D": 500, "M": 1000}

	var sum int = 0
	var acc int = 0

	for i := len(s) - 1; i >= 0; i-- {

    fmt.Println("s[i]:",string(s[i]))

		map_value := romanMap[string(s[i])]

		if acc > map_value  {
			sum -= map_value
			acc = 0
		} else {
			sum += map_value
			acc += map_value
      fmt.Println(acc,"<", s[i])
      fmt.Println("sum:",sum)
      fmt.Println("acc:",acc)
		}

	}

	return sum
}

