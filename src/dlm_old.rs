use std::io::{Error, ErrorKind};
use std::str::FromStr;

type PointerLink = *mut DancingLinksMatrixElement;

// Placement of an element in the original matrix
type Position = (usize, usize);

pub struct DancingLinksMatrixElement {
    left: PointerLink,
    right: PointerLink,
    top: PointerLink,
    down: PointerLink,

    position: Position,
}

pub struct DancingLinksMatrix {
    // could be changed to an array with const generics
    elements: Vec<DancingLinksMatrixElement>,
    removed_elements: Vec<PointerLink>,
}

impl From<Position> for DancingLinksMatrixElement {
    fn from(position: Position) -> Self {
        let mut res = DancingLinksMatrixElement {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            top: std::ptr::null_mut(),
            down: std::ptr::null_mut(),
            position,
        };
        res.left = &mut res;
        res.right = &mut res;
        res.top = &mut res;
        res.down = &mut res;
        res
    }
}

impl DancingLinksMatrixElement {
    fn connect_left(&mut self, other: &mut DancingLinksMatrixElement) {
        let other_ptr: *mut DancingLinksMatrixElement = other;
        self.left = other_ptr;
        self.right = other.right;
        unsafe {
            (*self.left).right = self;
            (*self.right).left = self;
        }
    }

    fn connect_top(&mut self, other: &mut DancingLinksMatrixElement) {
        let other_ptr: *mut DancingLinksMatrixElement = other;
        self.top = other_ptr;
        self.down = other.down;
        unsafe {
            (*self.top).down = self;
            (*self.down).top = self;
        }
    }
}

impl FromStr for DancingLinksMatrix {
    type Err = Error;

    fn from_str(literal: &str) -> Result<Self, Self::Err> {
        // Literal: 0s and 1s in rows of equal length
        if literal.len() == 0 {
            return Err(Error::new(ErrorKind::InvalidInput, "Literal cannot be empty!"));
        }

        let row_length = literal.split("\n").nth(0).unwrap().len();

        let mut res: DancingLinksMatrix = DancingLinksMatrix {
            elements: Vec::new(),
            removed_elements: Vec::new()
        };

        // List all elements by rows
        // TODO: put in pointers only after all nodes are in
        for (y, row) in literal.split("\n").enumerate() {
            if row.len() != row_length {
                return Err(Error::new(ErrorKind::InvalidInput, "All rows must have equal length"));
            }

            for (x, char) in row.chars().enumerate() {
                match char {
                    '0' => Ok(()),
                    '1' => {
                        let mut element = DancingLinksMatrixElement::from((x, y));
                        res.elements.push(element);
                        Ok(())
                    }
                    _ => Err(Error::new(ErrorKind::InvalidInput,
                                        "Values in the literal should only be 0 or 1"))
                }?
            }
        }

        res.reset();
        return Ok(res);
    }
}

impl DancingLinksMatrix {
    fn reset(&mut self) {
        for (index, element) in self.elements.iter_mut().enumerate() {
            for other_index in (index-1)..=0 {
                let other = &mut self.elements[other_index];
                if other.position.0 == element.position.0 {
                    element.connect_left(other);
                }
            }
            for other_index in (index-1)..=0 {
                let other = &mut self.elements[other_index];
                if other.position.1 == element.position.1 {
                    element.connect_top(other);
                }
            }
        }
    }

    fn get_anything(&mut self) -> Option<*mut DancingLinksMatrixElement> {
        self.elements.iter_mut().last().map(|x| { let x: *mut _ = x; x })
    }

    fn remove(&mut self, element: *mut DancingLinksMatrixElement) {
        unsafe {
            (*(*element).right).connect_left((*element).left.as_mut().unwrap());
            (*(*element).down).connect_top( (*element).left.as_mut().unwrap()); // TODO fix unwraps?
        }
        self.removed_elements.push(element);
    }

    fn undo(&mut self) -> Option<()> {
        unsafe {
            let element = self.removed_elements.pop()?;
            let left = (*element).left.as_mut()?;
            let top = (*element).top.as_mut()?;
            (*(*element).right).connect_left(element.as_mut()?);
            (*(*element).down).connect_top(element.as_mut()?);
            (*element).connect_left(left);
            (*element).connect_top(top);
        }
        Some(())
    }
}

#[test]
fn basics() {
    let mut dlm = DancingLinksMatrix::from_str("10101\
    10101\
    01010\
    10110").unwrap();

    unsafe {
        let old_dlme2_left;
        let dlme;
        let dlme2;
        {
            dlme = dlm.get_anything().unwrap();
            dlme2 = (*dlme).right;
            old_dlme2_left = (*dlme2).left;
        }
        assert!((*dlme2).left == old_dlme2_left);
        {
            dlm.remove(dlme);
        }
        assert!((*dlme2).left != old_dlme2_left);
        {
            dlm.undo();
        }
        assert!((*dlme2).left == old_dlme2_left);
    }
}