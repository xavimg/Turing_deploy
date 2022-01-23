package org.proj.game
import org.rol.ReadOnlyList

interface Resource {
    companion object {
        val ALL : ReadOnlyList<Resource> = ReadOnlyList.ofArray(
            Iron, Gold, Sulfur, Potassium, Oxygen, Diamond,
            Coal, Petroleum, Uranium,
            Bandage, Medicine, HealthKit,
            Railgun,
            Bullet, Niter, Gunpowder
        )

        fun valueOf (name: String): Resource? {
            return this.ALL.stream()
                    .filter { x -> x.name == name }
                    .findAny()
                    .orElse(null)
        }
    }

    val name : String
        get() = this.javaClass.simpleName
    val size: Float
    val probability: Float
    val price: Float
}

interface EnergySource: Resource {
    val energy: UInt
}

interface Healer: Resource {
    val health: UInt
}

interface Damager: Resource {
    val requisite: Pair<Resource, UInt>
    val damage: UInt
}

abstract class Composite: Resource {
    override val size: Float by lazy { this.parents.entries.stream()
            .map { x -> x.key.size * x.value.toFloat() }
            .reduce { x,y -> x + y }.get() }

    abstract val parents: Map<Resource, UInt>
}

// BASICS
object Iron: Resource {
    override val size: Float = 0.1f
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
}

object Gold: Resource {
    override val size: Float = 0.1f
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
}

object Sulfur: Resource {
    override val size: Float = 0.1f
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
}

object Potassium: Resource {
    override val size: Float = 0.1f
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
}

object Nitrogen: Resource {
    override val size: Float = 0.1f
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
}

object Oxygen: Resource {
    override val size: Float = 0.1f
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
}

object Diamond: Resource {
    override val size: Float = 1f
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
}

// ENERGY SOURCES
object Coal: EnergySource {
    override val size: Float = 1f
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
    override val energy: UInt = 1u
}

object Petroleum: EnergySource {
    override val size: Float = 0.2f
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
    override val energy: UInt = 3u
}

object Uranium: EnergySource {
    override val size: Float = 0.1f
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
    override val energy: UInt = 8u
}

// HEALERS
object Bandage: Healer {
    override val size: Float = 1f
    override val probability: Float = 0f // TBD
    override val price: Float = Float.NaN // TBD
    override val health: UInt = 5u
}

object Medicine: Healer {
    override val size: Float = 1.5f
    override val probability: Float = 0f // TBD
    override val price: Float = Float.NaN // TBD
    override val health: UInt = 15u
}

object HealthKit: Healer {
    override val size: Float = 2f
    override val probability: Float = 0f // TBD
    override val price: Float = Float.NaN // TBD
    override val health: UInt = 30u
}

// DAMAGERS
object Railgun: Damager {
    override val size: Float = 5f
    override val probability: Float = 0f // TBD
    override val price: Float = Float.NaN // TBD
    override val damage: UInt = 30u
    override val requisite: Pair<Resource, UInt> = Pair(Bullet, 1u)
}

// COMPOSITES
object Bullet: Composite() {
    override val probability: Float = 0f // TBD
    override val price: Float = Float.NaN // TBD
    override val parents: Map<Resource, UInt> = mapOf(Pair(Iron, 1u))
}

object Niter: Composite() {
    override val probability: Float = Float.NaN // TBD
    override val price: Float = Float.NaN // TBD
    override val parents: Map<Resource, UInt> = mapOf(Pair(Potassium, 1u), Pair(Nitrogen, 1u), Pair(Oxygen, 3u))
}

object Gunpowder: Composite() {
    override val probability: Float = 0f // TBD
    override val price: Float = Float.NaN // TBD
    override val parents: Map<Resource, UInt> = mapOf(Pair(Sulfur, 1u), Pair(Coal, 1u), Pair(Niter, 1u))
}