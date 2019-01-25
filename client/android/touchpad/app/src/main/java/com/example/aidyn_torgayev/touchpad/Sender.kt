package com.example.aidyn_torgayev.touchpad

import android.app.IntentService
import android.app.Service
import android.content.Intent
import android.os.*
import org.json.JSONObject
import java.io.DataOutputStream
import java.net.Socket
import java.util.concurrent.BlockingDeque
import java.util.concurrent.BlockingQueue
import java.util.concurrent.LinkedBlockingQueue

class TCPSenderThread: Thread() {
    private var s: Socket? = null
    private var d: DataOutputStream? = null
    val blockingQueue:BlockingQueue<Mouse> = LinkedBlockingQueue()

    override fun run() {
        connect()
        while (true) {
            val m = blockingQueue.poll()
            if (m != null) {
                if (s == null) {
                    connect()
                }
                d?.writeUTF(m.toString())
            }
        }
    }

    private fun connect(){
        try {
            s = Socket("10.6.85.68",7000)
            d = DataOutputStream(s?.getOutputStream())
            d?.writeUTF("Connected!")
        } catch(e: Throwable){
            println(e)
        }
    }
}

class Mouse (private val vx:Float,private val vy: Float) {
    private val data = JSONObject()
    override fun toString(): String {
        data.put("vx",vx)
        data.put("vy",vy)
        return data.toString()
    }
}
