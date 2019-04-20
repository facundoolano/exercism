#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        let mut size: usize = 0;
        let mut next = &self.head;
        while let Some(node) = next {
            size += 1;
            next = &node.next;
        }
        size
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, element: T) {
        // FIXME pretty much duplicates add_to_last
        if let Some(node) = &mut self.head {
            Self::add_to_last(node, element)
        } else {
            self.head = Self::new_node(element);
        }
    }

    fn new_node(element: T) -> Option<Box<Node<T>>> {
        Some(Box::new(Node {
            data: element,
            next: None,
        }))
    }

    fn add_to_last(node: &mut Node<T>, element: T) {
        if let Some(next) = &mut node.next {
            Self::add_to_last(next, element);
        } else {
            node.next = Self::new_node(element);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            Some(&node.data)
        } else {
            None
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        unimplemented!()
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
