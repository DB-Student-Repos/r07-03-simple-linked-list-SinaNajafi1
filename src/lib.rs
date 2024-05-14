use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None}
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut current_head = &self.head; 
        let mut num_of_element = 0;
        while let Some(e) = current_head {
            num_of_element +=1;
            current_head = &e.next;
        }
        num_of_element
    }

    pub fn push(&mut self, element: T) {
        let node = Node { data: element, next: self.head.take() };
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_some() {
            let head_node = self.head.take().unwrap();
            self.head = head_node.next;
            Some(head_node.data)
        }
        else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &(head.data))
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut sample = SimpleLinkedList::new();
        while let Some(e) = self.pop() {
            sample.push(e);
        }
        sample
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_list = SimpleLinkedList::new();
        for new_element in iter {
            new_list.push(new_element);
        }
        new_list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut new_vec = Vec::new();
        while let Some(e) = linked_list.pop() {
            new_vec.push(e);
        }
        new_vec.reverse();
        new_vec
    }
}
