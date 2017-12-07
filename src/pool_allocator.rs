// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/*
    What's an object pool ?

    It's a collection of reusable objects, which are allocated in advance.
    We can ask an object if he's "alive".

    When the pool is initialized, it creates the entire collection of objects and initialize them
    to the "not in use" state.

    When you want a new object, you ask the pool to give the first "not in use" object.
    When you no longer need the object, the object goes back to the "not in use" state.

    From the user's perspective, he creates(allocate) and destroys(deallocate) objects, but no
    allocations occur :

    let my_object = Box::new(Monster::default()) -> allocation on the heap

    let monsterPool: ObjectPool<Monster> = ObjectPool::with_capacity(20); // 20 monsters pre-allocated
    let my_object = monsterPool.new(); //return a &mut Monster or RefCell<Monster>, and no allocation occurred.

    We can maybe create a wrapper around a RefCell<my_type> and impl Drop -> set the in_use property automatically when the RefCell goes out of scope.
*/

/*
                DESIGN DECISIONS

    We want a generic object pool :

    Objects contained should not have to know they are in a pool.
    That way, any type can be pooled.
    Since they don't know they are in a pool, the query to know if they are "in use"
    must live outside of the objects.

    It is not the pool who initialize/re-initialize the objects, it's outside code.
    The pool will just return a mutable pointer, or reference to the first object available and mark it as used.
    The handler of the reference or pointer will be able to use the initialization functions of the object.

    Pool<T>.create() -> Option<T>. when we ask the pool to give use an object, we return an option because all objects might be in use.
    If it returns None, we can do :
    - nothing, don't create the object
    - ask the pool to free an object and ask again
    - panic and yell at the programmer that its pool is too small for what he's trying to do.

    Functions needed :
    initialization: with_capacity(usize) -> Self
    query: create() -> Option<A reference or pointer to the object>
    kill an object to create another :
     force_kill(closure with a bool predicat) like force_kill(|obj| {!obj.is_on_screen()}) -> the first object outside vision cone is deleted

   Data structure inside the pool :
   - Vector ?
   - Linked list ?
   - a "free list" ?
   - a simple array ?   We don't need a growable array, and since our type will probably be allocated on the heap
                        we would have a heap-allocated array holding pointers to data allocated on the heap.


   What's stocked :
   - Rc<RefCell<T>> ?
   - Arc<Mutex<T>> ? We want multi-threading in Maskerad ! maybe two versions of the objectPool.
    - T ?

*/

//TODO:
/*
See that :
https://en.wikipedia.org/wiki/Free_list
https://en.wikipedia.org/wiki/Object_pool_pattern
see the free list solution.
*/
use std::sync::Arc;
use std::sync::Mutex;

pub struct ConcurrentObject<T> {
    object: Arc<Mutex<T>>,

}

pub struct ConcurrentObjectPool<T> {
    objects: Vec<Arc<Mutex<T>>>,
}

impl<T, N: Integer> ObjectPool<T> {
    fn with_capacity(size: usize) -> Self {
        let vec = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push()
        }
        ObjectPool {
            objects: ,
        }
    }
}

#[cfg(test)]
mod objectpool_tests {
    use super::*;

    #[test]
    fn test_tatatata() {

        let test = ObjectPool::new();
        println!("{:?}", test.objects);
        panic!()
    }
}