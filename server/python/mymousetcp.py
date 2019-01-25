import mouse as m
import json
from SimpleWebSocketServer import SimpleWebSocketServer, WebSocket

class MyTCPHandler(WebSocket):
    def handleMessage(self):
        data = self.data
        print data
        if "{" in data:
            info = json.loads(data)
            vx = info['vx']
            vy = info['vy']
            m.move(vx,vy,absolute=False,duration=0)
        print "\n"

    def handleConnected(self):
        print(self.address, 'connected')

    def handleClose(self):
        print(self.address, 'closed')

if __name__ == "__main__":
    HOST, PORT = "10.6.85.68", 7000
    server = SimpleWebSocketServer(HOST, PORT, MyTCPHandler)
    server.serveforever()
    
    