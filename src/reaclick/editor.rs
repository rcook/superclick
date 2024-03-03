// Copyright (c) 2024 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use super::data::{DisplayData, DisplayDataRef};
use super::plugin::ReaClickParams;
use nih_plug::nih_error;
use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::button;
use nih_plug_iced::executor::Default;
use nih_plug_iced::{
    create_iced_editor, Button, Color, Column, Command, Element, IcedEditor, IcedState, Text,
    WindowQueue,
};
use std::sync::Arc;

pub fn create_default_state() -> Arc<IcedState> {
    IcedState::from_size(400, 300)
}

pub fn create_editor(
    params: Arc<ReaClickParams>,
    display_data: DisplayDataRef,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<ReaClickEditor>(
        editor_state,
        ReaClickEditorInitializationFlags {
            params,
            display_data,
        },
    )
}

#[derive(Clone)]
struct ReaClickEditorInitializationFlags {
    params: Arc<ReaClickParams>,
    display_data: DisplayDataRef,
}

struct ReaClickEditor {
    #[allow(unused)]
    params: Arc<ReaClickParams>,
    context: Arc<dyn GuiContext>,
    display_data: DisplayDataRef,
    report_bug_button_state: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ReportBugButtonPressed,
}

struct DisplayStrings {
    buffer: String,
    tempo: String,
    song_position: String,
    time_signature: String,
    error: Option<String>,
}

impl DisplayStrings {
    fn from_display_data(display_data: &DisplayData) -> Self {
        let buffer = format!(
            "fs={} spls={} ch={}",
            display_data.sample_rate, display_data.samples, display_data.channels,
        );

        let error = display_data
            .error
            .as_ref()
            .map(|e| format!("An error occurred: {:?}", e));

        if let Some(ref playhead) = display_data.playhead {
            Self {
                buffer,
                tempo: format!("Tempo: {} qpm", playhead.tempo),
                song_position: format!(
                    "Song position: {:04}/{:05.2}/{:05.2}",
                    playhead.bar_number, playhead.bar_start_pos_crotchets, playhead.pos_crotchets,
                ),
                time_signature: format!(
                    "Time signature: {}/{}",
                    playhead.time_signature_top, playhead.time_signature_bottom
                ),
                error,
            }
        } else {
            Self {
                buffer,
                tempo: String::from("(Tempo unavailable)"),
                song_position: String::from("(Song position unavailable)"),
                time_signature: String::from("(Time signature unavailable)"),
                error,
            }
        }
    }
}

impl IcedEditor for ReaClickEditor {
    type Executor = Default;
    type Message = Message;
    type InitializationFlags = ReaClickEditorInitializationFlags;

    fn new(
        initialization_flags: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = ReaClickEditor {
            params: initialization_flags.params,
            context,
            display_data: initialization_flags.display_data,
            report_bug_button_state: button::State::default(),
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Self::Message::ReportBugButtonPressed => {
                if let Err(e) = webbrowser::open(env!("CARGO_PKG_HOMEPAGE")) {
                    nih_error!("{}", e);
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let strs = {
            let display_data = self.display_data.lock().expect("TBD");
            DisplayStrings::from_display_data(&display_data)
        };

        let mut column = Column::new();

        if let Some(e) = strs.error {
            column = column.push(Text::new(e)).push(
                Button::new(&mut self.report_bug_button_state, Text::new("Report bug"))
                    .on_press(Self::Message::ReportBugButtonPressed),
            );
        }

        column = column
            .push(Text::new(&strs.tempo))
            .push(Text::new(&strs.song_position))
            .push(Text::new(&strs.time_signature))
            .push(Text::new(&strs.buffer));

        column.into()
    }

    fn background_color(&self) -> Color {
        Color {
            r: 0.58,
            g: 0.98,
            b: 0.58,
            a: 1.0,
        }
    }
}
