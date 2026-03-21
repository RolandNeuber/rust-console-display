use crate::optional_const_generics::sealed::Sealed;

mod sealed {
    pub const trait Sealed {}
}

pub const trait Dimension: [const] Sealed {
    fn value(&self) -> usize;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CompileTime<const V: usize>;

impl<const V: usize> const Dimension for CompileTime<V> {
    fn value(&self) -> usize {
        V
    }
}

impl<const V: usize> const Sealed for CompileTime<V> {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RunTime(pub usize);

impl const Dimension for RunTime {
    fn value(&self) -> usize {
        self.0
    }
}

impl const Sealed for RunTime {}

#[cfg(test)]
mod tests {
    use crate::optional_const_generics::{CompileTime, Dimension, RunTime};

    #[test]
    const fn compile_time() {
        assert!(42 == CompileTime::<42>.value());
    }

    #[test]
    fn run_time() {
        assert_eq!(36, RunTime(36).value());
    }
}