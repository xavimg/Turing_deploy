package org.proj.game

import org.rol.ReadOnlyList

interface Resource {
    companion object {
        val ALL : ReadOnlyList<Resource> = ReadOnlyList.ofArray(Iron)
    }

    val name : String
        get() = this.javaClass.simpleName
    val size: UInt
    val probability: Float
    val price: Float
}

interface EnergySource: Resource {
    val energy: UInt
}

interface Healer: Resource {
    val health: UInt
}

interface Damager {
    val requisite: Pair<Resource, UInt>
    val damage: UInt
}

// TODO COMPOSITE

// BASICS
object Iron: Resource {
    override val size: UInt = 1u
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
}

object Gold: Resource {
    override val size: UInt = 1u
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
}

object Diamond: Resource {
    override val size: UInt = 1u
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
}

// ENERGY SOURCES
object Coal: EnergySource {
    override val size: UInt = 1u
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
    override val energy: UInt = 1u
}

object Petroleum: EnergySource {
    override val size: UInt = 1u
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
    override val energy: UInt = 3u
}

object Uranium: EnergySource {
    override val size: UInt = 1u
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
    override val energy: UInt = 8u
}

// HEALERS
object Bandage: Healer {
    override val size: UInt = 2u
    override val probability: Float = 0f // TBD
    override val price: Float = Float.NaN // TBD
    override val health: UInt = 5u
}

object Medicine: Healer {
    override val size: UInt = 2u
    override val probability: Float = 0f // TBD
    override val price: Float = Float.NaN // TBD
    override val health: UInt = 15u
}

object HealthKit: Healer {
    override val size: UInt = 2u
    override val probability: Float = 0f // TBD
    override val price: Float = Float.NaN // TBD
    override val health: UInt = 30u
}

// COMPOSITES
object Bullet: Composi