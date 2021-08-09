use crate::style::{px, DynamicCSSProps, StaticCSSProps};
use crate::*;

#[derive(Default)]
pub struct Width<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Width<'a> {
    pub fn new(width: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("width", px(width));
        this
    }

    pub fn fill() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("width", "100%".into());
        this
    }

    pub fn min(mut self, width: u32) -> Self {
        self.static_css_props.insert("min-width", px(width));
        self
    }

    pub fn max(mut self, width: u32) -> Self {
        self.static_css_props.insert("max-width", px(width));
        self
    }
}

impl<'a> Style<'a> for Width<'a> {
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        (self.static_css_props, self.dynamic_css_props)
    }
}
