# -*- coding: utf-8 -*-

import socket
import json
import mouse as m

 

localIP     = "10.6.85.68"

localPort   = 7000

bufferSize  = 1024

 

msgFromServer       = "Hello UDP Client"

bytesToSend         = str.encode(msgFromServer)

 

# Create a datagram socket

UDPServerSocket = socket.socket(family=socket.AF_INET, type=socket.SOCK_DGRAM)

 

# Bind to address and ip

UDPServerSocket.bind((localIP, localPort))
UDPServerSocket.settimeout(0.01)
 

print("UDP server up and listening")

 

# Listen for incoming datagrams

pressing = False
while(True):
    try:
        bytesAddressPair = UDPServerSocket.recvfrom(bufferSize)
    except socket.error:
        continue
    message = bytesAddressPair[0].decode("utf-8")
    print(message)
    touchpad = json.loads(message)
    if touchpad["action"] == "move":
        cx, cy = m.get_position()
        vx = touchpad["vx"]
        vy = touchpad["vy"]
        if not pressing:
            m.move( cx +vx, cy + vy)
    if touchpad["action"] == "tap":
        pressing = True
        #m.press()
        #m.release()
        pressing = False
    # Sending a reply to client

    #UDPServerSocket.sendto(bytesToSend, address)