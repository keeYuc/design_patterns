package prototype

type Animal interface {
	GetName() string
	GetAge() int
}

type human struct {
	name string
	age  int
}

func (h *human) GetName() string {
	return h.name
}

func (h *human) GetAge() int {
	return h.age
}
