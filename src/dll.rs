#[derive(Debug)]
pub struct DllNode<T> {
    prev: PointerLink<T>,
    next: OwningLink<T>,
    payload: T,
}

type OwningLink<T> = Option<Box<DllNode<T>>>;
type PointerLink<T> = Option<*mut DllNode<T>>;

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    first: OwningLink<T>,
    last: PointerLink<T>,
}

#[derive(Debug)]
pub struct IntoIter<T>(DoublyLinkedList<T>);

#[derive(Debug)]
pub struct Iter<'a, T> {
    next: Option<&'a DllNode<T>>,
    prev: Option<&'a DllNode<T>>
}

#[derive(Debug)]
pub struct IterMut<'a, T> {
    next: Option<&'a mut DllNode<T>>,
    prev: Option<&'a mut DllNode<T>>,
}

// impl DllNode<T> {
// }

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList { first: None, last: None }
    }

    pub fn push_last(&mut self, payload: T) {
        let mut new_last_node = Box::new(DllNode {
            prev: self.last.clone(),
            next: None,
            payload: payload,
        });

        // Raw pointer must be constructed _before_ value is moved
        let raw_pointer_new_last_node: *mut _ = &mut *new_last_node;

        match self.last {
            None => { self.first = Some(new_last_node) } // empty list case
            Some(last_node) => unsafe { (*last_node).next = Some(new_last_node) }
        };

        self.last = Some(raw_pointer_new_last_node);
    }

    pub fn push_first(&mut self, payload: T) {
        let old_first_node = self.first.take();

        let mut new_first_node = Box::new(DllNode {
            prev: None,
            next: old_first_node,
            payload: payload,
        });

        match new_first_node.as_ref().next {
            Some(_) => {
                let raw_pointer_first_node: *mut _ = &mut *new_first_node;
                new_first_node.next.as_mut().unwrap().prev = Some(raw_pointer_first_node);
            },
            None => {
                let raw_pointer_new_last_node: *mut _ = &mut *new_first_node;
                self.last = Some(raw_pointer_new_last_node);
            }
        }

        self.first = Some(new_first_node);
    }

    pub fn pop_first(&mut self) -> Option<T> {
        self.first.take().map(|head| {
            let head = *head; // Take out of the box
            self.first = head.next; // Make list point to the second node
            let first_node = self.first.as_mut();
            if first_node.is_none() { // If list became empty
                self.last = None; // Then this pointed to the node we just took
            } else if first_node.unwrap().next.is_none() { // If list has only one element now
                self.first.as_mut().unwrap().prev = None;
            };
            head.payload
        })
    }

    pub fn pop_last(&mut self) -> Option<T> {
        self.last.map(|tail| { // None if empty list; Otherwise:
            if self.first.as_ref().unwrap().next.is_none() { // Single element case
                let tail_node = self.first.take();
                self.last = None;
                tail_node.unwrap().payload
            } else { // Multiple elements
                unsafe {
                    let tail_node = (*(*tail).prev.unwrap()).next.take();
                    self.last = Some((*tail).prev.unwrap());
                    tail_node.unwrap().payload
                }
            }
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.first.as_ref().map(|node| &**node),
            prev: self.last.as_ref().map(|node| unsafe { &**node } )
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.first.as_mut().map(|node| &mut **node),
            prev: self.last.as_mut().map(|node| unsafe { &mut **node })
        }
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.first.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_first()
    }
}
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_last()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {

        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.payload
        })
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.prev.map(|node| {
            self.prev = node.prev.as_ref().map(|node| unsafe { &**node } );
            &node.payload
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.payload
        })
    }
}
impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.prev.take().map(|node| {
            self.prev = node.prev.as_mut().map(|node| unsafe { &mut **node } );
            &mut node.payload
        })
    }
}

#[cfg(test)]
mod test {
    use super::DoublyLinkedList;

    #[test]
    fn basics() {
        let mut list = DoublyLinkedList::new();

        // Check empty list behaves right
        assert_eq!(list.pop_first(), None);

        // Populate list
        list.push_last(1);
        list.push_last(2);
        list.push_last(3);

        // Check normal removal
        assert_eq!(list.pop_first(), Some(1));
        assert_eq!(list.pop_first(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_last(4);
        list.push_last(5);

        // Check normal removal
        assert_eq!(list.pop_first(), Some(3));
        assert_eq!(list.pop_first(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_first(), Some(5));
        assert_eq!(list.pop_first(), None);

        // Check the exhaustion case fixed the pointer right
        list.push_last(6);
        list.push_last(7);

        // Check normal removal
        assert_eq!(list.pop_first(), Some(6));
        assert_eq!(list.pop_first(), Some(7));
        assert_eq!(list.pop_first(), None);

        // Check for pop/push in the backwards direction
        list.push_last(1);
        list.push_last(2);
        list.push_last(3);
        list.push_first(0);
        list.push_last(4);

        assert_eq!(list.pop_last(), Some(4));
        assert_eq!(list.pop_last(), Some(3));
        assert_eq!(list.pop_first(), Some(0));
        assert_eq!(list.pop_first(), Some(1));
        assert_eq!(list.pop_last(), Some(2));
        assert_eq!(list.pop_last(), None);
        assert_eq!(list.pop_last(), None);
        assert_eq!(list.pop_first(), None);

        // Test iteration both directions
        list.push_last(1);
        list.push_last(2);
        list.push_last(3);
        list.push_last(4);

        let mut list_rev = DoublyLinkedList::new();
        list_rev.push_first(1);
        list_rev.push_first(2);
        list_rev.push_first(3);
        list_rev.push_first(4);

        let mut expected = 1;
        for number in list.iter() {
            assert_eq!(*number, expected);
            expected += 1;
        }
        assert_eq!(expected, 5);

        let mut expected = 1;
        for number in list_rev.iter().rev() {
            assert_eq!(*number, expected);
            expected += 1;
        }
        assert_eq!(expected, 5);

        let mut expected = 1;
        for (number, number2) in list.iter().zip(list_rev.iter().rev()) {
            assert_eq!(*number, *number2);
            assert_eq!(*number, expected);
            expected += 1;
        }
        assert_eq!(expected, 5);
    }
}