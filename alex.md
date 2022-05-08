# Database cache
When I started the development of the database integration for our game's backend, I quickly realised that when the system could easily bottleneck if an even relatively small amount of players connected siultaneously, so I decided the best course of action would be a caching layer thet would reside all (or most) the interactions between the backend and the database.
In essence, the structure of the cache is the following

```rust
pub struct CollectionCache<T> {
    collection: Collection<T>,
    cache: Arc<RwLock<HashSet<Arc<T>>>>
}
```

Given the asynchronous nature of our backend, queries to the database are required to be executable between threads, and their validity and safety must be asserted.

The first field is the "raw" connection to the database, which has thread-safety implementet by it's provider (the [```mongodb```](https://crates.io/crates/mongodb) crate), whilsts the second is the actual cached entries stored in-memory, accompanied by thread-safety measures.

The second field the actual cache, stored inside a [```HashSet```](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html), the thread-safety of which is ensured by the structures arroud it, which we'll now deconstruct piece by piece.

## Reference Counting
A **R**eference **C**ounting pointer is a type with a single goal: Manage heap-allocated memory accessible for multiple functions simultaneously. To do so, reference counting pointers will **count** the number of **references** (get it?) they give, decreaseing the counter when a reference is dropped. When that number is zero, we know for a fact that no other part of the code has access to the underlaying data, so we'll drop and deallocate it from the heap, but we still have a problem if we want to access this memory from multiple threads.

