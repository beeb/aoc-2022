package main

import (
	"aoc-2022/utils"

	log "github.com/sirupsen/logrus"
)

func init() {
	log.SetFormatter(&log.TextFormatter{FullTimestamp: true})
}

func main() {
	err := utils.WriteDayInput(1)
	if err != nil {
		log.Fatal(err)
	}
}
