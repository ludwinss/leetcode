package main

import (
	"fmt"
	"sync"
)

type stack struct {
	lock  *sync.Mutex
	queue []string
}

func (s *stack) Push(parentheses string) {
	s.lock.Lock()

	s.queue = append(s.queue, parentheses)

	defer s.lock.Unlock()
}

func (s *stack) Pop() string {
	s.lock.Lock()

	lenght := len(s.queue)

	if lenght == 0 {
		return ""
	}

	lastParentheses := s.queue[lenght-1]

	s.queue = s.queue[:lenght-1]

	defer s.lock.Unlock()

	return lastParentheses
}

func isValid(s string) bool {
	stackParenthese := stack{lock: &sync.Mutex{}, queue: []string{}}

	mapRight := map[string]string{"{": "}", "[": "]", "(": ")"}
	mapLeft := map[string]string{"}": "{", "]": "[", ")": "("}

	for i := 0; i < len(s); i++ {
		_, ok := mapRight[string(s[i])]

		if ok {
			stackParenthese.Push(string(s[i]))
		} else {
			righParentheses := stackParenthese.Pop()
			if righParentheses != mapLeft[string(s[i])] {
				return false
			}
		}
	}
	if stackParenthese.Pop() != "" {
		return false
	}
	return true
}

func main() {
	fmt.Println(isValid("(("))
}
