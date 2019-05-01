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
        Self::add_to_last(&mut self.head, element)
    }

    fn add_to_last(maybe_node: &mut Option<Box<Node<T>>>, element: T) {
        if let Some(node) = maybe_node {
            Self::add_to_last(&mut node.next, element);
        } else {
            *maybe_node = Self::new_node(element);
        }
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Clone,
    {
        Self::remove_from_last(&mut self.head)
    }

    fn remove_from_last(maybe_node: &mut Option<Box<Node<T>>>) -> Option<T>
    where
        T: Clone,
    {
        if let Some(node) = maybe_node {
            if node.next.is_some() {
                Self::remove_from_last(&mut node.next)
            } else {
                let value = node.data.clone();
                *maybe_node = None;
                Some(value)
            }
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            Some(&node.data)
        } else {
            None
        }
    }

    fn new_node(element: T) -> Option<Box<Node<T>>> {
        Some(Box::new(Node {
            data: element,
            next: None,
        }))
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut next = &self.head;
        let mut result = Self::new();

        while let Some(node) = &next {
            let previous_head = result.head;
            result.head = Some(Box::new(Node {
                data: node.data.clone(),
                next: previous_head,
            }));
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
        result
    }
}
