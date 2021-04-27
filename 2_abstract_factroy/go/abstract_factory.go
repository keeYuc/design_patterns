package abstract_factory

import "fmt"

type factorys interface {
	Build() (goods, goods)
	Get_number() int
}

type goods interface {
	Run()
}

type Cpu struct {
	name string
}
type Gpu struct {
	name string
}
type Amd struct {
	number int
}
type Inter struct{}

func (c *Cpu) Run() {
	fmt.Println(c.name, ":", "cpu run")
}

func (c *Gpu) Run() {
	fmt.Println(c.name, ":", "gpu run")
}

func NewAmd() factorys {
	return &Amd{0}
}

func (a *Amd) Build() (goods, goods) {
	a.number++
	return &Cpu{"amd"}, &Gpu{"amd"}
}

func (a *Amd) Get_number() int {
	return a.number
}
