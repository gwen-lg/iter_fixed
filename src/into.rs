use crate::IteratorFixed;

use core::{array, iter, slice};

/// Conversion into an [`IteratorFixed`].
///
/// By implementing `IntoIteratorFixed` for a type, you define how it will be
/// converted to an iterator of fixed size.
///
/// See also: [`crate::FromIteratorFixed`].
///
/// # Safety
/// Implementer has to guarantee that the inner iterator will always yield exactly N elements
pub unsafe trait IntoIteratorFixed<const N: usize> {
    /// The type of the elements being iterated over.
    type Item;

    /// What will be the underlaying iterator for the [`IteratorFixed`] that we are turning this into?
    type IntoIter: Iterator<Item = Self::Item>;

    /// Creates a fixed size iterator from a value.
    ///
    /// Basic usage:
    /// ```
    /// use iter_fixed::IntoIteratorFixed;
    ///
    /// let two_four_six = [1, 2, 3].into_iter_fixed().map(|x| 2 * x);
    ///
    /// let a: [i32; 3] = two_four_six.collect();
    /// assert_eq!(a, [2, 4, 6]);
    /// ```
    fn into_iter_fixed(self) -> IteratorFixed<Self::IntoIter, N>;
}

// IteratorFixed implements IntoIteratorFixed
unsafe impl<I: Iterator, const N: usize> IntoIteratorFixed<N> for IteratorFixed<I, N>
where
    Self: IntoIterator,
{
    type Item = I::Item;
    type IntoIter = I;

    /// `IteratorFixed` implements `IntoIteratorFixed` as a no op. This allows passing an
    /// `IteratorFixed` where an `IntoIteratorFixed` was expected
    ///
    /// Basic usage with zip which expects an `IntoIteratorFixed` as its argument:
    /// ```
    /// use iter_fixed::IntoIteratorFixed;
    /// let one_one = [1, 1].into_iter_fixed();
    /// let zipped: [_; 2] = [1, 2].into_iter_fixed().zip(one_one).collect();
    ///
    /// assert_eq!(zipped, [(1, 1), (2, 1)]);
    /// ```
    #[inline]
    fn into_iter_fixed(self) -> IteratorFixed<Self::IntoIter, N> {
        self
    }
}

unsafe impl<T, const N: usize> IntoIteratorFixed<N> for [T; N] {
    type Item = T;
    type IntoIter = array::IntoIter<T, N>;

    /// Creates a fixed size iterator from an array.
    ///
    /// Basic usage:
    /// ```
    /// use iter_fixed::IntoIteratorFixed;
    ///
    /// let two_four_six = [1, 2, 3].into_iter_fixed().map(|x| 2 * x);
    ///
    /// let a: [i32; 3] = two_four_six.collect();
    /// assert_eq!(a, [2, 4, 6]);
    /// ```
    #[inline]
    fn into_iter_fixed(self) -> IteratorFixed<array::IntoIter<T, N>, N> {
        // Safety: array::IntoIter::new([T; N]) always yields N elements
        unsafe { IteratorFixed::from_iter(<[T; N] as IntoIterator>::into_iter(self)) }
    }
}

unsafe impl<'a, T, const N: usize> IntoIteratorFixed<N> for &'a [T; N] {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    /// Creates a fixed size iterator from a borrowed array.
    ///
    /// Basic usage:
    /// ```
    /// use iter_fixed::IntoIteratorFixed;
    /// fn double<const N: usize>(array: &[i32; N]) -> [i32; N] {
    ///     let two_four_six = array.into_iter_fixed().map(|&x| 2 * x);
    ///
    ///     two_four_six.collect()
    /// }
    /// assert_eq!(double(&[1, 2, 3]), [2, 4, 6]);
    /// ```
    #[inline]
    fn into_iter_fixed(self) -> IteratorFixed<Self::IntoIter, N> {
        // Safety: [T; N]::iter always yields N elements
        unsafe { IteratorFixed::from_iter(self.iter()) }
    }
}

unsafe impl<'a, T, const N: usize> IntoIteratorFixed<N> for &'a mut [T; N] {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    /// Creates a fixed size iterator from a mutably borrowed array.
    ///
    /// Basic usage:
    /// ```
    /// use iter_fixed::IntoIteratorFixed;
    ///
    /// // Update `array` in place to element wise x * 2
    /// // return the initial value of the array
    /// fn double<const N: usize>(array: &mut [i32; N]) -> [i32; N] {
    ///     let fixed_iter = array.into_iter_fixed().map(|x| {
    ///         let old = *x;
    ///         *x = 2 * *x;
    ///         old
    ///     });
    ///     fixed_iter.collect()
    /// }
    /// let mut a = [1, 2, 3];
    /// assert_eq!(double(&mut a), [1, 2, 3]);
    /// assert_eq!(a, [2, 4, 6]);
    /// ```
    #[inline]
    fn into_iter_fixed(self) -> IteratorFixed<Self::IntoIter, N> {
        // Safety: [T; N]::iter always yields N elements
        unsafe { IteratorFixed::from_iter(self.iter_mut()) }
    }
}

unsafe impl<T: Clone, const N: usize> IntoIteratorFixed<N> for iter::Repeat<T> {
    type Item = T;
    type IntoIter = iter::Take<Self>;

    /// Creates a fixed size iterator from an [`core::iter::Repeat`]
    ///
    /// Basic usage:
    /// ```
    /// use core::iter;
    /// use iter_fixed::IntoIteratorFixed;
    ///
    /// let one_one_one = iter::repeat(1).into_iter_fixed();
    ///
    /// let a: [i32; 3] = one_one_one.collect();
    /// assert_eq!(a, [1, 1, 1]);
    /// ```
    #[inline]
    fn into_iter_fixed(self) -> IteratorFixed<iter::Take<Self>, N> {
        // Safety: iter::repeat(_).take(N) always yields N elements
        unsafe { IteratorFixed::from_iter(self.take(N)) }
    }
}
