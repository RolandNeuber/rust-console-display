use std::ops::{Deref, DerefMut};

use crate::ConsoleDisplay;

pub trait Widget: ToString {
    fn get_width_characters(&self) -> usize;
    fn get_height_characters(&self) -> usize;
}

pub trait SingleWidget<T>: Widget {
    fn new(child: T) -> Self;
    fn get_child(&self) -> &T;
    fn get_child_mut(&mut self) -> &mut T;
}

pub trait TwoWidget<S, T>: Widget {
    fn new(child1: S, child2: T) -> Self;
    fn get_children(&self) -> (&S, &T);
    fn get_children_mut(&mut self) -> (&mut S, &mut T);
}

pub struct NoneWidget<T: ConsoleDisplay> {
    child: T
}

impl<T: ConsoleDisplay> Widget for NoneWidget<T> {
    fn get_width_characters(&self) -> usize {
        self.get_child().get_width_characters()
    }

    fn get_height_characters(&self) -> usize {
        self.get_child().get_height_characters()
    }
}

impl<T: ConsoleDisplay> SingleWidget<T> for NoneWidget<T> {
    fn new(child: T) -> Self {
        NoneWidget {
            child
        }
    }
    
    fn get_child(&self) -> &T {
        &self.child
    }
    
    fn get_child_mut(&mut self) -> &mut T {
        &mut self.child
    }
}

impl<T: ConsoleDisplay> ToString for NoneWidget<T> {
    fn to_string(&self) -> String {
        self.get_child().to_string()
    }
}

// pub struct OverlayWidget<S, T> {
//     child1: S,
//     child2: T
// }

// impl<S, T> Widget for OverlayWidget<S, T> {
//     fn get_width_characters(&self) -> usize {
//         todo!()
//     }

//     fn get_height_characters(&self) -> usize {
//         todo!()
//     }
// }

// impl<S, T> TwoWidget<S, T> for OverlayWidget<S, T> {
//     fn new(child1: S, child2: T) -> Self {
//         OverlayWidget {
//             child1,
//             child2
//         }
//     }
    
//     fn get_children(&self) -> (&S, &T) {
//         (&self.child1, &self.child2)
//     }
    
//     fn get_children_mut(&mut self) -> (&mut S, &mut T) {
//         (&mut self.child1, &mut self.child2)
//     }
// }

// impl<S, T> ToString for OverlayWidget<S, T> {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }

// pub struct TilingWidget<S, T> {
//     child1: S,
//     child2: T
// }

// impl<S, T> Widget for TilingWidget<S, T> {
//     fn get_width_characters(&self) -> usize {
//         todo!()
//     }

//     fn get_height_characters(&self) -> usize {
//         todo!()
//     }
// }

// impl<S, T> TwoWidget<S, T> for TilingWidget<S, T> {
//     fn new(child1: S, child2: T) -> Self {
//         TilingWidget { 
//             child1,
//             child2
//         }
//     }

//     fn get_children(&self) -> (&S, &T) {
//         (&self.child1, &self.child2)
//     }

//     fn get_children_mut(&mut self) -> (&mut S, &mut T) {
//         (&mut self.child1, &mut self.child2)
//     }
// }

// impl<S, T> ToString for TilingWidget<S, T> {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }

// pub struct UvWidget<T> {
//     child: T
// }

// impl<T> Widget for UvWidget<T> {
//     fn get_width_characters(&self) -> usize {
//         todo!()
//     }

//     fn get_height_characters(&self) -> usize {
//         todo!()
//     }
// }

// impl<T> SingleWidget<T> for UvWidget<T> {
//     fn new(child: T) -> Self {
//         UvWidget {
//             child
//         }
//     }

//     fn get_child(&self) -> &T {
//         &self.child
//     }
    
//     fn get_child_mut(&mut self) -> &mut T {
//         &mut self.child 
//     }
// }

// impl<T> ToString for UvWidget<T> {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }

// impl<T> Deref for UvWidget<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         todo!()
//     }
// }

// impl<T> DerefMut for UvWidget<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         todo!()
//     }
// }