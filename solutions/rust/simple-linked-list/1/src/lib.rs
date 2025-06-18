pub struct SimpleLinkedList<T> {
    data: Vec<T>,
    index: usize
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { data: Vec::new(), index: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, _element: T) {
        self.data.push(_element);
        self.index += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() { None }
        else {
            self.index -= 1;
            self.data.pop()
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.data.is_empty() { None }
        else { Some(&self.data[self.index - 1]) }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed_list = self.data;
        reversed_list.reverse();
        SimpleLinkedList { data: reversed_list, index: self.index }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut sll = SimpleLinkedList::new();
        for i in _iter {
            sll.data.push(i);
            sll.index += 1;
        }
        sll
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        _linked_list.data
    }
}
