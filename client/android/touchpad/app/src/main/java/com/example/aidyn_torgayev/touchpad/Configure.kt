package com.example.aidyn_torgayev.touchpad

import android.content.Intent
import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import android.view.View
import android.widget.Button
import kotlinx.android.synthetic.main.activity_configure.*

class Configure : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_configure)

        val button = findViewById<Button>(R.id.btn_connect)
        button.setOnClickListener(object: View.OnClickListener {
            override fun onClick(v: View?) {
                val bundle = Bundle()
                bundle.putInt("sensitivity", val_sensitivity.text.toString().toInt())
                bundle.putString("ip", val_ip.text.toString())
                bundle.putString("port", val_port.text.toString())

                val intent = Intent(this@Configure, Touchpad::class.java).apply {
                    putExtra("data", bundle)
                }
                startActivity(intent)
            }
        })
    }
}
