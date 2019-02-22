package com.example.aidyn_torgayev.touchpad

import android.content.Intent
import android.support.v7.app.AppCompatActivity
import android.os.Bundle
import android.widget.Button
import kotlinx.android.synthetic.main.activity_configure.*
import java.net.NetworkInterface

class Configure : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_configure)

        val button = findViewById<Button>(R.id.btn_connect)
        button.setOnClickListener {
            val bundle = Bundle()
            bundle.putString("ip", val_ip.text.toString())
            bundle.putInt("port", val_port.text.toString().toInt())

            val intent = Intent(this@Configure, TouchpadActivity::class.java).apply {
                putExtra("data", bundle)
            }
            startActivity(intent)
        }
    }

    fun findServer()  {
        val enumNetworkInterfaces =  NetworkInterface.getNetworkInterfaces();
        for (networkInterface in enumNetworkInterfaces) {
            val enumInetAddress = networkInterface.inetAddresses;
            for (inetAddress in enumInetAddress) {
                val ipAddress = inetAddress.hostAddress;
            }
        }
    }


}
