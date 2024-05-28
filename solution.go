package main

import (
	"fmt"
	"regexp"
	"strings"
)

func main() {

	emails := []string{"ludwinss@gmail.com", "ludwin.flores.ma+mani.f@gmail.com", "lll@gm.ail.com", "ff@gmail.com"}
	fmt.Println(numUniqueEmails(emails))
}

// NOTA Si no hubiera usado la libreria regexp, la solucion seria mas rapida
func numUniqueEmails(emails []string) int {
	pattern := `\+.*`
	regExp, _ := regexp.Compile(pattern)
	emailMap := make(map[string]bool)

	for i := 0; i < len(emails); i++ {
		email := strings.Split(emails[i], "@")
		local, domain := email[0], email[1]
		local = strings.ReplaceAll(local, ".", "")
		local = regExp.ReplaceAllString(local, "")
		emailMap[local+"@"+domain] = true
	}
	return len(emailMap)
}
