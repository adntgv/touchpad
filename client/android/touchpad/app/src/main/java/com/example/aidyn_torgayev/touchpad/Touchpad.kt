package com.example.aidyn_torgayev.touchpad

import android.content.ComponentName
import android.content.Context
import android.hardware.Sensor
import android.hardware.SensorEvent
import android.hardware.SensorEventListener
import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import android.view.MotionEvent
import android.view.VelocityTracker
import android.widget.Toast
import kotlinx.android.synthetic.main.activity_touchpad.*
import android.content.Intent
import android.content.ServiceConnection
import android.os.IBinder

class Touchpad : AppCompatActivity() {
    var mBound: Boolean = false
    var s: UDPSender? = null
    private var mConnection:ServiceConnection = object :ServiceConnection {

        override fun onServiceConnected(name: ComponentName?, service: IBinder?) {
            val binder =   service as UDPSender.UDPSenderBinder
            s = binder.getService()
            mBound = true
        }

        override fun onServiceDisconnected(name: ComponentName?) {
            mBound = false
        }
    }


    override fun onCreate(savedInstanceState: Bundle?) {

        val data = intent.getBundleExtra("data")

        val intent = Intent(this, UDPSender::class.java).apply {
            putExtra("data", data)
        }
        bindService(intent, mConnection, Context.BIND_AUTO_CREATE)

        setContentView(R.layout.activity_touchpad)
        super.onCreate(savedInstanceState)
    }

    override fun onTouchEvent(e: MotionEvent): Boolean {
        log(e.toString())
        try {
            s?.send(e)
        } catch (t: Throwable) {
            println(t)
        }
        return super.onGenericMotionEvent(e)
    }


    private fun log(str: String){
        //val prev = text_field.text
        text_field.text = str
    }

}