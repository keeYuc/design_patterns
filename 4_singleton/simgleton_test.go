package singleton

import (
	"fmt"
	"testing"
)

func TestSingleton(t *testing.T) {
	fmt.Println(m.GetNumbers())
	m.AddService("abc", 3)
	fmt.Println(m.GetNumbers())
	m.AddService("ddd", 5)
	fmt.Println(m.GetNumbers())
}
