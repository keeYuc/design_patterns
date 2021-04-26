package decorator

import "testing"

func TestDecorator(t *testing.T) {
	qingtianzhu := car{}

	feiji := NewDrone(&qingtianzhu)
	feiji.Start()

	chuan := NewShip(&qingtianzhu)
	chuan.Start()
}
