#![cfg(not(oj_no_merge))]

/// ## Arc
/// 
/// Handroll your own naive `Arc`!
/// You will need to write some `unsafe` code, like dereferencing raw pointers and implementing `Send` and `Sync` traits.
/// Don't be afraid, we will have a bigger one for you later 🤫.
/// 
/// If you run into problems, feel free to read the standard library implementation of `Arc` in the [Rust source code](https://doc.rust-lang.org/src/alloc/sync.rs.html#248-251).
/// 
/// ### Thread Safety
/// 
/// We know `Rc` is for reference counting, but not thread safe.
/// The reason is simple: the counter is not atomic.
/// Rust separates these two versions of referece counting pointers to avoid the
/// overhead of atomic operations, and support architectures that do not have
/// atomic operations (for example, the current version of WASI) to do basic reference counting.
/// 
/// `Rc` implements `!Send` and `!Sync` to prevent you from sending it across threads.
/// `Send` means you can move the value to another thread safely,
/// and `Sync` means you can share its reference between threads safely.
/// Hence type `T` is `Sync` if and only if `&T` is `Send`.
/// 
/// ### Atomics and Ordering
/// 
/// Rust provides you atomic basic types like `AtomicUsize` and `AtomicBool` in the `std::sync::atomic` module.
/// These types are `Sync`, which means different threads can safely borrow them at the same time.
/// However, they still follow the ownership rules of Rust,
/// which means different threads can't share the ownership of the same atomic.
/// For shared ownership across threads, you have to wrap them within an `Arc`.
/// 
/// But they do have **interior mutability**, which means you can mutate them without
/// a `Mutex` or `RwLock` behind a shared reference.
/// After all, atomic operations should have been lock-free!
/// 
/// The example below demonstrates a simple spinlock with `AtomicBool`:
/// 
/// ```rust
/// use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
/// use std::thread;
/// 
/// static COUNTER: AtomicUsize = AtomicUsize::new(0);
/// static READY: AtomicBool = AtomicBool::new(false);
/// 
/// thread::spawn(|| {
///     COUNTER.store(114, Ordering::Relaxed);
///     READY.store(true, Ordering::Release);
///     thread::sleep(std::time::Duration::from_millis(100));
///     COUNTER.store(514, Ordering::Relaxed);
/// });
/// while !READY.load(Ordering::Acquire) {
///     thread::sleep(std::time::Duration::from_millis(100));
/// }
/// println!("{}", COUNTER.load(Ordering::Relaxed));
/// ```
/// 
/// The `Ordering` enum specifies the memory ordering constraints for the atomic operations.
/// The constraints come directly from C++. Three ordering used in handrolling `Arc` are:
/// 
/// - `Relaxed` means no guarantees about the ordering. Any reordering is allowed.
/// - `Release` means all previous writes are guaranteed to be visible to other threads.
/// - `Acquire` means all subsequent reads are guaranteed to see the writes from the releasing thread.
/// 
/// Basically, a pair of `Release` and `Acquire` is a **synchronization point** between threads.
/// 
/// Read the atomic book from Mara Bos (who is incumbent standard library team lead)
/// for more details about Rust [atomics](https://marabos.nl/atomics/atomics.html) and [memory ordering](https://marabos.nl/atomics/memory-ordering.html).
/// 
/// ### Quiz
/// 
/// Now let's implement a naive `Arc` with the given struct definition.
/// **Remove this part before you submit!**
/// 
/// ```rust
/// use std::{
///     ops::Deref,
///     sync::atomic::{
///         AtomicUsize,
///         Ordering::{Acquire, Relaxed, Release},
///     },
/// };
/// 
/// struct Arc<T> {
///     inner: *mut ArcInner<T>,
///     phantom: std::marker::PhantomData<ArcInner<T>>,
/// }
/// 
/// struct ArcInner<T> {
///     data: T,
///     counter: AtomicUsize,
/// }
/// 
/// unsafe impl<T: Send + Sync> Send for Arc<T> {}
/// unsafe impl<T: Send + Sync> Sync for Arc<T> {}
/// unsafe impl<T: Send + Sync> Send for ArcInner<T> {}
/// unsafe impl<T: Send + Sync> Sync for ArcInner<T> {}
/// ```
/// 
/// The `ArcInner` is a helper storage for the real data, with a reference counter.
/// Here we ignore the weak reference count for simplicity.
/// The `data` is a raw pointer to our data, and `counter` is the reference counter.
/// 
/// Our mini `Arc` holds a raw mutable pointer to the `ArcInner` and a phantom data.
/// The phantom data is a zero-sized type that tells the compiler to treat our `Arc<T>`
/// as if it owns a `ArcInner<T>`.
/// 
/// Because we manually manage the synchronization, we declare our `Arc` to be `Send` and `Sync`.
/// 
/// Here are some methods you need to implement:
/// 
/// `Arc::new` - Move data into a `ArcInner` and then **leak it**, so that no one owns it.
/// ```no_run
/// impl<T> Arc<T> {
///     fn new(data: T) -> Self {
///         todo!()
///     }
/// }
/// ```
/// 
/// **Hint**: the data cannot live on the stack,
/// because the `Arc` may outlive the current scope, even be thrown across threads.
/// We don't have something like the C++ `new` keyword to allocate memory on
/// the heap, but `Box` handles memory allocation for us. Try to hack a `Box` to
/// get the raw mutable pointer we want.
/// 
/// `Clone::clone` - Create a new `Arc` pointing to the same data.
/// Think about this: do we have to use the `Release` ordering when incrementing the counter?
/// Is it safe to use `Relaxed`? 
/// 
/// ```no_run
/// impl<T> Clone for Arc<T> {
///     fn clone(&self) -> Self {
///         todo!()
///     }
/// }
/// ```
/// 
/// In our problem we guarantee the counter will never overflow.
/// 
/// `Drop::drop` - Destroy the `Arc` and check whether the data should be freed.
/// This time, when we decrement the counter, 
/// how do we guarantee that the actual data destruction cannot happen before the counter is decremented?
/// The destruction is not an atomic operation, so we probably need to insert one just for guarding the order.
/// 
/// ```no_run
/// impl<T> Drop for Arc<T> {
///     fn drop(&mut self) {
///         todo!()
///     }
/// }
/// ```
/// 
/// You also need to implement at least **one more trait** to make the `Arc` work as *a smart pointer*.
/// 
/// We test your Arc with the following toy concurrent code example:
/// 
/// ```rust
/// fn test() {
///     let mut buffer = String::new();
///     std::io::stdin().read_line(&mut buffer).unwrap();
///     let data: Vec<isize> = buffer
///         .trim()
///         .split_whitespace()
///         .map(|x| x.parse().unwrap())
///         .collect();
///     let data_size = data.len();
///     let data = Arc::new(data);
///     let answer = Arc::new(AtomicUsize::new(0));
///     
///     std::thread::scope(|s| {
///         for i in 0..data_size {
///             // A common pattern: clone `Arc`s and move them to threads.
///             let data = data.clone();
///             let answer = answer.clone();
///             s.spawn(move || {
///                 let element = data[i];
///                 answer.fetch_add((element * element) as usize, Relaxed);
///             });
///         }
///     });
///     println!("{}", answer.load(Relaxed));
///     unsafe { assert_eq!((*data.inner).counter.load(Relaxed), 1, "Your reference counting is wrong.") };
/// }
/// ```

