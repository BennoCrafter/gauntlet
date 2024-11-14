use crate::components::shortcut_selector::ShortcutSelector;
use crate::theme::shortcut_selector::ShortcutSelectorStyle;
use crate::theme::text::TextStyle;
use crate::theme::Element;
use common::model::PhysicalShortcut;
use common::rpc::backend_api::{BackendApi, BackendApiError};
use iced::alignment::Horizontal;
use iced::widget::text::Shaping;
use iced::widget::tooltip::Position;
use iced::widget::{column, container, row, text, tooltip, Space};
use iced::{alignment, Alignment, Command, Length, Padding};
use iced_aw::core::icons;
use crate::theme::container::ContainerStyle;

pub struct ManagementAppGeneralState {
    backend_api: Option<BackendApi>,
    current_shortcut: Option<PhysicalShortcut>,
    current_shortcut_error: Option<String>,
    currently_capturing: bool
}

#[derive(Debug, Clone)]
pub enum ManagementAppGeneralMsgIn {
    ShortcutCaptured(Option<PhysicalShortcut>),
    CapturingChanged(bool),
    RefreshShortcut {
        shortcut: Option<PhysicalShortcut>,
        error: Option<String>
    },
    Noop
}

#[derive(Debug, Clone)]
pub enum ManagementAppGeneralMsgOut {
    Noop,
    HandleBackendError(BackendApiError)
}

impl ManagementAppGeneralState {
    pub fn new(backend_api: Option<BackendApi>) -> Self {
        Self {
            backend_api,
            current_shortcut: None,
            current_shortcut_error: None,
            currently_capturing: false,
        }
    }

    pub fn update(&mut self, message: ManagementAppGeneralMsgIn) -> Command<ManagementAppGeneralMsgOut> {
        let backend_api = match &self.backend_api {
            Some(backend_api) => backend_api.clone(),
            None => {
                return Command::none()
            }
        };

        match message {
            ManagementAppGeneralMsgIn::ShortcutCaptured(shortcut) => {
                self.current_shortcut = shortcut.clone();

                let mut backend_api = backend_api.clone();

                Command::perform(async move {
                    backend_api.set_global_shortcut(shortcut)
                        .await?;

                    Ok(())
                }, |result| handle_backend_error(result, |()| ManagementAppGeneralMsgOut::Noop))
            }
            ManagementAppGeneralMsgIn::Noop => {
                Command::none()
            }
            ManagementAppGeneralMsgIn::RefreshShortcut { shortcut, error } => {
                self.current_shortcut = shortcut;
                self.current_shortcut_error = error;

                Command::perform(async move {}, |_| ManagementAppGeneralMsgOut::Noop)
            }
            ManagementAppGeneralMsgIn::CapturingChanged(capturing) => {
                self.currently_capturing = capturing;

                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<ManagementAppGeneralMsgIn> {

        let shortcut_selector: Element<_> = ShortcutSelector::new(
            &self.current_shortcut,
            move |value| { ManagementAppGeneralMsgIn::ShortcutCaptured(value) },
            move |value| { ManagementAppGeneralMsgIn::CapturingChanged(value) },
            ShortcutSelectorStyle::Default
        ).into();

        let field: Element<_> = container(shortcut_selector)
            .width(Length::Fill)
            .height(Length::Fixed(35.0))
            .into();

        let field = self.view_field("Global Shortcut", field.into());

        let content: Element<_> = column(vec![field])
            .into();

        let content: Element<_> = container(content)
            .width(Length::Fill)
            .into();

        let content: Element<_> = container(content)
            .width(Length::Fill)
            .into();

        content
    }

    fn view_field<'a>(&self, label: &str, input: Element<'a, ManagementAppGeneralMsgIn>) -> Element<'a, ManagementAppGeneralMsgIn> {
        let label: Element<_> = text(label)
            .shaping(Shaping::Advanced)
            .horizontal_alignment(Horizontal::Right)
            .width(Length::Fill)
            .into();

        let label: Element<_> = container(label)
            .width(Length::FillPortion(3))
            .padding(4)
            .into();

        let input_field = container(input)
            .width(Length::FillPortion(3))
            .padding(4)
            .into();

        let after = if self.currently_capturing {
            let hint1: Element<_> = text("Backspace - Unset Shortcut")
                .width(Length::Fill)
                .style(TextStyle::Subtitle)
                .into();

            let hint2: Element<_> = text("Escape - Stop Capturing")
                .width(Length::Fill)
                .style(TextStyle::Subtitle)
                .into();

            column(vec![hint1, hint2])
                .width(Length::FillPortion(3))
                .align_items(Alignment::Center)
                .padding(Padding::from([0.0, 8.0]))
                .into()
        } else {
            if let Some(current_shortcut_error) = &self.current_shortcut_error {
                let error_icon: Element<_> = text(icons::Bootstrap::ExclamationTriangleFill)
                    .font(icons::BOOTSTRAP_FONT)
                    .style(TextStyle::Destructive)
                    .into();

                let error_text: Element<_> = text(current_shortcut_error)
                    .style(TextStyle::Destructive)
                    .into();

                let error_text: Element<_> = container(error_text)
                    .padding(16.0)
                    .max_width(300)
                    .style(ContainerStyle::Box)
                    .into();

                let tooltip: Element<_> = tooltip(error_icon, error_text, Position::Bottom)
                    .into();

                let content = container(tooltip)
                    .width(Length::FillPortion(3))
                    .align_y(alignment::Vertical::Center)
                    .padding(Padding::from([0.0, 8.0]))
                    .into();

                content
            } else {
                Space::with_width(Length::FillPortion(3))
                    .into()
            }
        };

        let content = vec![
            label,
            input_field,
            after,
        ];

        let row: Element<_> = row(content)
            .align_items(Alignment::Center)
            .padding(12)
            .into();

        row
    }
}

pub fn handle_backend_error<T>(result: Result<T, BackendApiError>, convert: impl FnOnce(T) -> ManagementAppGeneralMsgOut) -> ManagementAppGeneralMsgOut {
    match result {
        Ok(val) => convert(val),
        Err(err) => ManagementAppGeneralMsgOut::HandleBackendError(err)
    }
}
