use iced::widget::Tooltip;
use iced::Renderer;

use crate::ui::theme::container::ContainerStyleInner;
use crate::ui::theme::get_theme;
use crate::ui::theme::Element;
use crate::ui::theme::GauntletComplexTheme;
use crate::ui::theme::ThemableWidget;

pub enum TooltipStyle {
    Tooltip,
}

impl<'a, Message: 'a> ThemableWidget<'a, Message> for Tooltip<'a, Message, GauntletComplexTheme, Renderer> {
    type Kind = TooltipStyle;

    fn themed(self, kind: TooltipStyle) -> Element<'a, Message> {
        let theme = get_theme();

        match kind {
            TooltipStyle::Tooltip => self.class(ContainerStyleInner::Tooltip).padding(theme.tooltip.padding),
        }
        .into()
    }
}
