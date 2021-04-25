package prototype

import "testing"

func TestProtoType(t *testing.T) {
	var n Animal
	n = &human{"张三", 10}
	n.GetName()
	n.GetAge()
}
