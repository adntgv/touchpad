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
import android.hardware.SensorManager
import android.os.IBinder
import kotlin.math.log

class TouchpadActivity : AppCompatActivity() {
    var mBound: Boolean = false
    var s: UDPSender? = null
    private lateinit var sensorManager: SensorManager
    private lateinit var sensorWatcher: SensorWatcher

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

        sensorManager = getSystemService(Context.SENSOR_SERVICE) as SensorManager
        val all_sensors = sensorManager.getSensorList(Sensor.TYPE_ALL)
        var s = ""
        for (sens in all_sensors){
            s += sens.name + "\n"
        }
        log(s)

        sensorWatcher = SensorWatcher()
        sensorWatcher.start()
        super.onCreate(savedInstanceState)
    }

    override fun onTouchEvent(e: MotionEvent): Boolean {
        //log(e.toString())
        try {
            s?.send(e)
        } catch (t: Throwable) {
            println(t)
        }
        return super.onGenericMotionEvent(e)
    }

    private fun log(str: String){
        text_field.text = str
    }

    override fun onDestroy() {
        if (mBound) {
            unbindService(mConnection)
            mBound = false
        }
        val intent = Intent(this, UDPSender::class.java)
        stopService(intent)
        super.onDestroy()
    }

    override fun onResume() {
        super.onResume()
        sensorWatcher.start()
    }

    override fun onPause() {
        super.onPause()
        sensorWatcher.stop()
    }

    inner class SensorWatcher : SensorEventListener {
        private var accelerometer: Sensor = sensorManager.getDefaultSensor(Sensor.TYPE_ACCELEROMETER)

        public fun start() {
            sensorManager.registerListener(this, accelerometer, SensorManager.SENSOR_DELAY_NORMAL);
        }

        public fun stop() {
            sensorManager.unregisterListener(this);
        }

        override fun onAccuracyChanged(sensor: Sensor?, accuracy: Int) {
            println(sensor)
        }

        override fun onSensorChanged(event: SensorEvent?) {
            if (event?.sensor?.type != Sensor.TYPE_ACCELEROMETER){
                return
            }
            for (v in event.values) {
                println(v)
            }
            print("\n")
            s?.send(event)
        }
    }
}