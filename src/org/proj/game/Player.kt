package org.proj.game

import org.proj.db.PlayerId

/**
 * WARNING: WHEN SENDING POSITION AND VELOCITY, IT MUST BE WITH RESPECT TO PLAYER'S PROPER TIME
 */
data class Player (
    val id: PlayerId,
    val inventory: Inventory,
    val health: Double
)