//! Implement your own reference-counting pointer.

/// ### Unsafe Practice: Implementing `Rc` from scratch
///
/// In this exercise, you are tasked with creating your own reference-counting
/// pointer, `Circ`, along with its weak counterpart, `WeakCirc`. These should
/// mirror the functionality of the standard library's [`std::rc::Rc`] and
/// [`std::rc::Weak`]. To meet the specific requirements, you need to implement
/// the `CricTrait` and `WeakCircTrait` traits for your `Circ` and `WeakCirc`
/// types, respectively. Refer to each trait's documentation for comprehensive
/// details.
///
/// This task may prove to be quite challenging, as it necessitates a solid
/// understanding of Rust's ownership, memory management, and unsafe operations.
/// It is advisable to study the documentation for [`std::rc::Rc`] and
/// [`std::rc::Weak`] to fully comprehend the requirements and behavior of these
/// types.
///
/// Consider the following questions as guidance throughout this exercise:
///
/// 1. How can you optimize a pointer with niche? What is the variance of the
///    pointer?
/// 2. When all strong references are dropped, how can you drop the data and
///    invalidate the weak references? Keep in mind that dropping and
///    deallocating are not synonymous.
/// 3. How can a weak reference determine whether the data is still valid? Is
///    there a direct or indirect indicator?
/// 4. The weak count is still accessible after all strong references are
///    dropped. How can you implement this?
mod circ {
    use super::{CricTrait, WeakCircTrait};
    use std::{
        cell::Cell,
        marker::PhantomData,
        mem::{ManuallyDrop, MaybeUninit},
        ops::Deref,
        ptr::NonNull,
    };

    #[repr(C)]
    struct CircInner<T> {
        strong: Cell<usize>,
        weak: Cell<usize>,
        data: ManuallyDrop<T>,
    }

    impl<T> CircInner<T> {
        fn inc_strong(&self) {
            self.strong.set(self.strong.get().wrapping_add(1));
            if self.strong.get() == 0 {
                panic!("overflow");
            }
        }

        fn dec_strong(&self) {
            self.strong.set(self.strong.get() - 1);
        }

        fn inc_weak(&self) {
            self.weak.set(self.weak.get().wrapping_add(1));
            if self.weak.get() == 0 {
                panic!("overflow");
            }
        }

        fn dec_weak(&self) {
            self.weak.set(self.weak.get() - 1);
        }
    }

    pub struct Circ<T> {
        ptr: NonNull<CircInner<T>>,
        _marker: PhantomData<CircInner<T>>,
    }

    impl<T> CricTrait<T> for Circ<T> {
        type Weak = WeakCirc<T>;

        fn new(data: T) -> Self {
            let inner = Box::new(CircInner {
                strong: Cell::new(1),
                weak: Cell::new(1),
                data: ManuallyDrop::new(data),
            });
            Circ {
                ptr: Box::leak(inner).into(),
                _marker: PhantomData,
            }
        }

        fn new_cyclic(data_fn: impl FnOnce(&WeakCirc<T>) -> T) -> Self {
            #[repr(C)]
            struct UninitInner<T> {
                strong: Cell<usize>,
                weak: Cell<usize>,
                data: MaybeUninit<T>,
            }

            let uninit_ptr: NonNull<_> = Box::leak(Box::new(UninitInner {
                strong: Cell::new(0),
                weak: Cell::new(1),
                data: MaybeUninit::<T>::uninit(),
            }))
            .into();

            let init_ptr = uninit_ptr.cast();
            let weak = WeakCirc { ptr: init_ptr };
            let data = data_fn(&weak);

            let rc = unsafe {
                let inner = init_ptr.as_ptr();
                std::ptr::write(
                    std::ptr::addr_of_mut!((*inner).data),
                    ManuallyDrop::new(data),
                );
                (*inner).strong.set(1);

                Circ {
                    ptr: init_ptr,
                    _marker: PhantomData,
                }
            };

            std::mem::forget(weak);
            rc
        }

