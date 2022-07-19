use core::iter::FusedIterator;
use crate::prelude::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::rc::Rc;
        use std::collections::VecDeque;
        use std::sync::{Arc, Mutex, MutexGuard, TryLockError};
    } else if #[cfg(feature = "alloc")] {
        extern crate alloc;
        use alloc::rc::Rc;
        use alloc::collections::VecDeque;
    }
}

impl<A: Iterator, B: Iterator> Iterator for Either<A, B> {
    type Item = Either<A::Item, B::Item>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Left(x) => x.next().map(Left),
            Right(x) => x.next().map(Right)
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Left(x) => x.size_hint(),
            Right(x) => x.size_hint()
        }
    }
}

impl<A: ExactSizeIterator, B: ExactSizeIterator> ExactSizeIterator for Either<A, B> {
    #[inline(always)]
    fn len(&self) -> usize {
        match self {
            Left(x) => x.len(),
            Right(x) => x.len()
        }
    }
}

impl<A: DoubleEndedIterator, B: DoubleEndedIterator> DoubleEndedIterator for Either<A, B> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Left(x) => x.next_back().map(Left),
            Right(x) => x.next_back().map(Right)
        }
    }
}

impl<A: FusedIterator, B: FusedIterator> FusedIterator for Either<A, B> {}

#[repr(transparent)]
pub struct LRIter<A, B> (Either<A, B>);

impl<T, A: Iterator<Item = T>, B: Iterator<Item = T>> Either<A, B> {
    /// Turns an [`Either`] object with iterators with the same [`Iterator::Item`] into an iterator
    #[inline(always)]
    pub fn into_same_iter (self) -> LRIter<A, B> {
        LRIter::new(self)
    }
}

impl<T, A: Iterator<Item = T>, B: Iterator<Item = T>> LRIter<A, B> {
    #[inline(always)]
    pub const fn new (iter: Either<A,B>) -> Self {
        Self(iter)
    }

    #[inline(always)]
    pub fn into_inner (self) -> Either<A, B> {
        self.0
    }
}

impl<T, A: Iterator<Item = T>, B: Iterator<Item = T>> Iterator for LRIter<A, B> {
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Left(x) => x.next(),
            Right(x) => x.next()
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.0 {
            Left(x) => x.size_hint(),
            Right(x) => x.size_hint()
        }
    }
}

impl<T, A: ExactSizeIterator<Item = T>, B: ExactSizeIterator<Item = T>> ExactSizeIterator for LRIter<A, B> {
    #[inline(always)]
    fn len(&self) -> usize {
        match &self.0 {
            Left(x) => x.len(),
            Right(x) => x.len()
        }
    }
}

impl<T, A: DoubleEndedIterator<Item = T>, B: DoubleEndedIterator<Item = T>> DoubleEndedIterator for LRIter<A, B> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Left(x) => x.next_back(),
            Right(x) => x.next_back()
        }
    }
}

/// An iterator with ```Either``` items
pub trait EitherIter<A, B>: Sized + Iterator<Item = Either<A, B>> {
    /// Returns two seperate iterators that return the left and right values seperately.
    /// This iterators are **not thread-safe**, but maintain item order and have less overhead than their thread-safe counterparts.
    #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[inline(always)]
    fn split_either (self) -> (SplitLeft<Self, A, B>, SplitRight<Self, A, B>) {
        let init_cap = match self.size_hint() {
            (_, Some(max)) => max,
            (min, None) => min.max(7)
        };

        let inner = Rc::new(RefCell::new(self));
        let left = Rc::new(RefCell::new(VecDeque::with_capacity(init_cap)));
        let right = Rc::new(RefCell::new(VecDeque::with_capacity(init_cap)));

        (
            SplitLeft {
                inner: inner.clone(),
                this: left.clone(),
                other: right.clone(),
            },

            SplitRight {
                inner,
                this: right,
                other: left,
            }
        )
    }

