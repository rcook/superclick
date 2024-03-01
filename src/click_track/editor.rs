use super::data::TransportInfoRef;
use super::plugin::ClickTrackPluginParams;
use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::*;
use std::sync::Arc;

pub fn default_state() -> Arc<IcedState> {
    IcedState::from_size(200, 150)
}

pub fn create(
    params: Arc<ClickTrackPluginParams>,
    transport_info: TransportInfoRef,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<ClickTrackPluginEditor>(editor_state, (params, transport_info))
}

struct ClickTrackPluginEditor {
    #[allow(unused)]
    params: Arc<ClickTrackPluginParams>,
    context: Arc<dyn GuiContext>,
    transport_info: TransportInfoRef,
}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl IcedEditor for ClickTrackPluginEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = (Arc<ClickTrackPluginParams>, TransportInfoRef);

    fn new(
        (params, transport_info): Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = ClickTrackPluginEditor {
            params,
            context,
            transport_info,
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        _message: Self::Message,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let s = {
            let transport_info = self.transport_info.lock().expect("TBD");
            format!(
                "pos={:04}/{:05.2}/{:05.2} time_sig={}/{}",
                transport_info.bar_number,
                transport_info.bar_start_pos_beats,
                transport_info.pos_beats,
                transport_info.time_sig_numerator,
                transport_info.time_sig_denominator
            )
        };

        Column::new()
            .align_items(Alignment::Center)
            .push(
                Text::new("Click Track")
                    .font(assets::NOTO_SANS_LIGHT)
                    .size(40)
                    .height(50.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Bottom),
            )
            .push(
                Text::new(&s)
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(Space::with_height(10.into()))
            .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.98,
            g: 0.98,
            b: 0.98,
            a: 1.0,
        }
    }
}
