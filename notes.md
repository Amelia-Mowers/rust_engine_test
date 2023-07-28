Notes for new rust project Rust:

Single static struct of arrays for collection

Generating a new individual struct adds to collection struct and returns reference to that point in array

Instead of using rc's have objects be in a static array Then you can store direct references

Fine for immutable objects, but some game objects will require interior mutability 

The only one compatible with direct references is using refcell, 


Objects
ObjectCollections

Events
EventStack

Messages (pub/sub)

Bi-directional relationships 



ObjectCollections when initialized register callbacks on events 


Calling an event pushes it to the event stack 



Main initializes all the object collections, including registering to events

Then runs a game loop of processing player inputs and stuff like "turn" events

Classes can have event listeners that run a function on an event

All instances of a class listen to an event (though there could be geographic isolation)

Classes have a static method that refers to their collection 

Events are classes
Each event has it's own event bus

Each event bus is a collection of references 
And the type of object referenced
Each type of object has an appropriate callback 

Actually better if it is a reference to a collection 

Having all instances of a class listen to an event can be implemented by having the class collection subscribe to the event, then passing that event to each contained class

Events that collections subscribe to would 

Collection by default would be "all" but could have additional indexes, such as geometric location 

Classes register to the event for their entire class
Static init function 

De-anonomyzing types at the collection level instead of the object level


Event filtering needs to be supported by the object, or have fallback to all



When you register an event for a class, it's the class, plus the callback 

Callback takes an object of that type, and the event


Calling an event is the event data, and any filter properties


Signals and messages
And pub/sub


Getting 



Collection of Collections of object 


In Rust, the `Vec<T>` type (a growable list type, aka 'vector') is generally the first choice for a collection with resizing capabilities. It provides amortized O(1) time complexity for append operations, and efficient memory use by reallocating memory in chunks when needed.

However, it doesn't specifically keep track of "unused" parts of the array, in the sense of non-contiguous free spaces due to object removal. When you remove an item from a vector, the space is not "saved" for future insertions; if you remove an item from the middle, all subsequent items are shifted down. The memory footprint of a `Vec<T>` may be larger than the number of elements it contains due to this reallocation strategy, but it doesn't explicitly reuse previous allocations within itself in the way you've described.

If you're looking for something like that, you might be interested in using a memory pool or object pool pattern, where you explicitly manage the reuse of memory slots. There's no built-in type for this in Rust's standard library, but you could create a custom type, or look for a crate that provides this functionality.

For example, you could use a `Vec<Option<T>>` where you store `None` in slots that are currently unoccupied. When you need to insert a new element, you could scan for a `None` value and replace it. This isn't as efficient as a `Vec<T>` for insertion (since you have to scan for free slots) or for memory usage (since you have to store the `None` values), but it does allow you to reuse memory slots in the way you've described.

If you frequently have a situation where you are removing elements from the middle and want to reuse their slots, then you might be better served by a different data structure, such as a linked list or a tree. The best choice will depend on the specific details of your use case.


User
Instead of doing option you could use a custom enum where the other option is the next empty data value, so you could have a type of linked list for finding open data slots 


That's a very clever idea! You're basically suggesting to implement a kind of free list within the array. Here's how you could define such a collection in Rust:

```rust
enum Slot<T> {
    Data(T),
    Empty(Option<usize>), // The next free slot's index
}

pub struct CustomVec<T> {
    data: Vec<Slot<T>>,
    next_free_slot: Option<usize>,
}

impl<T> CustomVec<T> {
    pub fn new() -> Self {
        CustomVec {
            data: Vec::new(),
            next_free_slot: None,
        }
    }

    pub fn insert(&mut self, value: T) -> usize {
        match self.next_free_slot {
            Some(index) => {
                // We have a free slot we can use
                if let Slot::Empty(next_free_slot) = &self.data[index] {
                    self.next_free_slot = *next_free_slot;
                } else {
                    panic!("Corrupted free slot list!");
                }

                self.data[index] = Slot::Data(value);
                index
            }
            None => {
                // No free slot, append to the end
                self.data.push(Slot::Data(value));
                self.data.len() - 1
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        match self.data.get_mut(index) {
            Some(slot @ Slot::Data(_)) => {
                let old_slot = std::mem::replace(slot, Slot::Empty(self.next_free_slot));
                self.next_free_slot = Some(index);
                if let Slot::Data(value) = old_slot {
                    Some(value)
                } else {
                    unreachable!()
                }
            }
            _ => None, // Either an empty slot, or out of bounds
        }
    }
}
```

