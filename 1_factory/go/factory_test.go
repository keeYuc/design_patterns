package factory

import "testing"

func TestFactory(t *testing.T) {
	a := get_machine(1)
	b := get_machine(2)
	a.Run()
	b.Run()
}
