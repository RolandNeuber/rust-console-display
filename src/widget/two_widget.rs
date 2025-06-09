use std::{
    fmt::Display,
    ops::{
        Deref,
        DerefMut,
    },
};

use crate::{
    eq,
    widget::DynamicWidget,
};

use super::StaticWidget;

pub trait TwoWidget<S: DynamicWidget, T: DynamicWidget>:
    DynamicWidget + Deref + DerefMut
{
    fn get_children(&self) -> (&S, &T);
    fn get_children_mut(&mut self) -> (&mut S, &mut T);
}

pub struct OverlayWidget<S: DynamicWidget, T: DynamicWidget> {
    child1_on_top: bool,
    children: (S, T),
}

impl<S: StaticWidget, T: StaticWidget> OverlayWidget<S, T> {
    pub const fn new(child1: S, child2: T, child1_on_top: bool) -> Self
    where
        eq!(S::WIDTH_CHARACTERS, T::WIDTH_CHARACTERS):,
        eq!(S::HEIGHT_CHARACTERS, T::HEIGHT_CHARACTERS):,
    {
        Self {
            child1_on_top,
            children: (child1, child2),
        }
    }
}

impl<S: DynamicWidget, T: DynamicWidget> OverlayWidget<S, T> {
    /// Builds an overlay widget with two children.
    /// The `child1_on_top` parameter determines whether the first child should be
    /// on top or below the second child.
    ///
    /// # Errors
    ///
    /// Returns an error if the dimensions of both children don't match.
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
            children: (child1, child2),
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
        self.children.0.get_width_characters()
    }

    fn get_height_characters(&self) -> usize {
        self.children.1.get_height_characters()
    }
}

impl<S: DynamicWidget, T: DynamicWidget> TwoWidget<S, T>
    for OverlayWidget<S, T>
{
    fn get_children(&self) -> (&S, &T) {
        (&self.children.0, &self.children.1)
    }

    fn get_children_mut(&mut self) -> (&mut S, &mut T) {
        (&mut self.children.0, &mut self.children.1)
    }
}

impl<S: DynamicWidget, T: DynamicWidget> Display for OverlayWidget<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.child1_on_top {
            write!(f, "{}", self.children.0.to_string())
        }
        else {
            write!(f, "{}", self.children.1.to_string())
        }
    }
}

impl<S: DynamicWidget, T: DynamicWidget> Deref for OverlayWidget<S, T> {
    type Target = (S, T);

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<S: DynamicWidget, T: DynamicWidget> DerefMut for OverlayWidget<S, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

pub struct HorizontalTilingWidget<S: DynamicWidget, T: DynamicWidget> {
    children: (S, T),
}

impl<S: DynamicWidget, T: DynamicWidget> HorizontalTilingWidget<S, T> {
    /// Builds horizontal tiling widget with two children.
    /// `child1` will be displayed on the left, `child2` on the right.
    ///
    /// # Errors
    ///
    /// Returns an error if the height of both children does not match.
    pub fn build(child1: S, child2: T) -> Result<Self, String> {
        if child1.get_height_characters() != child2.get_height_characters()
        {
            return Err(format!(
                "Height in characters of arguments does not match. {} and {}.",
                child1.get_height_characters(),
                child2.get_height_characters()
            ));
        }
        Ok(Self {
            children: (child1, child2),
        })
    }
}

impl<S: StaticWidget, T: StaticWidget> HorizontalTilingWidget<S, T> {
    pub const fn new(child1: S, child2: T) -> Self
    where
        eq!(S::HEIGHT_CHARACTERS, T::HEIGHT_CHARACTERS):,
    {
        Self {
            children: (child1, child2),
        }
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
        self.children.0.get_width_characters() +
            self.children.1.get_width_characters()
    }

    fn get_height_characters(&self) -> usize {
        self.children.0.get_height_characters()
    }
}

impl<S: DynamicWidget, T: DynamicWidget> TwoWidget<S, T>
    for HorizontalTilingWidget<S, T>
{
    fn get_children(&self) -> (&S, &T) {
        (&self.children.0, &self.children.1)
    }

    fn get_children_mut(&mut self) -> (&mut S, &mut T) {
        (&mut self.children.0, &mut self.children.1)
    }
}

impl<S: DynamicWidget, T: DynamicWidget> Display
    for HorizontalTilingWidget<S, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_repr1 = self.children.0.to_string();
        let str_repr2 = self.children.1.to_string();
        let lines = Iterator::zip(str_repr1.lines(), str_repr2.lines());
        let mut str_repr = String::new();
        for line_pair in lines {
            str_repr.push_str(line_pair.0);
            str_repr.push_str(line_pair.1);
        }
        write!(f, "{str_repr}")
    }
}

impl<S: DynamicWidget, T: DynamicWidget> Deref
    for HorizontalTilingWidget<S, T>
{
    type Target = (S, T);

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<S: DynamicWidget, T: DynamicWidget> DerefMut
    for HorizontalTilingWidget<S, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

pub struct VerticalTilingWidget<S: DynamicWidget, T: DynamicWidget> {
    children: (S, T),
}

impl<S: DynamicWidget, T: DynamicWidget> VerticalTilingWidget<S, T> {
    /// Builds vertical tiling widget with two children.
    /// `child1` will be displayed on the top, `child2` on the bottom.
    ///
    /// # Errors
    ///
    /// Returns an error if the width of both children does not match.
    pub fn build(child1: S, child2: T) -> Result<Self, String> {
        if child1.get_width_characters() != child2.get_width_characters() {
            return Err(format!(
                "Width in characters of arguments does not match. {} and {}.",
                child1.get_width_characters(),
                child2.get_width_characters()
            ));
        }
        Ok(Self {
            children: (child1, child2),
        })
    }
}

impl<S: StaticWidget, T: StaticWidget> VerticalTilingWidget<S, T> {
    pub const fn new(child1: S, child2: T) -> Self
    where
        eq!(S::WIDTH_CHARACTERS, T::WIDTH_CHARACTERS):,
    {
        Self {
            children: (child1, child2),
        }
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
        self.children.0.get_width_characters()
    }

    fn get_height_characters(&self) -> usize {
        self.children.0.get_height_characters() +
            self.children.1.get_height_characters()
    }
}

impl<S: DynamicWidget, T: DynamicWidget> TwoWidget<S, T>
    for VerticalTilingWidget<S, T>
{
    fn get_children(&self) -> (&S, &T) {
        (&self.children.0, &self.children.1)
    }

    fn get_children_mut(&mut self) -> (&mut S, &mut T) {
        (&mut self.children.0, &mut self.children.1)
    }
}

impl<S: DynamicWidget, T: DynamicWidget> Display
    for VerticalTilingWidget<S, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.children.0.to_string(),
            self.children.1.to_string()
        )
    }
}

impl<S: DynamicWidget, T: DynamicWidget> Deref
    for VerticalTilingWidget<S, T>
{
    type Target = (S, T);

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<S: DynamicWidget, T: DynamicWidget> DerefMut
    for VerticalTilingWidget<S, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}
