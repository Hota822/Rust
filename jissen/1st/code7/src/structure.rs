use std::cell::RefCell;

pub struct A {
    pub c: char,
    pub s: String,
}

pub struct B {
    pub c: char,
    pub s: RefCell<String>,
}


pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>,
    len: usize,
    pos: usize,
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    type Item = &'vec T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &T {
        match self.get(index) {
            Some(s) => s,
            None => default,
        }
    }

    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        Iter {
            elements: &self.elements,
            len: self.len,
            pos: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow()
        }
        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // let element = self.elements[self.len];
            let element = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(element)
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }


}




#[derive(Copy, Clone, Debug)]
pub struct Parent(
    pub usize,
    pub Child,
    pub Child
);

#[derive(Debug)]
pub struct Parent2(
    pub isize,
    pub Child3,
    pub Child3
);

// impl Drop for Parent {
//     fn drop(&mut self) {
//         println!("dropping {:?}", self);
//     }
// }

#[derive(Copy, Clone, Debug)]
pub struct Child(pub usize);

// impl Drop for Child {
//     fn drop(&mut self) {
//         println!("dropping {:?}", self);
//     }
// }

#[derive(Clone, Debug)]
pub struct Child2(pub usize);

#[derive(Debug)]
pub struct Child3(pub usize);

pub fn aaa() {
    println!("aaa");
}