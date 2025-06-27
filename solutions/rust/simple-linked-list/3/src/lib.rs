// Followed the following tutorial to help (that's an understatement!)
// https://rust-unofficial.github.io/too-many-lists

// Also relied on the Hints.md file!!!

// This just makes it easier so you don't have to type out Option<Box<Node<T>>>
type Link<T> = Option<Box<Node<T>>>;

// The SimpleLinkedList will hold the length of the list along with head, which
// is a pointer to the first Node in the list.  Note that it doesn't hold the
// actual list, since each element in the list will point to the next Node. 
pub struct SimpleLinkedList<T> {
    head: Link<T>,
    length: usize
}

// Each element in the list is a Node, which holds the actual data of the element
// along with a pointer to the next Node, which is just a Link
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, length: 0 }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Not sure if this is the best, but since I opted to keep a count of the Nodes
    // in the list, this operation is cheap.  I guess I could have opted to iterate
    // through the list, counting as I go, but that seemed much worse.
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        // Increase the size of the list, since we are adding a Node to it
        self.length += 1;

        // Create a new Node with the data, and set the next field to the 
        // current Node.  Note that the take() method takes the value from
        // self.head, places in next, and leaves None in its place.
        let new_node = Box::new(Node { data: element, next: self.head.take() });

        // Well, the previous line set self.head to None by the take() method,
        // but now that we've created the new node, we can make head point
        // to it, as opposed to the old node it was pointing to.
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // If there is a node in the list, we need to decrease the length
        // after popping it.
        if self.head.is_some() { self.length -= 1 }

        // Note that the map() here is not exactly the same as the map()
        // you're used to (or I'm used to).  That map() is for iterators
        // This one is specific to Option, and returns an Option.
        // Also note that in this map(), we are not iterating through
        // anything, we are simply replacing the Node we take() from
        // self.head (which leaves self.head pointing to None) with the
        // Node with the Node it WAS pointing to (self.next)
        self.head.take().map(|node| { self.head = node.next; node.data })
    }

    pub fn peek(&self) -> Option<&T> {
        // Again, this map() is for Options, which replaces &Box<Node<T>> 
        // (you get &Box<Node<T>> as opposed to Box<Node<T>> in the Option
        // because of as_ref()) with a reference to thje data, which is 
        // what we're looking to return.  We need the as_ref() because for
        // some reason I don't get yet, doing without it give a move out 
        // of reference error: 
        // "cannot move out of `self.head` which is behind a shared reference"
        self.head.as_ref().map(|x| &x.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        // Not sure if this is the best, but instead of dealing with handling
        // all the pointers to links when reversing the list, I figured I can
        // use the From trait function below to convert this list to a Vec,
        // then reverse it, then, since we've already implemented a FromIterator
        // trait as well below, we can create a SimpleLinkedList from an iterator.
        // The drain() method consumes the Vec and turns it into an iterator,
        // which can then be collected into a SimpleLinkedList. The collect()
        // method works because it relies on the FromIterator trait we implemented
        // see https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
        let mut v = Vec::from(self);
        v.reverse();
        v.drain(..).collect()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut sll = SimpleLinkedList::new();

        // All the hard stuff has been done.  To implement this trait now, we just
        // need to push each element in the iterator into the SimpleLinkedList
        for element in _iter { sll.push(element); }

        sll
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v = Vec::new();

        // To create a Vec from the SimpleLinkedList, we need to itereate through
        // each item in the list, until we reach the end.  The last element in
        // the SimpleLinkedList will be None.
        while _linked_list.head.is_some() {
            v.push(_linked_list.pop().unwrap());
        }

        // The Vec needs to be reversed because the last item in the 
        // SimpleLinkedList will be the first one popped
        v.reverse();
        
        v
    }
}
