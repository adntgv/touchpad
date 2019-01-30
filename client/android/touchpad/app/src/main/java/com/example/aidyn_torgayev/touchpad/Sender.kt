package com.example.aidyn_torgayev.touchpad

import org.java_websocket.client.WebSocketClient
import org.java_websocket.handshake.ServerHandshake
import org.json.JSONObject
import java.lang.Exception
import java.net.URI
import java.util.*
import java.util.concurrent.BlockingDeque
import java.util.concurrent.BlockingQueue
import java.util.concurrent.LinkedBlockingQueue
import android.R.attr.port
import android.os.Handler
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress


class WebSocketSender(serverUri: URI?) : WebSocketClient(serverUri) {

    override fun onError(ex: Exception?) {
        println(ex)
    }

    override fun onMessage(message: String?) {
        println("Recieved: $message")
    }

    override fun onClose(code: Int, reason: String?, remote: Boolean) {
        println("Closed because $reason Code: $code by remote: $remote")
    }

    override fun onOpen(handshakedata: ServerHandshake?) {
        send("Hello")
        println("Connection is established")
    }

    fun sendMouse(vararg params: Float?) {
        try {
            send(Mouse(params[0]!!,params[1]!!).toString())
        } catch (e : Throwable){
            throw e
        }
    }

    fun tap(){
        send("tap")
    }
}

class Mouse (private val vx:Float, private val vy: Float) {
    private val data = JSONObject()
    override fun toString(): String {
        data.put("vx", vx)
        data.put("vy", vy)
        return data.toString()
    }
}

class UDPSender (private val ip: String, private val port:Int, private val h: Handler) : Runnable {
    private val queue:Queue<String> = LinkedBlockingQueue<String>()
    private var run:Boolean = true

    override fun run() {
        val serverAddr = InetAddress.getByName(ip)
        val udpSocket = DatagramSocket()
        var buf = "The String to Send".toByteArray()
        var packet = DatagramPacket(buf, buf.size, serverAddr, port)
        udpSocket.send(packet)
        while (run) {
            try {
                val data = queue.poll()
                if (data != null) {
                    buf = data.toByteArray()
                    packet = DatagramPacket(buf, buf.size, serverAddr, port)
                    udpSocket.send(packet)
                }
            } catch (e: Exception) {
                println(e)
            }
        }
    }

    fun sendMouse(vararg params: Float?) {
        send(Mouse(params[0]!!,params[1]!!).toString())
    }

    fun send(st: String){
        queue.offer(st)
    }

}
