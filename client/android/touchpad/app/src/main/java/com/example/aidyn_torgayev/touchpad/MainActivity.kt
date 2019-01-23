package com.example.aidyn_torgayev.touchpad

import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import android.view.MotionEvent
import android.view.VelocityTracker
import kotlinx.android.synthetic.main.activity_main.*


class MainActivity : AppCompatActivity() {

    private val velocity: VelocityTracker? = VelocityTracker.obtain()
    override fun onCreate(savedInstanceState: Bundle?) {
        setContentView(R.layout.activity_main)
        super.onCreate(savedInstanceState)
    }

    override fun onTouchEvent(e: MotionEvent): Boolean {
        when (e.action) {
            MotionEvent.ACTION_MOVE -> {
                velocity?.addMovement(e);
                velocity?.computeCurrentVelocity(50)
                vx.text = velocity?.xVelocity.toString()
                vy.text = velocity?.yVelocity.toString()
            }
            MotionEvent.ACTION_UP -> {
                vx.text = "0"
                vy.text = "0"
            }
        }
        TCPSender().execute(vx.text.toString().toFloat(), vy.text.toString().toFloat())
        return super.onGenericMotionEvent(e)
    }

}