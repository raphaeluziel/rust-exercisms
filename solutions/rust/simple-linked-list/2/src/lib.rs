pub struct SimpleLinkedList<T> {
    data: Vec<(T, *const T)>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, _element: T) {
        // Set the last pointer, currently null to point to the next element
        // that will be pushed into the SimpleLinkedList
        if !self.is_empty() {
            self.data.last_mut().unwrap().1 = std::ptr::from_ref(&_element);
        }

        // Push the element into the SimpleLinkedList, setting the pointer to
        // null since this will be the last link
        self.data.push((_element, std::ptr::null()));
    }

    pub fn pop(&mut self) -> Option<T> {
        // If the SimpleLinkedList is empty, there's nothing to pop out
        if self.is_empty() {
            None
        } 
        else {
            // Pop out the last element, and save it to return
            let popped = self.data.pop().unwrap().0;

            // If after popping out the last element, the SimpleLinkedList
            // is still not empty, then set the now last element's pointer
            // to null
            if !self.is_empty() {
                self.data.last_mut().unwrap().1 = std::ptr::null();
            }

            // Return the previously popped element
            Some(popped)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        // If the SimpleLinkedList is empty, there's nothing to peek
        if self.data.is_empty() {
            None
        }
        // Otherwise, return the last element, so it can be peeked
        else {
            Some(&self.data.last().unwrap().0)
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut rsll: SimpleLinkedList<T> = SimpleLinkedList::new();

        // First reverse the list.  This will mean that while the actual
        // values in the list are reversed, the pointers are no longer valid.
        let mut reversed_list = self.data;
        reversed_list.reverse();

        // Now push the reversed values into the SimpleLinkedList to re-create
        // valid pointers
        for reversed_element in reversed_list {
            rsll.push(reversed_element.0);
        }

        rsll
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut sll: SimpleLinkedList<T> = SimpleLinkedList::new();

        // Pushing the elements will call MY push() method, which
        // will add valid pointers to the list.
        for i in _iter {
            sll.push(i);
        }

        sll
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        _linked_list
            .data
            .into_iter()
            .map(|(element, _pointer)| element)
            .collect()
    }
}
