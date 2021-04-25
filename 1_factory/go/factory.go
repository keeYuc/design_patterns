package factory

import "fmt"

type machine interface {
	Run()
}

type car struct{}
type drone struct{}

func (c *car) Run() {
	fmt.Println("car run")
}

func (c *drone) Run() {
	fmt.Println("drone run")
}

func get_machine(n int) machine {
	switch n {
	case 1:
		return new(car)
	default:
		return new(drone)
	}
}
