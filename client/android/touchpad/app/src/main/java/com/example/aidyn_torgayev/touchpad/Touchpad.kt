package com.example.aidyn_torgayev.touchpad

import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import android.view.MotionEvent
import android.view.VelocityTracker
import kotlinx.android.synthetic.main.activity_touchpad.*
import java.net.URI


class Touchpad : AppCompatActivity() {
    private val velocity: VelocityTracker? = VelocityTracker.obtain()
    private var s:WebSocketSender? = null
    private var sensetivity = 10
    private var vx:Float? = 0.0f
    private var vy:Float?= 0.0f
    private var x:Float? = 0.0f
    private var y:Float?= 0.0f

    override fun onCreate(savedInstanceState: Bundle?) {
        val data = intent.extras["data"] as Bundle
        val ip = data["ip"]
        val port = data["port"]
        sensetivity = data.getInt("sensitivity")
        s =  WebSocketSender(URI("ws://%s:%s".format(ip,port)))
        s?.connect()
        setContentView(R.layout.activity_touchpad)
        super.onCreate(savedInstanceState)
    }

    override fun onTouchEvent(e: MotionEvent): Boolean {
        when (e.action) {
            MotionEvent.ACTION_MOVE -> {
                velocity?.addMovement(e)
                velocity?.computeCurrentVelocity(sensetivity)
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
        s?.sendMouse(vx, vy)
        return super.onGenericMotionEvent(e)
    }

    override fun onGenericMotionEvent(event: MotionEvent?): Boolean {
        return super.onGenericMotionEvent(event)
    }


    override fun onDestroy() {
        s?.close()
        super.onDestroy()
    }

}