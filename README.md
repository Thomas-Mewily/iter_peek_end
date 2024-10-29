Define the IterPeekEnd trait that work on peekable iterator,
to know if the current element is the last one of the iterator.

```rust
pub trait IterPeekEnd
{ 
    fn is_last(&mut self) -> bool;
    fn is_not_last(&mut self) -> bool { !self.is_last() }
}
```

Useful to use when iterating for doing some processing between each value of an iterator :

```rust
let my_vec = vec![1, 2, 3];
let mut it = my_vec.iter().peekable();
        
while let Some(v) = it.next()
{
    print!("{}", v);
    if it.is_not_last() 
    {
        print!(", ");
    }
}
```
will display => `1, 2, 3`

Notice the lack of comma after the last element.

Based on [@ctrl-alt-delor anwser on StackOverflow : How to check if for loop is on the last element of an iterator?](https://stackoverflow.com/a/73355481/12048568), thank you !
