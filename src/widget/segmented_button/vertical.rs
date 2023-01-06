// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use super::state::{Selectable, State};
use super::style::StyleSheet;
use super::widget::{SegmentedButton, SegmentedVariant};

use iced::{Length, Rectangle, Size};
use iced_native::layout;

/// A type marker defining the vertical variant of a [`SegmentedButton`].
pub struct Vertical;

/// Vertical [`SegmentedButton`].
pub type VerticalSegmentedButton<'a, Selection, Message, Renderer> =
    SegmentedButton<'a, Vertical, Selection, Message, Renderer>;

/// Vertical implementation of the [`SegmentedButton`].
#[must_use]
pub fn vertical_segmented_button<Selection, Message, Renderer, Data>(
    state: &State<Selection, Data>,
) -> SegmentedButton<Vertical, Selection, Message, Renderer>
where
    Renderer: iced_native::Renderer
        + iced_native::text::Renderer
        + iced_native::image::Renderer
        + iced_native::svg::Renderer,
    Renderer::Theme: StyleSheet,
    Selection: Selectable,
{
    SegmentedButton::new(&state.inner)
}

impl<'a, Selection, Message, Renderer> SegmentedVariant
    for SegmentedButton<'a, Vertical, Selection, Message, Renderer>
where
    Renderer: iced_native::Renderer
        + iced_native::text::Renderer
        + iced_native::image::Renderer
        + iced_native::svg::Renderer,
    Renderer::Theme: StyleSheet,
    Selection: Selectable,
{
    type Renderer = Renderer;

    fn variant_appearance(
        theme: &<Self::Renderer as iced_native::Renderer>::Theme,
        style: &<<Self::Renderer as iced_native::Renderer>::Theme as StyleSheet>::Style,
    ) -> super::Appearance {
        theme.vertical(style)
    }

    #[allow(clippy::cast_precision_loss)]
    fn variant_button_bounds(&self, mut bounds: Rectangle, nth: usize) -> Rectangle {
        let num = self.state.buttons.len();
        if num != 0 {
            let spacing = f32::from(self.spacing);
            bounds.height = (bounds.height - (num as f32 * spacing) + spacing) / num as f32;

            if nth != 0 {
                bounds.y += (nth as f32 * bounds.height) + (nth as f32 * spacing);
            }
        }

        bounds
    }

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn variant_layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(self.width);
        let text_size = renderer.default_size();

        let (width, mut height) = self.max_button_dimensions(renderer, text_size, limits.max());

        let num = self.state.buttons.len();
        let spacing = f32::from(self.spacing);

        if num != 0 {
            height = (num as f32 * height) + (num as f32 * spacing) - spacing;
        }

        let size = limits
            .height(Length::Units(height as u16))
            .resolve(Size::new(width, height));

        layout::Node::new(size)
    }
}
