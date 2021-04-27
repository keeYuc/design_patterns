package builder

import "testing"

func TestBuild(t *testing.T) {
	x := NewDirctor(new(amd))
	x.do_someting1().do_someting3().do_someting2().do_someting3()
}
