package decorator

import "fmt"

//变形金刚
type transfarm interface {
	Start()
}

//车
type car struct{}

func (c *car) Start() {
	fmt.Println("car start")
}

//车 => 飞机  => 船

type drone struct {
	t transfarm
}

func NewDrone(t transfarm) transfarm {
	return &drone{t}
}

func (d *drone) Start() {
	d.t.Start()
	fmt.Println("变成无人机了")
}

type ship struct {
	t transfarm
}

func NewShip(t transfarm) transfarm {
	return &ship{t}
}

func (d *ship) Start() {
	d.t.Start()
	fmt.Println("变成船了")
}
