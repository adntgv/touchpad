#!/usr/bin/env python
# -*- coding: utf-8 -*-

import socket
import json
import MouseEvent

localIP     = "10.6.85.68"
localPort   = 7000
bufferSize  = 1024

UDPServerSocket = socket.socket(family=socket.AF_INET, type=socket.SOCK_DGRAM)
UDPServerSocket.bind((localIP, localPort))
UDPServerSocket.settimeout(0.01)
 
print("UDP server up and listening")

MouseEvent.DEBUG = True
me = MouseEvent.MouseEvent()

while(True):
    try:
        bytesAddressPair = UDPServerSocket.recvfrom(bufferSize)
    except socket.error:
        continue
    message = bytesAddressPair[0].decode("utf-8")
    me.act(str(message))