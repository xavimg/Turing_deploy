package org.proj.game.resource

import org.rol.ReadOnlyList
import org.sjr.JSONObj
import java.io.*
import java.lang.Exception
import kotlin.system.exitProcess

class PTElement (private val _name: Lazy<String>) {
    companion object {
        @JvmStatic
        val ELEMENTS : ReadOnlyList<PTElement> = PTElement.initElements()

        private const val URL = "https://raw.githubusercontent.com/Bowserinator/Periodic-Table-JSON/master/PeriodicTableJSON.json"

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
            val name = lazy { obj.getString("name").get() }
            return PTElement(name)
        }
    }

    val name : String
        get () { return _name.value }
}