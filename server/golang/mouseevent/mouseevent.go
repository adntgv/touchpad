package mouseevent

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/go-vgo/robotgo"
)

var (
	LongTapTimeout int64 = 500
	DragTimeout    int64 = 500
	Debug                = false
)

type MouseEvent struct {
	paction   string
	action    string
	dx        float64
	px        float64
	x         float64
	dy        float64
	py        float64
	y         float64
	cx        int
	cy        int
	eventTime int64
	downTime  int64
	Mul       float64
	pressed   bool
}

func (m *MouseEvent) Handle(message string) {
	var err error
	fmt.Println(message)
	if strings.Contains(message, "MotionEvent") {
		message = strings.Replace(message, "MotionEvent", "", -1)
		message = strings.Replace(message, " ", "", -1)
		message = strings.Trim(message, "{} ")
		fields := strings.Split(message, ",")

		for _, field := range fields {
			if strings.Contains(field, "action") {
				m.paction = m.action
				m.action = strings.Split(field, "=")[1]
			} else if strings.Contains(field, "x[0]") {
				m.px = m.x
				m.x, err = strconv.ParseFloat(strings.Split(field, "=")[1], 64)
				if err != nil {
					panic(err)
				}
			} else if strings.Contains(field, "y[0]") {
				m.py = m.y
				m.y, err = strconv.ParseFloat(strings.Split(field, "=")[1], 64)
				if err != nil {
					panic(err)
				}
			} else if strings.Contains(field, "eventTime") {
				m.eventTime, err = strconv.ParseInt(strings.Split(field, "=")[1], 10, 32)
				if err != nil {
					panic(err)
				}
			} else if strings.Contains(field, "downTime") {
				m.downTime, err = strconv.ParseInt(strings.Split(field, "=")[1], 10, 32)
				if err != nil {
					panic(err)
				}
			}

		}
		m.do()
	}
}

func (m *MouseEvent) do() {
	m.dx = m.x - m.px
	m.dy = m.y - m.py
	m.cx, m.cy = robotgo.GetMousePos()
	if m.paction == "ACTION_DOWN" {
		if m.action == "ACTION_UP" {
			if m.pressed {
				robotgo.MouseToggle("up")
				m.pressed = false
				return
			}
			if m.eventTime-m.downTime < LongTapTimeout {
				debug("clicking")
				robotgo.Click()
			} else {
				robotgo.Click("right")
			}
			return
		}
		if m.action == "ACTION_MOVE" {
			if m.eventTime-m.downTime > DragTimeout {
				if !m.pressed {
					robotgo.MouseToggle("down")
					m.pressed = true
				}
			}
			robotgo.Move(m.cx+int(m.dx), m.cy+int(m.dy))
			return
		}
		return
	}
	if m.paction == "ACTION_MOVE" {
		if m.action == "ACTION_MOVE" {
			robotgo.Move(m.cx+int(m.Mul*m.dx), m.cy+int(m.Mul*m.dy))
			return
		}
		return
	}
	return
}

func debug(msg string) {
	if Debug {
		fmt.Println(msg)
	}
}
