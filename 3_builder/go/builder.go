package builder

import "fmt"

type Builder interface {
	Build(string)
	Get()
}

type Director struct {
	builder Builder
}

type Product struct {
	name string
}

func (d *Director) do_someting1() *Director {
	d.builder.Build("amd yes")
	return d
}

func (d *Director) do_someting2() *Director {
	d.builder.Build("amd yes yes")
	return d
}

func (d *Director) do_someting3() *Director {
	d.builder.Get()
	return d
}

func NewDirctor(b Builder) Director {
	return Director{b}
}

type amd struct {
	Product
}

func (a *amd) Build(s string) {
	a.name = s
}

func (a *amd) Get() {
	fmt.Println(a.name)
}
