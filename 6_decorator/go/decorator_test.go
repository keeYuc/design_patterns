package decorator

import (
	"testing"
)

func TestDecorator(t *testing.T) {
	qingtianzhu := car{}

	qingtianzhu.Start()

	NewDrone(&qingtianzhu).Start()

	NewShip(&qingtianzhu).Start()

}