## Atomic operations
The problem aforementioned is solved in Rust by the [```Rc```](https://doc.rust-lang.org/stable/alloc/rc/struct.Rc.html) object, but that implementation is not thread-safe. The reason is that when loading and storing the newly updated counter between threads, we have the threat of [race conditions](https://en.wikipedia.org/wiki/Race_condition). For example, imagine we have an integer with value **1**. A thread that loads the value, followed by a different thread doing the same. The both threads increment the value read by one, and then the first thread stores it's result into memory, followed by the second thread. 

Whilst we expected the final value to be **3**, the order of operations has caused our integer to have a final value of **2**.
If we analise each instruction one by one, the problem becomes quickly aparent.

| Thread | Operation | Local Value | Actual Value |
| ------ | --------- | ----------- | ------------ |
| #1     | LOAD      | 1           | 1            |
| #2     | LOAD      | 1           | 1            |
| #1     | ADD       | 2           | 1            |
| #2     | ADD       | 2           | 1            |
| #1     | STORE     | 2           | 2            |
| #2     | STORE     | 2           | 2            |

The problem is that, when loading and storing values in different instructions, their thread-safety cannot be ensuread. Introduce [atomics](https://en.wikipedia.org/wiki/Linearizability#Primitive_atomic_instructions).
Atomic instructions are a set operations provided by the processor that guarantee the execution of two, traditionally sperate instructions, in a single instruction, making them **indivisible** (or atomic, hehehe).
With this new instructions, we can create an **A**tomic **R**eference **C**ounting pointer (also known as [```Arc```](https://doc.rust-lang.org/stable/std/sync/struct.Arc.html)), which is safe from race conditions and thus, thread safe.

Whilst processors have a variety of atomic instructions, the one we're interested in is the [fetch and add](https://en.wikipedia.org/wiki/Fetch-and-add) instruction (in x86 assembly, [```LOCK```](https://www.felixcloutier.com/x86/lock) [```XADD```](https://www.felixcloutier.com/x86/xadd)). Re-executing our example with atomic instructions, we get the following

| Thread | Operation    | Local Value | Actual Value |
| ------ | ------------ | ----------- | ------------ |
| #1     | LOCK XADD    |             | 1            |
| #1     | Locked LOAD  | 1           | 1            |
| #1     | Locked ADD   | 2           | 1            |
| #1     | Locked STORE | 2           | 2            |
| #2     | LOCK XADD    |             | 2            |
| #2     | Locked LOAD  | 2           | 2            |
| #2     | Locked ADD   | 3           | 2            |
| #2     | Locked STORE | 3           | 3            |

> **Note**\
> In this examples, the [```XADD```](https://www.felixcloutier.com/x86/xadd), and [```LOCK```](https://www.felixcloutier.com/x86/lock) ```XADD``` instruction and pseudo-instruction have been deconstructed, for clarity, into their different parts.

## Locks
But sometimes, we have objects that create and manage complex systems, systems which cannot be reduced to smaller atomic operations. What do we do then? Then, we use [locks](https://en.wikipedia.org/wiki/Lock_(computer_science)).

Locks are, in essence, objects that deny or restrict the access to shared data, and one of the simplest and most common locks is probably the [mutex](https://en.wikipedia.org/wiki/Mutual_exclusion).

### Mutex 
The structure of a **Mut**ually **Ex**clusive lock (also known as ```Mutex```) can probably be summed up into the following.

```rust
pub struct Mutex<T> {
    locked: AtomicBool,
    data: T
}
```

> **Note**\
> In Rust, atomic operations are done by wrapping the value inside an atomic type (in this case, [```AtomicBool```](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicBool.html)). This way, the use of this values as regular ones is no longer possible. In other languages, atomic operations are done seperately (see [Go](https://pkg.go.dev/sync/atomic)) or they offer both methods (see [C++](https://www.cplusplus.com/reference/atomic/atomic/))

Eyy, it's our friend the atom!!! Indeed, locks rely on atomic operations to provide their safety guarantees.
In specific, mutexes allow for the access of memory to one, **and only one**, thread at a time (if used correctly).

See, in most languages, mutexes are not binded to their value, which assigns the responsability of using them correctly entrirelly to the programmer (looking at you, [Go](https://pkg.go.dev/sync#Mutex)). Thankfully, in Rust, the language will manage locking and unlocking the mutex for us.

But first, lets see how we make the mutex lock

```rust
impl<T> Mutex<T> {
    pub fn lock (&self) -> MutexGuard<'_, T> {
        loop {
            match self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire) {
                Ok(prev) if !prev => break
                _ => spin_loop() // hint to the compiler: 'We'll probably not be here for long. I'm optimistic :)'
            }
        }

        MutexGuard {
            inner: self
        }
    }
}
```

> **Note**\
> To learn about atomic ordering, see [this](https://en.cppreference.com/w/cpp/atomic/memory_order) and/or [this](https://doc.rust-lang.org/stable/nomicon/atomics.html). I still don't fully understand it, so I won't talk about it

Okey, so to lock the mutex, we repeatedly check if it's locked and, whenever we see it isn't, we lock it ourselves, great! But what is this ```MutexGuard```?

So, remember when I said that Rust is one of the few languages that attaches mutexes (and all locks for that matter) to their value?
Well, the ```MutexGuard``` is the object that manages this attachement.

Its basic implementation is the following

```rust
pub struct MutexGuard<'a, T> {
    inner: &'a Mutex<T>
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref (&self) -> &Self::Target {
        unsafe { & *self.inner.data }
    }
} 

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut (&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner.data }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop (&mut self) {
        self.inner.locked.store(false, Ordering::Release)
    }
}
```

As you can see, the guard is basically a wrapper arround a reference of it's parent, which allows access to the data contained inside the mutex through [```Deref```](https://doc.rust-lang.org/stable/std/ops/trait.Deref.html) and [```DerefMut```](https://doc.rust-lang.org/stable/std/ops/trait.DerefMut.html), and unlocks the mutex when it's dropped. 

> **Note**\
> The actual Rust implementation of [```Mutex```](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html) is more complex and allows for _poison detection_, but this implementation is appropiate for demonstration purposes

### Read-Write Lock
Read-Write locks are similar to mutexes, with the diference that they can grant read and write access to their data seperately.
Read access can be had by multiple threads simultaneously, but when a thread has write access, no other thread can have access to the data, be it read or write. This gives read-write locks more flexibility, at the cost of a more complex and expensive implementation. In Rust, a read-write lock is known as a [```RWLock```](https://doc.rust-lang.org/stable/std/sync/struct.RwLock.html).

## Making the soup
With all this acumulated knwoledge, lets revisit the fiald which contains our cache.

```rust
cache: Arc<RwLock<HashSet<Arc<T>>>>
```

Okey, so we have an ```Arc``` that allows us to share the cache between threads, this is good.
But see, reference counting pointers don't (or shouldn't) give mutable access to their data. Remember, their only job is to guarantee multiple functions/threads can access their data without [leaking memory](https://en.wikipedia.org/wiki/Memory_leak), so they cannot guarantee secure mutable access to their data.

Threrfore, we wrap our cache inside a ```RWLock``` before puting it inside the ```Arc```. This will allow us to get read or write access to the cache depending on our needs.

Finally, we'll wrap the values inside the cache into ```Arc```s. This way, we'll be able to return read-only access to the cached data, instead of returning a clone of the data, which might be expensive, depending on the type.

## Implementing the queries
> **Note**\
> From here onward, we'll be using the async locks provided by [tokio](https://crates.io/crates/tokio).
> Also, we'll assume we're in a magical world where no errors ocurr, so no error handling will be shown here.

Okey, time to do something usefull with this cache. Since most query types aren't actually cacheable, we'll only show the implementation for which caching makes the most sense; Selection.

### Select
We'll start for the most basic selection then. For this query, the most straight forward implementation would be to look inside the cache, and if we don't find anything, do so on the database. Something like this

```rust
impl<T> CollectionCache<T> {
    pub async fn find_one<F: FnMut(&T) -> bool> (&self, cache_query: F, db_query: Document) -> Option<Arc<T>> {
        let cache = self.cache.read().await; // await read-only access to the cache
        if let Some(value) = cache.iter().find(|x: &Arc<T>| (cache_query)(&x)) { // search inside the cache
            return Some(value) // we found a value, our search is over
        }

        drop(cache); // we don't need access to the cache anymore, so we'll preemtively drop it's guard so others can use it.
        if let Some(value) = self.collection.find_one(db_query, None).await { // search on the database
            let value = Arc::new(value); // wrap the result inside an Arc
            if let Some(mut cache) = self.cache.try_write() { // since we're here, we should see if we can put the result inside the cache
                cache.insert(Arc::clone(value)); // we can?? great, I wasn't expecting it :)
            }

            return Some(value) // return the database result
        }

        None // no matching value found on the cache or the database, so no value matches the query
    }
}
```

This implementation is probably faster than just waiting for the database, but it has some problems. First of all, we'll have to wait to have access to the cache, and maybe it doesn't even have the value we're searching. Parhaps there are some rare cases where searching in the database is actually faster than the cache (maybe the queue for access to the cache is very long, for example).
To aliviate this problems, our implementation is as follows.

```rust
impl<T> CollectionCache<T> {
    pub async fn find_one<F: FnMut(&T) -> bool> (&self, cache_query: F, db_query: Document) -> Option<Arc<T>> {
        select! {
            Some(result) = self.find_one_cache(cache_query) => Some(result), // cache found a value first
            Some(result) = self.find_one_db(cache_query) => Some(result), // database found a value first
            else => None // no one found a matching value
        }
    }

    async fn find_one_cache<F: FnMut(&T) -> bool> (&self, query: F) -> Option<Arc<T>> {
        let cache = self.cache.read().await; // await read-only access to the cache
        cache.iter().find(|x: &Arc<T>| (query)(&x)) // perform the query
    }

    async fn find_one_db (&self, query: Document) -> Option<Arc<T>> {
        if let Some(value) = self.collection.find_one(query, None).await { // perform the query
            let value = Arc::new(value); // wrap the result inside an Arc
            if let Some(mut cache) = self.cache.try_write() { // since we're here, we should see if we can put the result inside the cache
                cache.insert(Arc::clone(value)); // we can?? great, I wasn't expecting it :)
            }

            return Some(value) // return the result
        }

        None // no matching value was found
    }
}
```

As you can see, in this implementation both queries are executed simultaneously, and we just keep the value that first returns, cancelling the pending query. This allows us to avoid wasting time waiting for access to the cache.