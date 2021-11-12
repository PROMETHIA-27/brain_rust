#![allow(unused_macros)]
#![recursion_limit = "100000"]

#[macro_export]
macro_rules! brain_rust {
    ($($toks:tt)*) => {
        #[allow(unused_mut, unused_imports)]
        {
            use std::ops::IndexMut;

            let mut array = UnboundedArray::<u8>::new();
            let mut index = 0isize;
            
            $crate::brain_rust_impl! { {$($toks)*} array index }
        }
    }
}

#[macro_export]
macro_rules! brain_rust_impl {
    ({> $($rest:tt)*} $array:ident $index:ident) => {
        $index = $index.wrapping_add(1);

        $crate::brain_rust_impl!({$($rest)*} $array $index)
    };

    ({< $($rest:tt)*} $array:ident $index:ident) => {
        $index = $index.wrapping_sub(1);

        $crate::brain_rust_impl!({$($rest)*} $array $index)
    };

    ({>> $($rest:tt)*} $array:ident $index:ident) => {
        $index = $index.wrapping_add(2);

        $crate::brain_rust_impl!({$($rest)*} $array $index)
    };

    ({<< $($rest:tt)*} $array:ident $index:ident) => {
        $index = $index.wrapping_sub(2);

        $crate::brain_rust_impl!({$($rest)*} $array $index)
    };

    ({-> $($rest:tt)*} $array:ident $index:ident) => {
        $array[$index] = (*$array.index_mut($index)).wrapping_sub(1);
        $index = $index.wrapping_add(1);

        $crate::brain_rust_impl!({$($rest)*} $array $index)
    };

    ({<- $($rest:tt)*} $array:ident $index:ident) => {
        $index = $index.wrapping_sub(1);
        $array[$index] = (*$array.index_mut($index)).wrapping_sub(1);

        $crate::brain_rust_impl!({$($rest)*} $array $index)
    };
    
    ({+ $($rest:tt)*} $array:ident $index:ident) => {
        $array[$index] = (*$array.index_mut($index)).wrapping_add(1);

        $crate::brain_rust_impl!({$($rest)*} $array $index)
    };
    
    ({- $($rest:tt)*} $array:ident $index:ident) => {
        $array[$index] = (*$array.index_mut($index)).wrapping_sub(1);

        $crate::brain_rust_impl!({$($rest)*} $array $index)
    };
    
    ({, $($rest:tt)*} $array:ident $index:ident) => {
        {
            use std::io::*;

            let mut c = [0];
            if let Ok(1) = stdin().lock().read(&mut c) {
                $array[$index] = c[0];
            } else {
                panic!("You must input a character for brainrust to consume with ,!")
            }
        }

        $crate::brain_rust_impl!({$($rest)*} $array $index)
    };
    
    ({. $($rest:tt)*} $array:ident $index:ident) => {
        {
            use std::{ops::IndexMut, io::Write};

            let c: char = (*$array.index_mut($index)).into();

            print!("{}", c);

            let _ = std::io::stdout().flush();
        }

        $crate::brain_rust_impl!({$($rest)*} $array $index)
    };
    
    ({[$($loop_contents:tt)*] $($rest:tt)*} $array:ident $index:ident) => {
        { 
            use std::ops::IndexMut;

            while (*$array.index_mut($index)) > 0 {
                $crate::brain_rust_impl!({$($loop_contents)*} $array $index);
            }
        }

        $crate::brain_rust_impl!({$($rest)*} $array $index)
    };

    ({ } $array:ident $index:ident) => {

    };
}

pub struct UnboundedArray<T> {
    left: Vec<T>,
    right: Vec<T>,
}

impl<T: Default> UnboundedArray<T> {
    pub fn new() -> UnboundedArray<T> {
        UnboundedArray {
            left: vec!(),
            right: vec!(),
        }
    }

    pub fn ensure_capacity(&mut self, size: isize) {
        if size < 0 {
            let cap = self.left.capacity();
            let size = size.abs().try_into().unwrap();
            if cap < size {
                self.left.reserve(size - cap);
                while self.left.len() < self.left.capacity() {
                    self.left.push(Default::default());
                }
            }
        } else {
            let cap = self.right.capacity();
            let size = size.try_into().unwrap();
            if cap < size {
                self.right.reserve(size - cap);
                while self.right.len() < self.right.capacity() {
                    self.right.push(Default::default());
                }
            }
        }
    }

    pub fn assert_capacity(&self, size: isize) {
        if size < 0 {
            let cap = self.left.capacity();
            let size = size.abs().try_into().unwrap();
            if cap < size {
                panic!("Insufficient capacity in unboundedarray, try borrowing mutably")
            }
        } else {
            let cap = self.right.capacity();
            let size = size.try_into().unwrap();
            if cap < size {
                panic!("Insufficient capacity in unboundedarray, try borrowing mutably")
            }
        }
    }
}

impl<T> std::ops::Index<isize> for UnboundedArray<T> {
    type Output = T;

    fn index(&self, _index: isize) -> &Self::Output {
        // self.assert_capacity(index + (if index.is_negative() {-1} else {1}));
        // let uindex: usize = index.abs().try_into().unwrap();
        // if index < 0 {
        //     &self.left[uindex]
        // } else {
        //     &self.right[uindex]
        // }
        panic!("Non-mutable indexing is not allowed for unbounded array, as it may have to resize on indexing!");
    }
}

impl<T: Default> std::ops::IndexMut<isize> for UnboundedArray<T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        self.ensure_capacity(index + (if index.is_negative() {-1} else {1}));
        let uindex: usize = index.abs().try_into().unwrap();
        if index < 0 {
            &mut self.left[uindex]
        } else {
            &mut self.right[uindex]
        }
    }
}