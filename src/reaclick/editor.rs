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
use super::data::DisplayDataRef;
use super::plugin::ReaClickParams;
use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::executor::Default;
use nih_plug_iced::{
    create_iced_editor, Color, Column, Command, Element, IcedEditor, IcedState, Text, WindowQueue,
};
use std::sync::Arc;

pub fn create_default_state() -> Arc<IcedState> {
    IcedState::from_size(200, 150)
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
}

#[derive(Debug, Clone, Copy)]
enum Message {}

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
        let (buffer_str, tempo_str, pos_str, time_sig_str) = {
            let display_data = self.display_data.lock().expect("TBD");
            let (tempo_str, pos_str, time_sig_str) =
                if let Some(ref playhead) = display_data.playhead {
                    (
                        format!("Tempo: {} qpm", playhead.tempo),
                        format!(
                            "Song position: {:04}/{:05.2}/{:05.2}",
                            playhead.bar_number,
                            playhead.bar_start_pos_crotchets,
                            playhead.pos_crotchets,
                        ),
                        format!(
                            "Time signature: {}/{}",
                            playhead.time_sig_numerator, playhead.time_sig_denominator
                        ),
                    )
                } else {
                    (
                        String::from("(Tempo unavailable)"),
                        String::from("(Song position unavailable)"),
                        String::from("(Time signature unavailable)"),
                    )
                };
            let buffer_str = format!(
                "{} / {:?} / {} / {}",
                display_data.sample_rate,
                display_data.min_buffer_size,
                display_data.max_buffer_size,
                display_data.samples
            );
            (buffer_str, tempo_str, pos_str, time_sig_str)
        };

        Column::new()
            .push(Text::new("ReaClick"))
            .push(Text::new(&buffer_str))
            .push(Text::new(&tempo_str))
            .push(Text::new(&pos_str))
            .push(Text::new(&time_sig_str))
            .into()
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
