package com.example.aidyn_torgayev.touchpad

import android.app.Service
import android.content.Intent
import android.os.*
import android.view.MotionEvent
import android.view.VelocityTracker
import org.json.JSONObject
import java.lang.Exception
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress
import java.util.concurrent.BlockingQueue
import java.util.concurrent.LinkedBlockingQueue

class UDPSender : Service() {
    val queue:BlockingQueue<JSONObject> = LinkedBlockingQueue<JSONObject>()
    private val mBinder = UDPSenderBinder()
    private lateinit var ip: InetAddress
    private var port:Int = 0

    private val velocity: VelocityTracker? = VelocityTracker.obtain()
    private var x: Float = 0.0f
    private var y: Float = 0.0f
    private var vx: Float? = 0.0f
    private var vy: Float? = 0.0f
    private var sensitivity = 0

    inner class UDPSenderBinder : Binder() {
        fun getService() :  UDPSender {
            return this@UDPSender
        }
    }

    override fun onBind(intent: Intent): IBinder? {
        val data = intent.getBundleExtra("data")
        ip = InetAddress.getByName(data.getString("ip"))
        port = data.getInt("port")
        sensitivity = data.getInt("sensitivity")
        Thread(localSender).start()
        return mBinder
    }

    fun send(e:MotionEvent){
        when (e.action) {
            MotionEvent.ACTION_MOVE -> {
                velocity?.addMovement(e)
                velocity?.computeCurrentVelocity(sensitivity)
                vx = velocity?.xVelocity
                vy = velocity?.yVelocity
                move()
            }
            MotionEvent.ACTION_DOWN -> {
                x = e.x
                y = e.y
            }
            MotionEvent.ACTION_UP -> {
                if (e.x == x && e.y == y) {
                    tap()
                }
            }
        }
    }

    private fun tap(){
        queue.offer(Mouse("tap",vx,vy).toJSON())
    }

    private fun move(){
        if (vy != 0.0f && vx != 0.0f) {
            queue.offer(Mouse("move",vx,vy).toJSON())
        }
    }

    private fun doubleTap(){
        TODO()
    }

    private fun drag(){
        TODO()
    }

    private fun scroll(){
        TODO()
    }

    private val localSender: Runnable = object : Runnable {
        private val clientSocket = DatagramSocket()
        private var running = true
        private lateinit var sendData:ByteArray
        private lateinit var sendPacket: DatagramPacket
        override fun run() {
            clientSocket.broadcast = true;
            while (running) {
                val data = queue.poll()
                if (data != null) {
                    sendData = data.toString().toByteArray()
                    sendPacket = DatagramPacket(sendData,sendData.size,ip,port)
                    try {
                        clientSocket.send(sendPacket)
                    } catch (e : Exception) {
                        println(e)
                    }
                }
            }
        }
    }
}

class Mouse (private val action: String, private val vx:Float?, private val vy: Float?) {
    private val data = JSONObject()
    fun toJSON(): JSONObject{
        data.put("action", action)
        data.put("vx", vx)
        data.put("vy", vy)
        return data
    }
    override fun toString(): String {
        return toJSON().toString()
    }
}