import mouse as m
import json
import SocketServer

class MyTCPHandler(SocketServer.BaseRequestHandler):
    def handle(self):
        data = self.request.recv(1024).strip()
        print data
        #info = json.loads(self.data[2:])
        #vx = info['vx']
        #vy = info['vy']
        #m.move(vx,vy,absolute=False)
        print "\n"
        
        #m.move(x,y, absolute=False, duration=0)

if __name__ == "__main__":
    HOST, PORT = "10.6.85.68", 7000

    server = SocketServer.TCPServer((HOST, PORT), MyTCPHandler)
    server.serve_forever()
    
    