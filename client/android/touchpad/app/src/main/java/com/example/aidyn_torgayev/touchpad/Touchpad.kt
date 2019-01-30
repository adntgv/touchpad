package com.example.aidyn_torgayev.touchpad

import android.content.Context
import android.hardware.Sensor
import android.hardware.SensorEvent
import android.hardware.SensorEventListener
import android.hardware.SensorManager
import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import android.view.MotionEvent
import android.view.VelocityTracker
import android.widget.Toast
import kotlinx.android.synthetic.main.activity_touchpad.*
import java.net.URI
import java.time.Duration
import java.util.concurrent.TimeUnit


class Touchpad : AppCompatActivity(), SensorEventListener {

    private lateinit var mSensorManager: SensorManager
    private var mAxl: Sensor? = null

    private val velocity: VelocityTracker? = VelocityTracker.obtain()
    private var st : Thread? = null
    private var s:WebSocketSender? = null
    private var sensitivity = 10
    private var vx:Float? = 0.0f
    private var vy:Float?= 0.0f
    private var x:Float? = 0.0f
    private var y:Float?= 0.0f
    private var tElapsed:Duration? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        mSensorManager = getSystemService(Context.SENSOR_SERVICE) as SensorManager
        mAxl = mSensorManager.getDefaultSensor(Sensor.TYPE_ALL)

        val data = intent.extras["data"] as Bundle
        val ip = data.getString("ip")
        val port = data.getString("port")
        sensitivity = data.getInt("sensitivity")
        s =  WebSocketSender(URI("ws://%s:%s".format(ip,port)))
        s?.connectBlocking(100L, TimeUnit.MILLISECONDS)
        if (!s?.isOpen!!) {
            connFail()
        }
        st?.start()

        setContentView(R.layout.activity_touchpad)
        super.onCreate(savedInstanceState)
    }

    override fun onTouchEvent(e: MotionEvent): Boolean {
        log(e.toString())
        when (e.action) {
            MotionEvent.ACTION_MOVE -> {
                velocity?.addMovement(e)
                velocity?.computeCurrentVelocity(sensitivity)
                vx = velocity?.xVelocity
                vy = velocity?.yVelocity
            }
            MotionEvent.ACTION_DOWN -> {
                x = e.x
                y = e.y

            }
            MotionEvent.ACTION_UP -> {
                if (e.x == x && e.y == y) {
                    s?.tap()
                }
                vx = 0.0f
                vy = 0.0f
            }
        }
        try {
            s?.sendMouse(vx, vy)
        } catch (e : Throwable) {
            connFail()
        }
        return super.onGenericMotionEvent(e)
    }

    override fun onSensorChanged(event: SensorEvent?) {
        log(event.toString())
    }

    override fun onAccuracyChanged(sensor: Sensor?, accuracy: Int) {
        log(sensor.toString())
    }

    override fun onDestroy() {
        s?.close()
        super.onDestroy()
    }

    private fun log(str: String){
        //val prev = text_field.text
        text_field.text = str
    }

    private fun connFail(){
        val text = "Could not connect to server"
        val toast = Toast.makeText(applicationContext, text, Toast.LENGTH_SHORT)
        toast.show()
        s?.close()
        this.finish()
    }
}