package decorator

import "testing"

func TestDecorator(t *testing.T) {
	qingtianzhu := car{}

	NewDrone(&qingtianzhu).Start()

	NewShip(&qingtianzhu).Start()
}
