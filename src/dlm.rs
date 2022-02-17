use std::ptr::{null_mut};

// Placement of the element in the original matrix
type Coordinates = [usize; 2];

type ElementPtr = *mut Element;

#[derive(Debug)]
struct ElementPtrPair {
    prev: ElementPtr,
    next: ElementPtr,
}

type Dimension = usize;
const ROW_DIM : Dimension = 0;
const COL_DIM : Dimension = 1;

enum Direction {
    Back(Dimension),
    Forward(Dimension),
}

use Direction::*;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct Element {
    links: [ElementPtrPair; 2], // change to N on const-generics

    pos: Coordinates,
}

struct DoublyLinkedList {
    count: usize,
    index: usize,
    dimension: Dimension,

    // we use raw pointers, so null_ptr takes place of 'None'
    // and dereferenceable pointers take place of 'Some(&mut Element)'
    first: ElementPtr,
    last: ElementPtr,
}

pub struct DancingLinksMatrix {
    // Vec used in place of an array (since we can't use const-generics yet)
    elements: Vec<Element>,
    headers: [Vec<DoublyLinkedList>; 2],
    // header contains info on a 1-dim. slice and therefore should be identified by N-1 coordinates
    // but for now since we only have 2 dimensions we can use a simple vector
    shape: [usize; 2],

    // Vec used as a stack
    removed_elements: Vec<ElementPtr>,
}

impl ElementPtrPair {
    fn new() -> Self {
        ElementPtrPair {
            prev: null_mut(),
            next: null_mut(),
        }
    }
}

impl From<Coordinates> for Element {
    fn from(pos: Coordinates) -> Self {
        let mut res: Element = Element {
            links: [ElementPtrPair::new(), ElementPtrPair::new()],
            pos,
        };
        // By default, a DLM elements should point to itself in all directions
        let res_ptr = res.as_mut_ptr();
        for link in res.links.iter_mut() {
            link.prev = res_ptr;
            link.next = res_ptr;
        }
        res
    }
}

#[test]
fn test_element_from_coordinates() {
    let mut element = Element::from([1, 2]);
    assert_eq!(element.links[0].prev, element.links[0].next);
    assert_eq!(element.pos, [1, 2]);
    let element_ptr: ElementPtr = &mut element;
    assert_eq!(element.links[0].prev, element_ptr);
}

impl Element {
    fn as_mut_ptr(&mut self) -> ElementPtr {
        let res: *mut Element = self;
        res
    }

    fn connect(&mut self, other: &mut Element, direction: Direction) {
        let self_ptr: ElementPtr = self;
        let other_ptr: ElementPtr = other;

        match direction {
            Forward(i) => {
                self.links[i].next = other_ptr;
                other.links[i].prev = self_ptr;
            }
            Back(i) => {
                self.links[i].prev = other_ptr;
                other.links[i].next = self_ptr;
            }
        }
    }
}

#[test]
fn test_element_connect() {
    let mut element = Element::from([1, 2]);
    let mut other = Element::from([5, 7]);
    let element_ptr: ElementPtr = &mut element;
    let other_ptr: ElementPtr = &mut other;

    assert_eq!(element.links[0].prev, element_ptr);
    assert_eq!(other.links[0].prev, other_ptr);

    element.connect(&mut other, Back(0));
    element.connect(&mut other, Forward(0));

    assert_eq!(other.links[0].prev, element_ptr);
    assert_eq!(element.links[0].prev, other_ptr);
    assert_eq!(element.links[0].next, other_ptr);
}

impl DoublyLinkedList {
    fn new(dimension: Dimension, index: usize) -> Self {
        DoublyLinkedList {
            count: 0,
            index,
            dimension: dimension,
            first: null_mut(),
            last: null_mut(),
        }
    }

    fn insert(&mut self, element: ElementPtr) {
        unsafe {
            let dim = self.dimension;

            if self.count == 0 {
                self.first = element;
                self.last = element;

            } else if self.count == 1 {
                (*element).connect(self.first.as_mut().unwrap(), Back(dim));
                (*element).connect(self.first.as_mut().unwrap(), Forward(dim));
                if (*self.first).pos[dim] < (*element).pos[dim] {
                    self.last = element;
                } else {
                    self.first = element;
                }

            } else {
                // Special case 1: new element is first
                if (*self.first).pos[dim] > (*element).pos[dim] {
                    (*element).connect(self.last.as_mut().unwrap(), Back(dim));
                    (*element).connect(self.first.as_mut().unwrap(), Forward(dim));
                    self.first = element;

                } else {
                    let mut existing = (*self.first).links[dim].next;

                    // Iterate until element finds itself or we end up at the end
                    while (*existing).pos[dim] < (*element).pos[dim]
                        && existing != self.first {
                        existing = (*existing).links[dim].next;
                    }
                    // After loop: existing.prev < element < existing, or
                    // (existing.prev = self.last) < element < end-of-list (exisitng = self.first)

                    // Connect element to the rest
                    let prev = (*existing).links[dim].prev.as_mut().unwrap();
                    let existing = existing.as_mut().unwrap();
                    (*element).connect(prev, Back(dim));
                    (*element).connect(existing, Forward(dim));

                    // Special case 2: new element is last
                    if (*self.last).pos[self.dimension] < (*element).pos[self.dimension] {
                        self.last = element;
                    }
                }
            }

            self.count += 1;
        }
    }

