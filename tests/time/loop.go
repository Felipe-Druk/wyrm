package main

import "fmt"

func main() {
	var i int64 = 0
	var acc int64 = 0
	for i < 1000000 {
		acc = (acc + i*3) % 1000007
		i = i + 1
	}
	fmt.Println(acc)
}
