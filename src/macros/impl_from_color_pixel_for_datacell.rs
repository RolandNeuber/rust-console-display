#[macro_export]
macro_rules! impl_from_color_pixel_for_datacell {
    ($type:ty, $base:ty) => {
        impl From<$type> for DataCell {
            fn from(val: $type) -> Self {
                let colors = val.pixels();
                let grouping = $crate::color::Groupable::group(&colors);
                let symb = <$base>::new(grouping).character();

                let mut col1 = vec![];
                let mut col2 = vec![];
                for i in 0..grouping.len() {
                    if grouping[i] {
                        col1.push(colors[i]);
                    }
                    else {
                        col2.push(colors[i]);
                    }
                }
                let col1 = $crate::color::Mixable::mix(&col1);
                let col2 = $crate::color::Mixable::mix(&col2);

                Self {
                    character: symb,
                    foreground: col1,
                    background: col2,
                }
            }
        }
    };
}
