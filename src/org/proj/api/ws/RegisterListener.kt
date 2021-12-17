package org.proj.api.ws

import java.io.BufferedReader
import java.io.InputStreamReader
import java.net.ServerSocket
import java.net.Socket

class RegisterListener (var server: ServerSocket): Thread() {
    constructor () : this (ServerSocket(1234))

    override fun run () {
        while (true) {
            var socket = server.accept();
            read(socket)
        }
    }

    fun read (socket: Socket) {
        println("Connected with "+socket.inetAddress)
        var output = socket.getOutputStream();
        output.write("hello world".toByteArray());

        var input = socket.getInputStream()
        var reader = BufferedReader(InputStreamReader(input))
        println(reader.readText())
    }
}

fun main () {
    var listener = RegisterListener()
    listener.run()
}