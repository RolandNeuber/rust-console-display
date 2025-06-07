use std::fmt::Display;

use crate::{
    eq,
    widget::DynamicWidget,
};

use super::StaticWidget;

pub trait TwoWidget<S: DynamicWidget, T: DynamicWidget>:
    DynamicWidget
{
    fn get_children(&self) -> (&S, &T);
    fn get_children_mut(&mut self) -> (&mut S, &mut T);
}

pub struct OverlayWidget<S: DynamicWidget, T: DynamicWidget> {
    child1_on_top: bool,
    child1: S,
    child2: T,
}

impl<S: StaticWidget, T: StaticWidget> OverlayWidget<S, T> {
    pub fn new(child1: S, child2: T, child1_on_top: bool) -> Self
    where
        eq!(S::WIDTH_CHARACTERS, T::WIDTH_CHARACTERS):,
        eq!(S::HEIGHT_CHARACTERS, T::HEIGHT_CHARACTERS):,
    {
        Self {
            child1_on_top,
            child1,
            child2,
        }
    }
}

impl<S: DynamicWidget, T: DynamicWidget> OverlayWidget<S, T> {
    pub fn build(
        child1: S,
        child2: T,
        child1_on_top: bool,
    ) -> Result<Self, String> {
        if child1.get_width_characters() != child2.get_width_characters() ||
            child1.get_height_characters() !=
                child2.get_height_characters()
        {
            return Err(format!(
                "Height and/or width in characters of arguments does not match. Height {} and {}. Width: {} and {}",
                child1.get_height_characters(),
                child2.get_height_characters(),
                child1.get_width_characters(),
                child2.get_width_characters(),
            ));
        }
        Ok(Self {
            child1_on_top,
            child1,
            child2,
        })
    }

    pub const fn get_child1_on_top(&self) -> bool {
        self.child1_on_top
    }

    pub const fn set_child1_on_top(&mut self, child1_on_top: bool) {
        self.child1_on_top = child1_on_top;
    }
}

impl<S: StaticWidget, T: StaticWidget> StaticWidget
    for OverlayWidget<S, T>
{
    const WIDTH_CHARACTERS: usize = S::WIDTH_CHARACTERS;

    const HEIGHT_CHARACTERS: usize = S::HEIGHT_CHARACTERS;
}

impl<S: DynamicWidget, T: DynamicWidget> DynamicWidget
    for OverlayWidget<S, T>
{
    fn get_width_characters(&self) -> usize {
        self.child1.get_width_characters()
    }

    fn get_height_characters(&self) -> usize {
        self.child1.get_height_characters()
    }
}

impl<S: DynamicWidget, T: DynamicWidget> TwoWidget<S, T>
    for OverlayWidget<S, T>
{
    fn get_children(&self) -> (&S, &T) {
        (&self.child1, &self.child2)
    }

    fn get_children_mut(&mut self) -> (&mut S, &mut T) {
        (&mut self.child1, &mut self.child2)
    }
}

impl<S: DynamicWidget, T: DynamicWidget> Display for OverlayWidget<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.child1_on_top {
            write!(f, "{}", self.child1.to_string())
        }
        else {
            write!(f, "{}", self.child2.to_string())
        }
    }
}

pub struct HorizontalTilingWidget<S: DynamicWidget, T: DynamicWidget> {
    child1: S,
    child2: T,
}

impl<S: DynamicWidget, T: DynamicWidget> HorizontalTilingWidget<S, T> {
    pub fn build(child1: S, child2: T) -> Result<Self, String> {
        if child1.get_height_characters() != child2.get_height_characters()
        {
            return Err(format!(
                "Height in characters of arguments does not match. {} and {}.",
                child1.get_height_characters(),
                child2.get_height_characters()
            ));
        }
        Ok(Self { child1, child2 })
    }
}

