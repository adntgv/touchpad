package com.example.aidyn_torgayev.touchpad

import android.annotation.SuppressLint
import android.os.AsyncTask
import android.os.Handler
import android.os.Looper
import android.os.Message
import org.json.JSONObject
import java.io.DataOutputStream
import java.net.Socket
import java.util.*
import java.util.concurrent.Semaphore

/*
* TCP Sender Simple
* */
class TCPSender: AsyncTask<Float,Void,Void>() {
    var s: Socket? = null
    var d: DataOutputStream? = null

    override fun doInBackground(vararg params: Float?): Void? {
        try {
            s = Socket("10.6.85.68",7000)
            d = DataOutputStream(s?.getOutputStream())
        } catch(e: Throwable){
            println(e)
        }
        val data = JSONObject()
        data.put("vx", params[0])
        data.put("vy", params[1])
        d?.writeUTF(data.toString())
        s?.close()
        return null
    }
}