use std::{
    ops::Deref,
    sync::atomic::{
        AtomicUsize,
        Ordering::{Acquire, Relaxed, Release},
    },
};

struct Arc<T> {
    inner: *mut ArcInner<T>,
    phantom: std::marker::PhantomData<ArcInner<T>>,
}

struct ArcInner<T> {
    data: T,
    counter: AtomicUsize,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}
unsafe impl<T: Send + Sync> Send for ArcInner<T> {}
unsafe impl<T: Send + Sync> Sync for ArcInner<T> {}

#[cfg(not(feature = "judge"))]
impl<T> Arc<T> {
    fn new(data: T) -> Self {
        let inner = Box::new(ArcInner {
            data,
            counter: AtomicUsize::new(1),
        });
        Self {
            inner: Box::into_raw(inner),
            phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(not(feature = "judge"))]
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        // The other thread cannot accidentally free the data
        // because we have at least one reference in our current thread.
        unsafe { (*self.inner).counter.fetch_add(1, Relaxed) };
        Self {
            inner: self.inner,
            phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(not(feature = "judge"))]
impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        // The other thread may accidentally read the data.
        if unsafe { (*self.inner).counter.fetch_sub(1, Release) } != 1 {
            // Not the last reference, just return.
            return;
        }

        // Add a barrier to prevent from reordering the following operations.
        std::sync::atomic::fence(Acquire);

        // Now we are sure that we are the last reference.
        unsafe { std::ptr::drop_in_place(self.inner) };
    }
}

#[cfg(not(feature = "judge"))]
impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &unsafe { &*self.inner }.data
    }
}

fn test() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data: Vec<isize> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let data_size = data.len();
    let data = Arc::new(data);
    let answer = Arc::new(AtomicUsize::new(0));

    std::thread::scope(|s| {
        for i in 0..data_size {
            // A common pattern: clone `Arc`s and move them to threads.
            let data = data.clone();
            let answer = answer.clone();
            s.spawn(move || {
                let element = data[i];
                answer.fetch_add((element * element) as usize, Relaxed);
            });
        }
    });
    println!("{}", answer.load(Relaxed));
    unsafe { assert_eq!((*data.inner).counter.load(Relaxed), 1, "Your reference counting is wrong.") };
}

fn main() {
    test();
}
