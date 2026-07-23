#[macro_export]
macro_rules! impl_from_mono_chrome_pixel_for_datacell {
    ($type:ty) => {
        impl const From<$type> for DataCell {
            fn from(val: $type) -> Self {
                Self {
                    character: val.character(),
                    foreground: TerminalColor::Default,
                    background: TerminalColor::Default,
                }
            }
        }
    };
}
