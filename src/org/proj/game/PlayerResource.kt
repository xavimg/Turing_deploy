package org.proj.game

import org.proj.db.PlayerId

data class PlayerResource (
    val player: PlayerId,
    val resource: Resource,
    val lifeLeft: Double = 100.0
) {
    fun equals (other: Resource): Boolean {
        return this.resource === other
    }
}
