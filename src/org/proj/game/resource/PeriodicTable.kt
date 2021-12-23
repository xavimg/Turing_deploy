package org.proj.game.resource

import org.rol.ReadOnlyList
import org.sjr.JSONObj
import java.io.*
import java.lang.Exception
import kotlin.system.exitProcess

class PTElement (
    val number: Int,
    val name: String,
    val mass: Float
) {
    companion object {
        const val URL = "https://raw.githubusercontent.com/Bowserinator/Periodic-Table-JSON/master/PeriodicTableJSON.json"

        @JvmStatic
        val ELEMENTS : ReadOnlyList<PTElement> = initElements()

        private fun getReader () : InputStreamReader {
            val url = java.net.URL(URL)
            return InputStreamReader(url.openStream())
        }

        private fun initElements () : ReadOnlyList<PTElement> {
            val json = JSONObj(getReader())
            val elements = json.getObjectArray("elements").get()
            val map = elements.map { x -> initElement(x) }
            return ReadOnlyList.ofList(map)
        }

        private fun initElement (obj: JSONObj) : PTElement {
            val number = obj.getInt("number").get()
            val name = obj.getString("name").get()
            val mass = obj.getFloat("atomic_mass").get()

            return PTElement(number, name, mass)
        }
    }
}