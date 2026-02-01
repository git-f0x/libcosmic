// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use crate::widget::button::Catalog;

use crate::{
    Apply, Element, theme,
    widget::{Button, button, container, divider, vertical_space},
};

#[inline]
pub fn button_column<'a, Message: Clone + 'static>() -> ButtonColumn<'a, Message> {
    ButtonColumn::default()
}

#[must_use]
pub struct ButtonColumn<'a, Message> {
    children: Vec<Button<'a, Message>>,
}

impl<Message> Default for ButtonColumn<'_, Message> {
    fn default() -> Self {
        Self {
            children: Vec::with_capacity(4),
        }
    }
}

impl<'a, Message: Clone + 'static> ButtonColumn<'a, Message> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, item: impl Into<Element<'a, Message>>, on_press: Message) -> Self {
        let cosmic_theme::Spacing {
            space_xxs, space_m, ..
        } = theme::spacing();

        let list_item = iced::widget::row![
            container(item).align_y(iced::Alignment::Center),
            vertical_space().height(iced::Length::Fixed(32.))
        ]
        .align_y(iced::Alignment::Center)
        .apply(button::custom)
        .padding([space_xxs, space_m])
        .width(iced::Length::Fill)
        .on_press(on_press);

        self.children.push(list_item);
        self
    }

    #[must_use]
    pub fn into_element(self) -> Element<'a, Message> {
        let count = self.children.len();
        let mut content: Vec<Element<'a, Message>> = Vec::with_capacity(count * 2 - 1);

        for (i, btn) in self.children.into_iter().enumerate() {
            let is_first = i == 0;
            let is_last = i == count - 1;

            let styled_btn = btn.class(theme::Button::Custom {
                active: Box::new(move |focused, theme| {
                    let mut s = theme.active(focused, false, &theme::Button::ListItem);
                    s.border_radius = get_radius(theme, is_first, is_last).into();
                    s
                }),
                hovered: Box::new(move |focused, theme| {
                    let mut s = theme.hovered(focused, false, &theme::Button::ListItem);
                    s.border_radius = get_radius(theme, is_first, is_last).into();
                    s
                }),
                pressed: Box::new(move |focused, theme| {
                    let mut s = theme.pressed(focused, false, &theme::Button::ListItem);
                    s.border_radius = get_radius(theme, is_first, is_last).into();
                    s
                }),
                disabled: Box::new(move |theme| {
                    let mut s = theme.disabled(&theme::Button::ListItem);
                    s.border_radius = get_radius(theme, is_first, is_last).into();
                    s
                }),
            });

            content.push(styled_btn.into());

            if !is_last {
                content.push(container(divider::horizontal::default()).into());
            }
        }

        crate::widget::column::with_children(content)
            .apply(container)
            .class(theme::Container::List)
            .width(iced::Length::Fill)
            .into()
    }
}

fn get_radius(theme: &crate::Theme, first: bool, last: bool) -> [f32; 4] {
    let r = theme.cosmic().radius_s();
    match (first, last) {
        (true, true) => [r[0], r[1], r[2], r[3]],
        (true, false) => [r[0], r[1], 0.0, 0.0],
        (false, true) => [0.0, 0.0, r[2], r[3]],
        (false, false) => [0.0, 0.0, 0.0, 0.0],
    }
}

impl<'a, Message: Clone + 'static> From<ButtonColumn<'a, Message>> for Element<'a, Message> {
    fn from(column: ButtonColumn<'a, Message>) -> Self {
        column.into_element()
    }
}