    /// Returns two seperate iterators that return the left and right values seperately.
    /// This iterators are **thread-safe**, but may not maintain item order and add more overhead than their non-thread-safe counterparts.
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    #[cfg(feature = "std")]
    #[inline(always)]
    fn split_either_arc (self) -> (SplitLeftArc<Self, A, B>, SplitRightArc<Self, A, B>) {
        let init_cap = match self.size_hint() {
            (_, Some(max)) => max,
            (min, None) => min.max(7)
        };

        let inner = Arc::new(Mutex::new(self));
        let left = Arc::new(Mutex::new(VecDeque::with_capacity(init_cap)));
        let right = Arc::new(Mutex::new(VecDeque::with_capacity(init_cap)));

        (
            SplitLeftArc {
                inner: inner.clone(),
                this: left.clone(),
                other: right.clone(),
            },

            SplitRightArc {
                inner,
                this: right,
                other: left,
            }
        )
    }
}

impl<I: Iterator<Item = Either<A, B>>, A, B> EitherIter<A, B> for I {}

cfg_if::cfg_if! {
    if #[cfg(any(feature = "std", feature = "alloc"))] {
        use core::cell::RefCell;
        
        /// Iterator that returns the left values of an iterator of Either values.
        /// This iterator is **not thread-safe**, but maintains item order and has less overhead than ```SplitLeftArc```.
        #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
        #[derive(Debug)]
        pub struct SplitLeft<I, A, B> {
            inner: Rc<RefCell<I>>,
            this: Rc<RefCell<VecDeque<A>>>,
            other: Rc<RefCell<VecDeque<B>>>
        }

        impl<I: Iterator<Item = Either<A, B>>, A, B> Iterator for SplitLeft<I, A, B> {
            type Item = A;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if let Some(next) = self.this.borrow_mut().pop_front() { 
                    return Some(next); 
                }

                while let Some(next) = self.inner.borrow_mut().next() {
                    match next {
                        Left(value) => return Some(value),
                        Right(value) => self.other.borrow_mut().push_back(value)
                    }
                }

                None
            }

            #[inline(always)]
            fn size_hint (&self) -> (usize, Option<usize>) {
                let (mut min, mut max) = self.inner.borrow().size_hint();
                let len = self.this.borrow().len();

                min += len;
                if let Some(ref mut max) = max {
                    *max += len
                }
                
                (min, max)
            }
        }

        impl<I, A, B> Clone for SplitLeft<I, A, B> {
            #[inline(always)]
            fn clone(&self) -> Self {
                Self { inner: self.inner.clone(), this: self.this.clone(), other: self.other.clone() }
            }
        }

        /// Iterator that returns the right values of an iterator of Either values.
        /// This iterator is **not thread-safe**, but maintains item order and has less overhead than ```SplitRightArc```.
        #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
        #[derive(Debug)]
        pub struct SplitRight<I, A, B> {
            inner: Rc<RefCell<I>>,
            this: Rc<RefCell<VecDeque<B>>>,
            other: Rc<RefCell<VecDeque<A>>>
        }

        impl<I: Iterator<Item = Either<A, B>>, A, B> Iterator for SplitRight<I, A, B> {
            type Item = B;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if let Some(next) = self.this.borrow_mut().pop_front() { 
                    return Some(next); 
                }

                while let Some(next) = self.inner.borrow_mut().next() {
                    match next {
                        Right(value) => return Some(value),
                        Left(value) => self.other.borrow_mut().push_back(value)
                    }
                }

                None
            }

            #[inline(always)]
            fn size_hint (&self) -> (usize, Option<usize>) {
                let (mut min, mut max) = self.inner.borrow().size_hint();
                let len = self.this.borrow().len();

                min += len;
                if let Some(ref mut max) = max {
                    *max += len
                }
                
                (min, max)
            }
        }

        impl<I, A, B> Clone for SplitRight<I, A, B> {
            #[inline(always)]
            fn clone(&self) -> Self {
                Self { inner: self.inner.clone(), this: self.this.clone(), other: self.other.clone() }
            }
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        /// Iterator that returns the left values of an iterator of Either values.
        /// This iterator is **thread-safe**, but may not maintain item order and adds more overhead than ```SplitLeft```.
        #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
        #[derive(Debug)]
        pub struct SplitLeftArc<I, A, B> {
            inner: Arc<Mutex<I>>,
            this: Arc<Mutex<VecDeque<A>>>,
            other: Arc<Mutex<VecDeque<B>>>
        }

        impl<I: Iterator<Item = Either<A, B>>, A, B> Iterator for SplitLeftArc<I, A, B> {
            type Item = A;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if let Some(mut this) = try_lock_deep(&self.this) {
                    if let Some(next) = this.pop_front() { 
                        return Some(next); 
                    }
                }

                let mut inner = lock_deep(&self.inner);
                let mut other = None;

                while let Some(next) = inner.next() {
                    match next {
                        Left(value) => return Some(value),
                        Right(value) => {
                            if other.is_none() {
                                other = Some(lock_deep(&self.other));
                            }

                            match other {
                                Some(ref mut other) => other.push_back(value),
                                _ => unsafe { core::hint::unreachable_unchecked() }
                            }
                        }
                    }
                }

                None
            }

            #[inline(always)]
            fn size_hint (&self) -> (usize, Option<usize>) {
                let inner = lock_deep(&self.inner);
                let (mut min, mut max) = inner.size_hint();
                drop(inner);

                let this = lock_deep(&self.this);
                let len = this.len();
                drop(this);

                min += len;
                if let Some(ref mut max) = max {
                    *max += len
                }
                
                (min, max)
            }
        }

        impl<I, A, B> Clone for SplitLeftArc<I, A, B> {
            #[inline(always)]
            fn clone(&self) -> Self {
                Self { inner: self.inner.clone(), this: self.this.clone(), other: self.other.clone() }
            }
        }

        /// Iterator that returns the right values of an iterator of Either values.
        /// This iterator is **thread-safe**, but may not maintain item order and adds more overhead than ```SplitRight```.
        #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
        #[derive(Debug)]
        pub struct SplitRightArc<I, A, B> {
            inner: Arc<Mutex<I>>,
            this: Arc<Mutex<VecDeque<B>>>,
            other: Arc<Mutex<VecDeque<A>>>
        }

        impl<I: Iterator<Item = Either<A, B>>, A, B> Iterator for SplitRightArc<I, A, B> {
            type Item = B;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if let Some(mut this) = try_lock_deep(&self.this) {
                    if let Some(next) = this.pop_front() { 
                        return Some(next); 
                    }
                }

                let mut inner = lock_deep(&self.inner);
                let mut other = None;

                while let Some(next) = inner.next() {
                    match next {
                        Right(value) => return Some(value),
                        Left(value) => {
                            if other.is_none() {
                                other = Some(lock_deep(&self.other));
                            }

                            match other {
                                Some(ref mut other) => other.push_back(value),
                                _ => unsafe { core::hint::unreachable_unchecked() }
                            }
                        }
                    }
                }

                None
            }

            #[inline(always)]
            fn size_hint (&self) -> (usize, Option<usize>) {
                let inner = lock_deep(&self.inner);
                let (mut min, mut max) = inner.size_hint();
                drop(inner);

                let this = lock_deep(&self.this);
                let len = this.len();
                drop(this);

                min += len;
                if let Some(ref mut max) = max {
                    *max += len
                }
                
                (min, max)
            }
        }

        impl<I, A, B> Clone for SplitRightArc<I, A, B> {
            #[inline(always)]
            fn clone(&self) -> Self {
                Self { inner: self.inner.clone(), this: self.this.clone(), other: self.other.clone() }
            }
        }

        #[inline(always)]
        fn lock_deep<T> (mutex: &Mutex<T>) -> MutexGuard<T> {
            match mutex.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner()
            }
        }

        #[inline(always)]
        fn try_lock_deep<T> (mutex: &Mutex<T>) -> Option<MutexGuard<T>> {
            match mutex.try_lock() {
                Ok(guard) => Some(guard),
                Err(TryLockError::Poisoned(e)) => Some(e.into_inner()),
                _ => None
            }
        }
    }
}