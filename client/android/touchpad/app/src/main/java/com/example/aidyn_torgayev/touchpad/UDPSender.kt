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
    val queue:BlockingQueue<TouchpadOuterClass.Touchpad> = LinkedBlockingQueue<TouchpadOuterClass.Touchpad>()
    private val mBinder = UDPSenderBinder()
    private lateinit var ip: InetAddress
    private var threadRunning = true
    private var port:Int = 0

    inner class UDPSenderBinder : Binder() {
        fun getService() :  UDPSender {
            return this@UDPSender
        }
    }

    override fun onBind(intent: Intent): IBinder? {
        val data = intent.getBundleExtra("data")
        ip = InetAddress.getByName(data.getString("ip"))
        port = data.getInt("port")
        Thread(localSender).start()
        return mBinder
    }

    override fun onUnbind(intent: Intent?): Boolean {
        onDestroy()
        return super.onUnbind(intent)
    }

    override fun onDestroy() {
        threadRunning = false
        super.onDestroy()
    }

    fun send(e:MotionEvent){
        val touch: TouchpadOuterClass.Touchpad = convertToTouchpad(e)
        queue.offer(touch)
    }

    private fun convertToTouchpad(e: MotionEvent): TouchpadOuterClass.Touchpad {
        val touchBuilder =  TouchpadOuterClass.Touchpad.newBuilder()
        touchBuilder.action = getAction(e)
        touchBuilder.eventTime = e.eventTime
        touchBuilder.downTime = e.downTime
        if (e.pointerCount > 1) {
            val positions: Array<Array<Double>> = getPositions(e)
            touchBuilder.addAllX(positions[0].toMutableList())
            touchBuilder.addAllY(positions[1].toMutableList())
        } else {
            touchBuilder.addX(e.x.toDouble())
            touchBuilder.addY(e.y.toDouble())
        }
        return touchBuilder.build()
    }

    private fun getPositions(e: MotionEvent): Array<Array<Double>> {
        val count = e.pointerCount
        val ps: Array<Array<Double>> = Array(2) { Array(count) {0.0} }
        for (i in 0..count) {
            ps[0][i] = e.getX(i).toDouble()
            ps[1][i] = e.getY(i).toDouble()
        }
        return ps
    }

    private fun getAction(e:MotionEvent): TouchpadOuterClass.Action {
        var action = TouchpadOuterClass.Action.NONE
        when (e.action) {
            MotionEvent.ACTION_DOWN -> action = TouchpadOuterClass.Action.DOWN
            MotionEvent.ACTION_UP -> action = TouchpadOuterClass.Action.UP
            MotionEvent.ACTION_MOVE -> action = TouchpadOuterClass.Action.MOVE
        }
        return action
    }

    private val localSender: Runnable = object : Runnable {
        private val clientSocket = DatagramSocket()
        private var sendData:ByteArray = "".toByteArray()
        private var sendPacket: DatagramPacket = DatagramPacket(sendData, sendData.size)
        override fun run() {
            clientSocket.broadcast = true;
            while (threadRunning) {
                val data = queue.poll()
                if (data != null) {
                    sendData = data.toByteArray()
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
