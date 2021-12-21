package org.proj.game.resource

import org.bson.json.JsonReader
import org.rol.ReadOnlyList
import org.sjr.JSONObjectWrapper
import java.io.*
import java.lang.Exception
import kotlin.system.exitProcess

class PTElement (private val _name: Lazy<String>) {
    companion object {
        @JvmStatic
        val ELEMENTS : ReadOnlyList<PTElement> = initElements()

        private const val URL = "https://raw.githubusercontent.com/Bowserinator/Periodic-Table-JSON/master/PeriodicTableJSON.json"

        private fun getReader () : InputStreamReader {
            val url = java.net.URL(URL)
            var stream : InputStream = try {
                url.openStream()
            } catch (e: Exception) {
                e.printStackTrace()
                try {
                    val conn = url.openConnection()
                    conn.setRequestProperty("accept", "application/json");
                    conn.getInputStream()
                } catch (e2: Exception) {
                    e2.printStackTrace()
                    exitProcess(0)
                }
            }

            return InputStreamReader(url.openStream())
        }

        private fun initElements () : ReadOnlyList<PTElement> {
            val json : JSONObjectWrapper
            try {
                json = JSONObjectWrapper(getReader())
            } catch (e: Exception) {
                e.printStackTrace()
                exitProcess(1)
            }

            val elements = json.getAsArray<JSONObjectWrapper>("elements").get()
            val map = elements.map { x -> initElement(x) }
            return ReadOnlyList.ofList(map)
        }

        private fun initElement (obj: JSONObjectWrapper) : PTElement {
            val name = lazy { obj.getString("name").get() }
            return PTElement(name)
        }
    }

    val name : String
        get () { return _name.value }
}