        fn downgrade(this: &Circ<T>) -> WeakCirc<T> {
            let inner = unsafe { this.ptr.as_ref() };
            inner.inc_weak();
            WeakCirc { ptr: this.ptr }
        }

        fn strong_count(this: &Self) -> usize {
            let inner = unsafe { this.ptr.as_ref() };
            inner.strong.get()
        }

        fn weak_count(this: &Self) -> usize {
            let inner = unsafe { this.ptr.as_ref() };
            inner.weak.get() - 1
        }

        fn ptr_eq(this: &Self, other: &Self) -> bool {
            this.ptr == other.ptr
        }

        fn make_mut(this: &mut Self) -> &mut T
        where
            T: Clone,
        {
            if Self::strong_count(this) > 1 {
                let inner = Box::new(CircInner {
                    strong: Cell::new(1),
                    weak: Cell::new(1),
                    data: ManuallyDrop::new((**this).clone()),
                });
                let rc = Circ {
                    ptr: Box::leak(inner).into(),
                    _marker: PhantomData,
                };
                *this = rc;
            } else if Self::weak_count(this) > 0 {
                let inner = unsafe { this.ptr.as_ref() };
                inner.dec_strong();
                inner.dec_weak();

                let new_inner = Box::new(CircInner {
                    strong: Cell::new(1),
                    weak: Cell::new(1),
                    data: unsafe { std::mem::transmute_copy(&inner.data) },
                });

                this.ptr = Box::leak(new_inner).into();
            }

            assert_eq!(Self::strong_count(this), 1);
            assert_eq!(Self::weak_count(this), 0);
            unsafe { &mut this.ptr.as_mut().data }
        }
    }

    impl<T> Clone for Circ<T> {
        fn clone(&self) -> Self {
            let inner = unsafe { self.ptr.as_ref() };
            inner.inc_strong();
            Circ {
                ptr: self.ptr,
                _marker: PhantomData,
            }
        }
    }

    impl<T> Drop for Circ<T> {
        fn drop(&mut self) {
            let inner = unsafe { self.ptr.as_ref() };
            inner.dec_strong();
            if inner.strong.get() == 0 {
                unsafe { ManuallyDrop::drop(&mut self.ptr.as_mut().data) };

                let inner = unsafe { self.ptr.as_ref() };
                inner.dec_weak();
                if inner.weak.get() == 0 {
                    let _ = unsafe { Box::from_raw(self.ptr.as_ptr()) };
                }
            }
        }
    }

    impl<T> AsRef<T> for Circ<T> {
        fn as_ref(&self) -> &T {
            unsafe { &self.ptr.as_ref().data }
        }
    }

    impl<T> Deref for Circ<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

    struct WeakInner<'a> {
        strong: &'a Cell<usize>,
        weak: &'a Cell<usize>,
    }

    impl<'a> WeakInner<'a> {
        fn inc_weak(&self) {
            self.weak.set(self.weak.get().wrapping_add(1));
            if self.weak.get() == 0 {
                panic!("overflow");
            }
        }

        fn dec_weak(&self) {
            self.weak.set(self.weak.get() - 1);
        }
    }

    pub struct WeakCirc<T> {
        ptr: NonNull<CircInner<T>>,
    }

    impl<T> WeakCirc<T> {
        fn inner(&self) -> WeakInner {
            unsafe {
                let ptr = self.ptr.as_ptr();
                WeakInner {
                    strong: &(*ptr).strong,
                    weak: &(*ptr).weak,
                }
            }
        }
    }

    impl<T> WeakCircTrait<T> for WeakCirc<T> {
        type Strong = Circ<T>;

        fn upgrade(&self) -> Option<Circ<T>> {
            if Self::strong_count(self) == 0 {
                return None;
            }
            let inner = unsafe { self.ptr.as_ref() };
            inner.inc_strong();
            Some(Circ {
                ptr: self.ptr,
                _marker: PhantomData,
            })
        }

        fn strong_count(this: &Self) -> usize {
            this.inner().strong.get()
        }

        fn weak_count(this: &Self) -> usize {
            if Self::strong_count(this) == 0 {
                this.inner().weak.get()
            } else {
                this.inner().weak.get() - 1
            }
        }

        fn ptr_eq(this: &Self, other: &Self) -> bool {
            this.ptr == other.ptr
        }
    }

    impl<T> Clone for WeakCirc<T> {
        fn clone(&self) -> Self {
            self.inner().inc_weak();
            WeakCirc { ptr: self.ptr }
        }
    }

    impl<T> Drop for WeakCirc<T> {
        fn drop(&mut self) {
            let inner = self.inner();
            inner.dec_weak();
            if inner.weak.get() == 0 {
                let _ = unsafe { Box::from_raw(self.ptr.as_ptr()) };
            }
        }
    }
}

