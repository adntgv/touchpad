package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"net/http"
	"strings"

	"github.com/go-vgo/robotgo"
	"github.com/gorilla/websocket"
)

type Mouse struct {
	Vx float64 `json:"vx"`
	Vy float64 `json:"vy"`
}

func (m Mouse) String() string {
	return fmt.Sprintf("vx:%v vy:%v", m.Vx, m.Vy)
}

var addr = flag.String("addr", "10.6.85.68:7000", "http service address")

var upgrader = websocket.Upgrader{} // use default options

func echo(w http.ResponseWriter, r *http.Request) {
	c, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Print("upgrade:", err)
		return
	}
	defer c.Close()
	for {
		_, message, err := c.ReadMessage()
		if err != nil {
			log.Println("read:", err)
			break
		}
		//log.Printf("recv: %s", message)
		if strings.Contains(string(message), "{") {
			m := Mouse{}
			err = json.Unmarshal(message, &m)
			if err != nil {
				log.Print("json.unmarshal:", err)
				return
			}
			fmt.Println(m)
			x, y := robotgo.GetMousePos()
			robotgo.Move(x+int(m.Vx), y+int(m.Vy))
		}
		if strings.EqualFold(string(message), "tap") {
			robotgo.MouseClick()
		}
	}
}

func getDirections(m string) Mouse {
	ms := strings.Split(m, ",")
	fmt.Println(ms)
	return Mouse{}
}

func main() {
	flag.Parse()
	log.SetFlags(0)
	http.HandleFunc("/", echo)
	log.Fatal(http.ListenAndServe(*addr, nil))
}
