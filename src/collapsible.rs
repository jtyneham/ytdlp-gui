use iced::{
    Alignment, Element,
    widget::{Column, column, row, text},
};

use crate::{Message, i18n::is_rtl, theme::button};

pub fn collapsible<'a>(
    title: String,
    expanded: bool,
    on_toggle: Message,
    body: impl Into<Element<'a, Message>>,
) -> Column<'a, Message> {
    let arrow = if expanded {
        iced_fonts::Bootstrap::ChevronDown
    } else if is_rtl() {
        iced_fonts::Bootstrap::ChevronLeft
    } else {
        iced_fonts::Bootstrap::ChevronRight
    };
    let label = row![iced_fonts::bootstrap::icon(arrow), text(title)]
        .spacing(10)
        .align_y(Alignment::Center);

    let header = button(label).on_press(on_toggle).width(iced::Length::Fill);

    let mut col = column![header].spacing(8);

    if is_rtl() {
        col = col.align_x(Alignment::End);
    }

    if expanded {
        col = col.push(body);
    }
    col
}
