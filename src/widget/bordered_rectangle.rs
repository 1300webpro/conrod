//! The `BorderedRectangle` widget and related items.

use {
    Color,
    Colorable,
    Dimensions,
    Borderable,
    Positionable,
    Scalar,
    Sizeable,
    Widget,
};
use widget;


/// A filled rectangle widget that may or may not have some border.
///
/// NOTE: BorderedRectangle is currently implemented as two filled rectangles:
///
/// 1. A `Rectangle` for the border.
/// 2. A `Rectangle` for the non-border area.
///
/// This is flawed in that, if a user specifies an alpha lower than 1.0, the front `Rectangle` will
/// blend with the border `Rectangle`, which is likely unexpected behaviour. This should be changed
/// so that the border is drawn using a outlined `Rectangle`.
#[derive(Copy, Clone, Debug, WidgetCommon_)]
pub struct BorderedRectangle {
    /// Data necessary and common for all widget builder types.
    #[conrod(common_builder)]
    pub common: widget::CommonBuilder,
    /// Unique styling for the **BorderedRectangle**.
    pub style: Style,
}

widget_ids! {
    struct Ids {
        border,
        rectangle,
    }
}

/// Unique styling for the **BorderedRectangle** widget.
#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle_)]
pub struct Style {
    /// Shape styling for the inner rectangle.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<Color>,
    /// The thickness of the border.
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the border.
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
}

/// Unique state for the `BorderedRectangle`.
pub struct State {
    ids: Ids,
}

impl BorderedRectangle {

    /// Build a new **BorderedRectangle**.
    pub fn new(dim: Dimensions) -> Self {
        BorderedRectangle {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }.wh(dim)
    }

    builder_method!(pub with_style { style = Style });

}


impl Widget for BorderedRectangle {
    type State = State;
    type Style = Style;
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the Rectangle.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, style, rect, ui, .. } = args;

        let border = style.border(&ui.theme);
        if border > 0.0 {
            let border_color = style.border_color(&ui.theme);
            widget::Rectangle::fill(rect.dim())
                .xy(rect.xy())
                .color(border_color)
                .parent(id)
                .graphics_for(id)
                .set(state.ids.border, ui);
        }

        let color = style.color(&ui.theme);
        widget::Rectangle::fill(rect.pad(border).dim())
            .xy(rect.xy())
            .color(color)
            .parent(id)
            .graphics_for(id)
            .set(state.ids.rectangle, ui);
    }

}


impl Colorable for BorderedRectangle {
    builder_method!(color { style.color = Some(Color) });
}


impl Borderable for BorderedRectangle {
    builder_methods!{
        border { style.border = Some(Scalar) }
        border_color { style.border_color = Some(Color) }
    }
}