trait CricTrait<T>: AsRef<T> + std::ops::Deref<Target = T> + Clone {
    /// The weak reference type. It should be `WeakCirc<T>`.
    type Weak;

    /// Create a new reference-counted pointer.
    ///
    /// This function will take ownership of the data and return a new
    /// reference-counted pointer to it. The data will be automatically
    /// deallocated when the last reference-counted pointer pointing to it is
    /// dropped.
    ///
    /// For a newly created reference-counted pointer, the strong count should
    /// be initialized to 1, and the weak count should be set to 0.
    ///
    /// The `Circ<T>`` type should exhibit covariance over `T``. This implies
    /// that if you have a type `U`` that is a subtype of `T``, you should be
    /// able to pass a `Circ<U>` into a function that expects a `Circ<T>`.
    ///
    /// Moreover, the `Circ<T>` type should support niche optimization. This
    /// implies that an `Option<Circ<T>>` should have the same size as `Circ<T>`,
    /// which is the size of a single pointer. The `WeakCirc<T>` type should
    /// satisfy the same requirement.
    fn new(data: T) -> Self;

    /// Create a new reference-counted pointer with cyclic references.
    ///
    /// This function bears similarities to `new`, but it permits the data to
    /// reference the reference-counted pointer itself. To avoid the occurrence
    /// of reference cycles, the self-reference is provided as a weak pointer.
    ///
    /// The `data_fn` function is invoked with a weak reference and should
    /// return the data. During the execution of `data_fn`, the strong count of
    /// the reference-counted pointer is 0. Consequently, any attempt to upgrade
    /// the weak reference will yield `None`.
    fn new_cyclic(data_fn: impl FnOnce(&Self::Weak) -> T) -> Self;

    /// Downgrade this reference-counted pointer to a weak reference.
    ///
    /// A weak reference does not contribute to the strong count. When the
    /// strong count of the data drops to 0, the data is dropped, and all weak
    /// references are invalidated.
    fn downgrade(this: &Self) -> Self::Weak;

    /// Get the number of strong references to this data.
    fn strong_count(this: &Self) -> usize;

    /// Get the number of weak references to this data.
    fn weak_count(this: &Self) -> usize;

    /// Compare two reference-counted pointers to see if they point to the same
    /// data.
    ///
    /// This function returns `true` if the two reference-counted pointers are
    /// equal, i.e. they are cloned from the same reference-counted pointer.
    fn ptr_eq(this: &Self, other: &Self) -> bool;

    /// Make a mutable reference to the data, cloning it if necessary.
    ///
    /// This function should adhere to the "clone-on-write" policy. Post
    /// invocation, the reference-counted pointer should hold exclusive access
    /// to the data (i.e. the strong count is 1, and the weak count is 0).
    ///
    /// To ensure this, initially verify the strong and weak counts of the
    /// reference-counted pointer. If the strong count exceeds 1, clone the data
    /// and reassign the reference-counted pointer to the newly cloned data. If
    /// the weak count is more than 0, invalidate all existing weak references
    /// to the data.
    fn make_mut(this: &mut Self) -> &mut T
    where
        T: Clone;
}

trait WeakCircTrait<T>: Clone {
    /// The strong reference type. It should be `Circ<T>`.
    type Strong;

    /// Try to upgrade the weak reference to a strong reference.
    ///
    /// This function returns `Some` if there are still strong references to the
    /// data. Otherwise, it returns `None`.
    fn upgrade(&self) -> Option<Self::Strong>;