impl<S: StaticWidget, T: StaticWidget> HorizontalTilingWidget<S, T> {
    pub fn new(child1: S, child2: T) -> Self
    where
        eq!(S::HEIGHT_CHARACTERS, T::HEIGHT_CHARACTERS):,
    {
        Self { child1, child2 }
    }
}

impl<S: StaticWidget, T: StaticWidget> StaticWidget
    for HorizontalTilingWidget<S, T>
{
    const WIDTH_CHARACTERS: usize =
        S::WIDTH_CHARACTERS + T::WIDTH_CHARACTERS;

    const HEIGHT_CHARACTERS: usize = S::HEIGHT_CHARACTERS;
}

impl<S: DynamicWidget, T: DynamicWidget> DynamicWidget
    for HorizontalTilingWidget<S, T>
{
    fn get_width_characters(&self) -> usize {
        self.child1.get_width_characters() +
            self.child2.get_width_characters()
    }

    fn get_height_characters(&self) -> usize {
        self.child1.get_height_characters()
    }
}

impl<S: DynamicWidget, T: DynamicWidget> TwoWidget<S, T>
    for HorizontalTilingWidget<S, T>
{
    fn get_children(&self) -> (&S, &T) {
        (&self.child1, &self.child2)
    }

    fn get_children_mut(&mut self) -> (&mut S, &mut T) {
        (&mut self.child1, &mut self.child2)
    }
}

impl<S: DynamicWidget, T: DynamicWidget> Display
    for HorizontalTilingWidget<S, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_repr1 = self.child1.to_string();
        let str_repr2 = self.child2.to_string();
        let lines = Iterator::zip(str_repr1.lines(), str_repr2.lines());
        let mut str_repr = String::new();
        for line_pair in lines {
            str_repr.push_str(line_pair.0);
            str_repr.push_str(line_pair.1);
        }
        write!(f, "{str_repr}")
    }
}

pub struct VerticalTilingWidget<S: DynamicWidget, T: DynamicWidget> {
    child1: S,
    child2: T,
}

impl<S: DynamicWidget, T: DynamicWidget> VerticalTilingWidget<S, T> {
    pub fn build(child1: S, child2: T) -> Result<Self, String> {
        if child1.get_width_characters() != child2.get_width_characters() {
            return Err(format!(
                "Height in characters of arguments does not match. {} and {}.",
                child1.get_width_characters(),
                child2.get_width_characters()
            ));
        }
        Ok(Self { child1, child2 })
    }
}

impl<S: StaticWidget, T: StaticWidget> VerticalTilingWidget<S, T> {
    pub fn new(child1: S, child2: T) -> Self
    where
        eq!(S::WIDTH_CHARACTERS, T::WIDTH_CHARACTERS):,
    {
        Self { child1, child2 }
    }
}

impl<S: StaticWidget, T: StaticWidget> StaticWidget
    for VerticalTilingWidget<S, T>
{
    const WIDTH_CHARACTERS: usize = S::WIDTH_CHARACTERS;

    const HEIGHT_CHARACTERS: usize =
        S::HEIGHT_CHARACTERS + T::HEIGHT_CHARACTERS;
}

impl<S: DynamicWidget, T: DynamicWidget> DynamicWidget
    for VerticalTilingWidget<S, T>
{
    fn get_width_characters(&self) -> usize {
        self.child1.get_width_characters()
    }

    fn get_height_characters(&self) -> usize {
        self.child1.get_height_characters() +
            self.child2.get_height_characters()
    }
}

impl<S: DynamicWidget, T: DynamicWidget> TwoWidget<S, T>
    for VerticalTilingWidget<S, T>
{
    fn get_children(&self) -> (&S, &T) {
        (&self.child1, &self.child2)
    }

    fn get_children_mut(&mut self) -> (&mut S, &mut T) {
        (&mut self.child1, &mut self.child2)
    }
}

impl<S: DynamicWidget, T: DynamicWidget> Display
    for VerticalTilingWidget<S, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.child1.to_string(),
            self.child2.to_string()
        )
    }
}
