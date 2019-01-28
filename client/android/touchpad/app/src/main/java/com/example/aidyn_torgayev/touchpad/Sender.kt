package com.example.aidyn_torgayev.touchpad

import org.java_websocket.client.WebSocketClient
import org.java_websocket.handshake.ServerHandshake
import org.json.JSONObject
import java.lang.Exception
import java.net.URI

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
        send(Mouse(params[0]!!,params[1]!!).toString())
    }

    fun tap(){
        send("tap")
    }
}

class Mouse (private val vx:Float,private val vy: Float) {
    private val data = JSONObject()
    override fun toString(): String {
        data.put("vx", vx)
        data.put("vy", vy)
        return data.toString()
    }
}