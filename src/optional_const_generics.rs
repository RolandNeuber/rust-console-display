use std::marker::PhantomData;

use crate::optional_const_generics::sealed::Sealed;

mod sealed {
    pub trait Sealed {}
}

pub trait Dimension: Sealed {
    fn value(&self) -> usize;
}

#[derive(Clone, Copy)]
pub struct CompileTime<const V: usize>;

impl<const V: usize> Dimension for CompileTime<V> {
    fn value(&self) -> usize {
        V
    }
}

impl<const V: usize> Sealed for CompileTime<V> {}

#[derive(Clone, Copy)]
pub struct RunTime(pub usize);

impl Dimension for RunTime {
    fn value(&self) -> usize {
        self.0
    }
}

impl Sealed for RunTime {}