In this custom vector, when you remove an element it becomes the head of the free list, and when you insert an element it first checks if there's a free slot to reuse.

This implementation is quite simplistic and could be improved in many ways (for example, by handling out-of-bounds accesses more robustly), but hopefully it gives you a good starting point. Keep in mind that this is a more advanced topic and requires a solid understanding of Rust's memory safety rules and ownership model.


User
But want to prioritize filling lower portions of the vector, possibly using a tree structure?

Keep track of the farthest filled vector

So that getting an iterable of filled objects can be efficient


If you want to maintain the order of filled slots and keep track of the farthest filled index, you could implement this using a Binary Search Tree (BST) or a Segment Tree. These tree structures can help you maintain the order of filled slots and locate the next available slot efficiently.

In your case, where you want to prioritize filling lower portions of the vector, you may use a BST where each node represents a free slot. The tree could be balanced such as an AVL tree or a red-black tree to maintain an efficient search, insertion and deletion operations.

However, implementing this kind of tree is not a trivial task and might be overkill for your needs, considering that Rust doesn't have built-in support for this kind of structure. Also, balancing tree algorithms can be complex to implement correctly. Rust standard library does not provide a balanced binary tree, you would have to use an external crate or implement it yourself.

An alternative to consider might be a PriorityQueue or BinaryHeap, where the priority is given to lower indices. This way, you always fill the lowest available slot.

Here is a simplified version using a BinaryHeap:

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

enum Slot<T> {
    Data(T),
    Empty,
}

pub struct CustomVec<T> {
    data: Vec<Slot<T>>,
    empty_slots: BinaryHeap<Reverse<usize>>, // min heap to prioritize lower indices
}

impl<T> CustomVec<T> {
    pub fn new() -> Self {
        CustomVec {
            data: Vec::new(),
            empty_slots: BinaryHeap::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> usize {
        let index = match self.empty_slots.pop() {
            Some(Reverse(index)) => index,
            None => {
                let index = self.data.len();
                self.data.push(Slot::Empty);
                index
            }
        };
        self.data[index] = Slot::Data(value);
        index
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        match self.data.get_mut(index) {
            Some(slot @ Slot::Data(_)) => {
                let old_slot = std::mem::replace(slot, Slot::Empty);
                self.empty_slots.push(Reverse(index)); // add index to available slots
                if let Slot::Data(value) = old_slot {
                    Some(value)
                } else {
                    unreachable!()
                }
            }
            _ => None, // Either an empty slot, or out of bounds
        }
    }

    // Iterating over data slots
    pub fn iter_data(&self) -> impl Iterator<Item = &T> {
        self.data.iter().filter_map(|slot| {
            match slot {
                Slot::Data(ref t) => Some(t),
                _ => None,
            }
        })
    }
}
```

This variant will keep track of empty slots and prioritize using lower indices. It's important to note that while this is a creative way to meet your requirements, it adds complexity and computational overhead to your data structure. If performance is a high priority, it might be better to find a way to work with Rust's built-in collection types, even if it means changing your design or algorithms to better suit their behavior.


User
Wait, what if when I deallocate an object I copy over the data from the last object in the array, then the assigned objects is always a contiguous array


Yes, that is a very common and effective strategy used in many programming contexts, and it would ensure that your data remains contiguous. This is essentially the swap-and-pop idiom, and it's a great way to efficiently remove an item from an unordered list while keeping memory usage compact.

Here's how you could implement it:

```rust
pub struct CustomVec<T> {
    data: Vec<Option<T>>,
}

impl<T> CustomVec<T> {
    pub fn new() -> Self {
        CustomVec {
            data: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> usize {
        let index = self.data.len();
        self.data.push(Some(value));
        index
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let len = self.data.len();
        if index >= len {
            return None; // Out of bounds
        }

        let last_element = self.data.pop();
        let removed_element = std::mem::replace(&mut self.data[index], last_element.unwrap());

        removed_element
    }

    // Iterating over data slots
    pub fn iter_data(&self) -> impl Iterator<Item = &T> {
        self.data.iter().filter_map(|slot| {
            match slot {
                Some(ref t) => Some(t),
                _ => None,
            }
        })
    }
}
```

In this version of your `CustomVec`, the `remove` function swaps the element to be removed with the last element in the array (popped off the end), then returns the removed element. This keeps all the remaining data elements contiguous at the start of the vector.

Be aware that this method doesn't preserve the ordering of the elements. If order matters, this approach won't work, and you would need to consider other strategies.