    fn remove(&mut self, element: ElementPtr) {
        if self.count == 1 {
            self.first = null_mut();
            self.last = null_mut();
        } else {
            // TODO
            unsafe {
                if self.first == element {
                    self.first = (*element).links[self.dimension].next;
                }
                if self.last == element {
                    self.last = (*element).links[self.dimension].prev;
                }


                (*element).links[self.dimension].prev.as_mut().unwrap().connect(
                    (*element).links[self.dimension].next.as_mut().unwrap(),
                    Forward(self.dimension)
                );
            }
        }

        self.count -= 1;
    }
}

#[test]
fn test_header_insert_and_remove() {
    let mut column = DoublyLinkedList::new(0, 1);
    let mut el1 = Element::from([1, 5]);
    let mut el2 = Element::from([1, 4]);
    let mut el3 = Element::from([1, 3]);
    let mut el4 = Element::from([1, 2]);
    let mut el5 = Element::from([1, 1]);

    let el1_ptr: ElementPtr = &mut el1;
    let el2_ptr: ElementPtr = &mut el2;
    let el3_ptr: ElementPtr = &mut el3;
    let el4_ptr: ElementPtr = &mut el4;
    let el5_ptr: ElementPtr = &mut el5;

    unsafe {
        assert_eq!(column.first, null_mut());
        assert_eq!(column.last, null_mut());
        assert_eq!(column.count, 0);
    }

    column.insert(&mut el2);

    unsafe {
        assert_eq!(column.first, el2_ptr);
        assert_eq!(column.last, el2_ptr);
        assert_eq!((*column.first).links[0].next, el2_ptr);
    }

    column.insert(&mut el1);

    unsafe {
        assert_eq!(column.first, el1_ptr);
        assert_eq!(column.last, el2_ptr);
        assert_eq!((*column.first).links[0].next, el2_ptr);
    }

    column.insert(&mut el3);

    unsafe {
        assert_eq!(column.first, el1_ptr);
        assert_eq!(column.last, el3_ptr);
        assert_eq!((*column.first).links[0].next, el2_ptr);
    }

    column.insert(&mut el5);

    unsafe {
        assert_eq!(column.first, el1_ptr);
        assert_eq!(column.last, el5_ptr);
        assert_eq!((*column.first).links[0].next, el2_ptr);
    }

    column.insert(&mut el4);

    unsafe {
        assert_eq!(column.first, el1_ptr);
        assert_eq!(column.last, el5_ptr);
        assert_eq!((*column.first).links[0].next, el2_ptr);
        assert_eq!(column.count, 5);
    }

    column.remove(&mut el5);

    unsafe {
        assert_eq!(column.first, el1_ptr);
        assert_eq!(column.last, el4_ptr);
        assert_eq!((*column.first).links[0].next, el2_ptr);
    }

    column.remove(&mut el2);

    unsafe {
        assert_eq!(column.first, el1_ptr);
        assert_eq!(column.last, el4_ptr);
        assert_eq!((*column.first).links[0].next, el3_ptr);
    }

    column.remove(&mut el1);

    unsafe {
        assert_eq!(column.first, el3_ptr);
        assert_eq!(column.last, el4_ptr);
        assert_eq!((*column.first).links[0].next, el4_ptr);
    }

    column.remove(&mut el4);

    unsafe {
        assert_eq!(column.first, el3_ptr);
        assert_eq!(column.last, el3_ptr);
        assert_eq!((*column.first).links[0].next, el3_ptr);
    }

    column.remove(&mut el3);

    unsafe {
        assert_eq!(column.first, null_mut());
        assert_eq!(column.last, null_mut());
        assert_eq!(column.count, 0);
    }
}

impl DancingLinksMatrix {
    fn new(shape: [usize; 2]) -> Self {
        DancingLinksMatrix {
            elements: Vec::new(),
            headers: [
                (0..shape[1]).map(|i| DoublyLinkedList::new(0, i)).collect(), // row-headers (len = # of columns)
                (0..shape[0]).map(|i| DoublyLinkedList::new(1, i)).collect() // column-headers (len = # of rows)
            ],
            shape,
            removed_elements: Vec::new()
        }
    }

    fn insert(&mut self, mut element: Element) {
        self.elements.push(element);
        let element_ptr: ElementPtr = self.elements.last_mut().unwrap();
        let element = self.elements.last().unwrap();
        for dim in 0..self.shape.len() {
            self.headers[dim][element.pos[1 - dim]].insert(element_ptr);
        }
    }

    fn remove(&mut self, element: ElementPtr) {
        for dim in 0..self.shape.len() {
            unsafe { self.headers[dim][(*element).pos[1 - dim]].remove(element); }
        }
        self.removed_elements.push(element);
    }

    fn undo_remove(&mut self) -> Option<()> {
        let mut element = self.removed_elements.pop()?;
        for dim in 0..self.shape.len() {
            unsafe { self.headers[dim][(*element).pos[1 - dim]].insert(element); }
        };
        Some(())
    }
}

#[test]
fn test_dlm_basic() {
    let mut dlm = DancingLinksMatrix::new([3, 7]);
    dlm.insert(Element::from([1, 2]));
    dlm.insert(Element::from([2, 4]));
    dlm.insert(Element::from([0, 3]));
    dlm.insert(Element::from([0, 5]));
    assert_eq!(dlm.elements.len(), 4);


    dlm.undo_remove();

}