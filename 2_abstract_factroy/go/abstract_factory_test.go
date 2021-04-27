package abstract_factory

import (
	"fmt"
	"testing"
)

func TestFactory(t *testing.T) {
	a := NewAmd()
	x, y := a.Build()
	x.Run()
	y.Run()
	fmt.Println(a.Get_number())
}