    /// Get the number of strong references to the data.
    fn strong_count(this: &Self) -> usize;

    /// Get the number of weak references to the data.
    fn weak_count(this: &Self) -> usize;

    /// Compare two weak references to see if they point to the same data.
    fn ptr_eq(this: &Self, other: &Self) -> bool;
}

fn main() {
    use circ::{Circ, WeakCirc};
    use std::{cell::Cell, ops::Deref};

    fn test_trait_impl() {
        trait RcTrait<T>: AsRef<T> + Deref<Target = T> + Clone {}
        impl<T> RcTrait<T> for Circ<T> {}
        fn test_rc_trait<T: RcTrait<i32>>(rc: T) {
            let i = *rc;
            assert_eq!(rc.as_ref(), &i);
        }
        test_rc_trait(Circ::new(5));
    }
    test_trait_impl();
    println!("Trait implementation passed");

    fn test_covariance() {
        fn stringify<'a>(c: Circ<&'a str>, _tie: &'a i32) -> String {
            c.as_ref().to_string()
        }
        let s: Circ<&'static str> = Circ::new("hello");
        let r: i32 = 5;
        let ss = stringify(s, &r);
        assert_eq!(&ss, "hello");
    }
    test_covariance();
    println!("Covariance passed");

    fn test_size() {
        use std::mem::size_of;
        assert_eq!(size_of::<Circ<i32>>(), size_of::<usize>());
        assert_eq!(size_of::<WeakCirc<i32>>(), size_of::<usize>());
        assert_eq!(size_of::<Option<Circ<i32>>>(), size_of::<usize>());
        assert_eq!(size_of::<Option<WeakCirc<i32>>>(), size_of::<usize>());
    }
    test_size();
    println!("Size passed");

    fn test_ptr_eq() {
        let a = Circ::new(5);
        let b = Circ::clone(&a);
        let c = Circ::new(5);

        assert!(Circ::ptr_eq(&a, &a));
        assert!(Circ::ptr_eq(&a, &b));
        assert!(!Circ::ptr_eq(&a, &c));
    }
    test_ptr_eq();
    println!("Pointer equality passed");

    fn test_sharing() {
        let a = Circ::new(Cell::new(5));
        let b = Circ::clone(&a);
        let c = Circ::downgrade(&a);

        assert_eq!(a.as_ref().get(), 5);
        assert_eq!(b.as_ref().get(), 5);
        assert_eq!(c.upgrade().unwrap().as_ref().get(), 5);

        a.as_ref().set(75);

        assert_eq!(a.as_ref().get(), 75);
        assert_eq!(b.as_ref().get(), 75);
        assert_eq!(c.upgrade().unwrap().as_ref().get(), 75);
    }
    test_sharing();
    println!("Sharing passed");

    fn test_drop() {
        let drop_flag = std::rc::Rc::new(Cell::new(false));

        struct Test {
            drop_flag: std::rc::Rc<Cell<bool>>,
        }

        impl Drop for Test {
            fn drop(&mut self) {
                self.drop_flag.set(true);
            }
        }

        let a = Circ::new(Test {
            drop_flag: drop_flag.clone(),
        });
        assert!(!drop_flag.get());

        let b = Circ::clone(&a);
        assert!(!drop_flag.get());

        let c = Circ::downgrade(&a);
        assert!(!drop_flag.get());

        let d = Circ::downgrade(&a);
        assert!(!drop_flag.get());

        drop(a);
        assert!(!drop_flag.get());

        drop(d);
        assert!(!drop_flag.get());

        drop(b);
        assert!(drop_flag.get());

        drop(c);
        assert!(drop_flag.get());
    }
    test_drop();
    println!("Drop passed");

    fn test_make_mut() {
        let mut data = Circ::new(5);
        *Circ::make_mut(&mut data) += 1;
        assert_eq!(*data, 6);

        let mut other = Circ::clone(&data);
        assert_eq!(*other, 6);

        *Circ::make_mut(&mut data) += 1;
        assert_eq!(*data, 7);
        assert_eq!(*other, 6);

        *Circ::make_mut(&mut data) += 1;
        assert_eq!(*data, 8);
        assert_eq!(*other, 6);

        *Circ::make_mut(&mut other) *= 2;
        assert_eq!(*data, 8);
        assert_eq!(*other, 12);

        drop(other);

        let weak = Circ::downgrade(&data);
        assert_eq!(*data, 8);
        assert_eq!(*weak.upgrade().unwrap(), 8);

        *Circ::make_mut(&mut data) += 2;

        assert_eq!(*data, 10);
        assert!(weak.upgrade().is_none());
    }
    test_make_mut();
    println!("Make mut passed");

    fn test_strong_weak_count() {
        let a = Circ::new(75);
        assert_eq!(Circ::strong_count(&a), 1);
        assert_eq!(Circ::weak_count(&a), 0);

        let b = Circ::downgrade(&a);
        assert_eq!(Circ::strong_count(&a), 1);
        assert_eq!(Circ::weak_count(&a), 1);
        assert_eq!(WeakCirc::strong_count(&b), 1);
        assert_eq!(WeakCirc::weak_count(&b), 1);

        let c = Circ::downgrade(&a);
        assert_eq!(Circ::strong_count(&a), 1);
        assert_eq!(Circ::weak_count(&a), 2);
        assert_eq!(WeakCirc::strong_count(&b), 1);
        assert_eq!(WeakCirc::weak_count(&b), 2);
        assert_eq!(WeakCirc::strong_count(&c), 1);
        assert_eq!(WeakCirc::weak_count(&c), 2);

        let d = Circ::clone(&a);
        assert_eq!(Circ::weak_count(&a), 2);
        assert_eq!(Circ::weak_count(&d), 2);
        assert_eq!(WeakCirc::strong_count(&b), 2);

        drop(b);
        assert_eq!(Circ::weak_count(&a), 1);
        assert_eq!(Circ::weak_count(&d), 1);
        assert_eq!(WeakCirc::weak_count(&c), 1);

        drop(a);
        assert_eq!(Circ::strong_count(&d), 1);
        assert_eq!(Circ::weak_count(&d), 1);
        assert_eq!(WeakCirc::strong_count(&c), 1);

        drop(d);
        assert_eq!(WeakCirc::strong_count(&c), 0);
        assert_eq!(WeakCirc::weak_count(&c), 1);
    }
    test_strong_weak_count();
    println!("Strong weak count passed");

    fn test_weak_ptr_eq() {
        let a = Circ::new(75);
        let b = Circ::downgrade(&a);
        let c = Circ::downgrade(&a);
        let d = Circ::downgrade(&Circ::new(75));

        assert!(WeakCirc::ptr_eq(&b, &b));
        assert!(WeakCirc::ptr_eq(&b, &c));
        assert!(!WeakCirc::ptr_eq(&b, &d));
    }
    test_weak_ptr_eq();
    println!("Weak pointer equality passed");

    fn test_cyclic() {
        let drop_flag = std::rc::Rc::new(Cell::new(false));

        struct Cyclic {
            data: i32,
            drop_flag: std::rc::Rc<Cell<bool>>,
            me: WeakCirc<Cyclic>,
        }

        impl Drop for Cyclic {
            fn drop(&mut self) {
                self.drop_flag.set(true);
            }
        }

        let a = Circ::new_cyclic(|weak| {
            assert!(weak.upgrade().is_none());
            Cyclic {
                data: 75,
                drop_flag: drop_flag.clone(),
                me: WeakCirc::clone(weak),
            }
        });
        let b = Circ::downgrade(&a).upgrade().unwrap();
        assert_eq!(b.data, 75);

        let c = a.me.upgrade().unwrap();
        assert_eq!(c.data, 75);

        drop(a);
        assert!(!drop_flag.get());

        drop(b);
        assert!(!drop_flag.get());

        drop(c);
        assert!(drop_flag.get());
    }
    test_cyclic();
    println!("Cyclic passed");
}
