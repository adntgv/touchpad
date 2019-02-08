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
    val queue:BlockingQueue<MotionEvent> = LinkedBlockingQueue<MotionEvent>()
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
        queue.offer(e)
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
