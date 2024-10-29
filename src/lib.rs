//! Define the [IterPeekEnd] trait that work on peekable iterator,
//! to know if the current element is the last one of the iterator.
//! 
//! ```rust
//! pub trait IterPeekEnd
//! { 
//!     fn is_last(&mut self) -> bool;
//!     fn is_not_last(&mut self) -> bool { !self.is_last() }
//! }
//! ```
//! 
//! Useful to use when iterating for doing some processing between each value of an iterator :
//! 
//! ```rust
//! use iter_peek_end::*;
//! 
//! let my_vec = vec![1, 2, 3];
//! let mut it = my_vec.iter().peekable();
//!         
//! while let Some(v) = it.next()
//! {
//!     print!("{}", v);
//!     if it.is_not_last() 
//!     {
//!         print!(", ");
//!     }
//! }
//! ```
//! will display => `1, 2, 3`
//! 
//! Notice the lack of comma after the three / the last element.
//! 
//! Based on [@ctrl-alt-delor anwser on StackOverflow : How to check if for loop is on the last element of an iterator?](https://stackoverflow.com/a/73355481/12048568), thank you !


/// Define `is_last()` and `is_not_last()` on a peekable iterator
/// 
/// ```
/// use iter_peek_end::*;
/// 
/// let my_vec = vec![1, 2, 3];
/// let mut it = my_vec.iter().peekable();
/// 
/// assert_eq!(it.is_last(), false);
/// assert_eq!(it.is_not_last(), !it.is_last());
///
/// assert_eq!(it.next(), Some(&1));
/// assert_eq!(it.is_last(), false);
/// assert_eq!(it.is_not_last(), !it.is_last());
///
/// assert_eq!(it.next(), Some(&2));
/// assert_eq!(it.is_last(), false);
/// assert_eq!(it.is_not_last(), !it.is_last());
/// assert_eq!(it.next(), Some(&3));
///
/// assert_eq!(it.is_last(), true);
/// assert_eq!(it.is_not_last(), !it.is_last());
/// assert_eq!(it.next(), None);
/// ```
/// 
/// Based on [@ctrl-alt-delor anwser on StackOverflow : How to check if for loop is on the last element of an iterator?](https://stackoverflow.com/a/73355481/12048568), thank you !
pub trait IterPeekEnd
{ 
    /// True is the current element is the last one of the iterator
    /// 
    /// ```
    /// use iter_peek_end::*;
    /// 
    /// let my_vec = vec![1, 2, 3];
    /// let mut it = my_vec.iter().peekable();
    /// 
    /// assert_eq!(it.is_last(), false);
    ///
    /// assert_eq!(it.next(), Some(&1));
    /// assert_eq!(it.is_last(), false);
    ///
    /// assert_eq!(it.next(), Some(&2));
    /// assert_eq!(it.is_last(), false);
    /// 
    /// assert_eq!(it.next(), Some(&3));
    /// assert_eq!(it.is_last(), true);
    /// 
    /// assert_eq!(it.next(), None);
    /// assert_eq!(it.is_last(), true);
    /// ```
    fn is_last(&mut self) -> bool;

    /// True is the current element is **not** the last one of the iterator.
    /// Will always be different than `is_last()`.
    /// 
    /// ```
    /// use iter_peek_end::*;
    /// 
    /// let my_vec = vec![1, 2, 3];
    /// let mut it = my_vec.iter().peekable();
    /// 
    /// assert_eq!(it.is_not_last(), true);
    ///
    /// assert_eq!(it.next(), Some(&1));
    /// assert_eq!(it.is_not_last(), true);
    ///
    /// assert_eq!(it.next(), Some(&2));
    /// assert_eq!(it.is_not_last(), true);
    /// 
    /// assert_eq!(it.next(), Some(&3));
    /// assert_eq!(it.is_not_last(), false);
    /// 
    /// assert_eq!(it.next(), None);
    /// assert_eq!(it.is_not_last(), false);
    /// ```
    fn is_not_last(&mut self) -> bool { !self.is_last() }
}
impl<I: Iterator> IterPeekEnd for  std::iter::Peekable<I> 
{
    fn is_last(&mut self) -> bool { self.peek().is_none() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        let my_vec = vec![1, 2, 3];
        let mut it = my_vec.iter().peekable();

        assert_eq!(it.is_last(), false);
        assert_eq!(it.is_not_last(), !it.is_last());

        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.is_last(), false);
        assert_eq!(it.is_not_last(), !it.is_last());

        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.is_last(), false);
        assert_eq!(it.is_not_last(), !it.is_last());
        assert_eq!(it.next(), Some(&3));

        assert_eq!(it.is_last(), true);
        assert_eq!(it.is_not_last(), !it.is_last());
        assert_eq!(it.next(), None);
    }
}
