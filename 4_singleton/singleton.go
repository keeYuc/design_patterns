package singleton

import "sync"

type mgr struct {
	service map[string]int
	numbers int
	mutx    *sync.RWMutex
}

var (
	m    *mgr
	once sync.Once
)

func init() {
	once.Do(
		func() {
			m = &mgr{
				service: make(map[string]int, 10),
				numbers: 0,
				mutx:    new(sync.RWMutex),
			}
		})
}

func (m *mgr) AddService(name string, nums int) {
	m.mutx.Lock()
	defer m.mutx.Unlock()
	n, ok := m.service[name]
	if ok && n == nums {
		return
	} else {
		m.service[name] = nums
		m.numbers -= n
		m.numbers += nums
	}
}

func (m *mgr) GetNumbers() int {
	m.mutx.RLocker().Lock()
	defer m.mutx.RLocker().Unlock()
	return m.numbers
}
