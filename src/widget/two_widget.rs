use std::fmt::Display;

use super::Widget;

pub trait TwoWidget<S, T>: Widget {
    fn get_children(&self) -> (&S, &T);
    fn get_children_mut(&mut self) -> (&mut S, &mut T);
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

pub struct HorizontalTilingWidget<S: Widget, T: Widget> {
    child1: S,
    child2: T
}

impl<S: Widget, T: Widget> HorizontalTilingWidget<S, T> {
    pub fn build(child1: S, child2: T) -> Result<Self, String> {
        if child1.get_height_characters() != child2.get_height_characters() {
            return Err(format!(
                "Height in characters of arguments does not match. {} and {}.",
                child1.get_height_characters(),
                child2.get_height_characters()
            ));
        }
        Ok(HorizontalTilingWidget { 
            child1,
            child2
        })
    }
}

impl<S: Widget, T: Widget> Widget for HorizontalTilingWidget<S, T> {
    fn get_width_characters(&self) -> usize {
        self.child1.get_width_characters() + self.child2.get_width_characters()
    }

    fn get_height_characters(&self) -> usize {
        self.child1.get_height_characters()
    }
}

impl<S: Widget, T: Widget> TwoWidget<S, T> for HorizontalTilingWidget<S, T> {
    fn get_children(&self) -> (&S, &T) {
        (&self.child1, &self.child2)
    }

    fn get_children_mut(&mut self) -> (&mut S, &mut T) {
        (&mut self.child1, &mut self.child2)
    }
}

impl<S: Widget, T: Widget> Display for HorizontalTilingWidget<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_repr1 = self.child1.to_string();
        let str_repr2 = self.child2.to_string();
        let lines= Iterator::zip(
            str_repr1.lines(), 
            str_repr2.lines()
        );
        let mut str_repr = String::new();
        for line_pair in lines {
            str_repr.push_str(line_pair.0);
            str_repr.push_str(line_pair.1);
        }
        write!(f, "{str_repr}")
    }
}

pub struct VerticalTilingWidget<S: Widget, T: Widget> {
    child1: S,
    child2: T
}

impl<S: Widget, T: Widget> VerticalTilingWidget<S, T> {
    pub fn build(child1: S, child2: T) -> Result<Self, String> {
        if child1.get_width_characters() != child2.get_width_characters() {
            return Err(format!(
                "Height in characters of arguments does not match. {} and {}.",
                child1.get_width_characters(),
                child2.get_width_characters()
            ));
        }
        Ok(VerticalTilingWidget { 
            child1,
            child2
        })
    }
}

impl<S: Widget, T: Widget> Widget for VerticalTilingWidget<S, T> {
    fn get_width_characters(&self) -> usize {
        self.child1.get_width_characters()
    }

    fn get_height_characters(&self) -> usize {
        self.child1.get_height_characters() + self.child2.get_height_characters()
    }
}

impl<S: Widget, T: Widget> TwoWidget<S, T> for VerticalTilingWidget<S, T> {
    fn get_children(&self) -> (&S, &T) {
        (&self.child1, &self.child2)
    }

    fn get_children_mut(&mut self) -> (&mut S, &mut T) {
        (&mut self.child1, &mut self.child2)
    }
}

impl<S: Widget, T: Widget> Display for VerticalTilingWidget<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.child1.to_string(), self.child2.to_string())
    }
}