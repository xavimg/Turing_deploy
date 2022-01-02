package org.proj.game

import org.proj.db.PlayerId
import java.security.InvalidParameterException

class Inventory (
        val player: PlayerId,
        var maxCapacity: Double,
        private val internal: HashMap<PlayerResource, UInt>
): AbstractMap<PlayerResource, UInt>() {

    constructor (player: PlayerId, maxCapacity: Double) : this (player, maxCapacity, HashMap())
    constructor (player: PlayerId, maxCapacity: Double, entries: Map<PlayerResource, UInt>) : this (player, maxCapacity, HashMap(entries))

    override val entries: Set<Map.Entry<PlayerResource, UInt>>
        get() = this.internal.entries

    val capacity: Double
        get() = this.internal.entries.stream()
                .map { x -> x.key.resource.size * x.value.toDouble() }
                .reduce { x,y -> x + y }.orElse(0.0)

    private fun getEntryOf (resource: Resource): Map.Entry<PlayerResource, UInt>? {
        return this.internal.entries.stream()
                .filter { x -> x.key.equals(resource) }
                .findAny()
                .orElse(null)
    }

    private fun getKeyOf (resource: Resource): PlayerResource? {
        return getEntryOf(resource)?.key
    }

    private fun capacityWithout (entry: Map.Entry<PlayerResource, UInt>?): Double {
        if (entry == null) {
            return this.capacity
        }

        return this.internal.entries.stream()
                .filter { x -> x !== entry }
                .map { x -> x.key.resource.size * x.value.toDouble() }
                .reduce { x,y -> x + y }.orElse(0.0)
    }

    fun offset (resource: Resource, amount: Int): Result<Inventory> {
        val entry = this.getEntryOf(resource)
        val key: PlayerResource
        val value: UInt

        if (entry == null) {
            if (amount < 0) {
                return Result.failure(ArithmeticException())
            } else if (amount == 0) {
                return Result.success(this)
            }

            key = PlayerResource(this.player, resource)
            value = amount.toUInt()
        } else {
            key = entry.key
            value = entry.value
        }

        val current = this.capacityWithout(entry)
        if (current + key.resource.size * value.toFloat() >= this.maxCapacity) {
            return Result.failure(InvalidParameterException())
        }

        this.internal[key] = value
        return Result.success(this)
    }

    fun add (resource: Resource, amount: UInt): Result<Inventory> {
        return offset(resource, amount.toInt())
    }

    fun remove (resource: Resource, amount: UInt): Result<Inventory> {
        return offset(resource, -amount.toInt())
    }

    fun get (resource: Resource): UInt {
        val key = getKeyOf(resource) ?: return 0u
        return this.internal[key]!!
    }
}