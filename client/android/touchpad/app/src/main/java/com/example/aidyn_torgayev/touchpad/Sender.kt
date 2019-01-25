package com.example.aidyn_torgayev.touchpad

import android.app.IntentService
import android.app.Service
import android.content.Intent
import android.os.*
import org.java_websocket.client.WebSocketClient
import org.java_websocket.handshake.ServerHandshake
import org.json.JSONObject
import java.io.DataOutputStream
import java.lang.Exception
import java.net.Socket
import java.net.URI
import java.util.concurrent.BlockingDeque
import java.util.concurrent.BlockingQueue
import java.util.concurrent.LinkedBlockingQueue

class WebSocketSender(serverUri: URI?) : WebSocketClient(serverUri) {

    override fun onError(ex: Exception?) {
        println(ex)
    }

    override fun onMessage(message: String?) {
        println("Recieved: " + message)
    }

    override fun onClose(code: Int, reason: String?, remote: Boolean) {
        println("Closed because " + reason + " Code: " + code.toString() + " by remote: "  + remote )
    }

    override fun onOpen(handshakedata: ServerHandshake?) {
        send("Hello")
        println("Connection is established")
    }

    fun sendMouse(vararg params: Float) {
        send(Mouse(params[0],params[1]).toString())
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