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
        self.head = Self::new_node(element, self.head.take());
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();
        if let Some(node) = head {
            self.head = node.next;
            Some(node.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    fn new_node(data: T, next: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        Some(Box::new(Node { data, next }))
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut next = &self.head;
        let mut result = Self::new();

        while let Some(node) = &next {
            let previous_head = result.head;
            result.head = Self::new_node(node.data.clone(), previous_head);
            next = &node.next;
        }
        result
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(slice: &[T]) -> Self {
        let mut result = Self::new();
        for item in slice.iter() {
            result.push(item.clone());
        }
        result
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut next = self.head;
        let mut result: Vec<T> = vec![];
        while let Some(node) = next {
            result.push(node.data);
            next = node.next;
        }
        result.reverse();
        result
    }
}
