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
use crate::package::{PACKAGE_BUILD_VERSION, PACKAGE_HOME_PAGE, PACKAGE_NAME, PACKAGE_VERSION};
use crate::reaclick::display::Display;
use crate::reaclick::params::ReaClickParams;
use nih_plug::prelude::{util, Editor};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

enum EditorEvent {
    Increment,
}

#[derive(Lens)]
struct EditorData {
    params: Arc<ReaClickParams>,
    display: Arc<Display>,
    count: i32,
}

impl Model for EditorData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            EditorEvent::Increment => self.count += 1,
        });
    }
}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub fn create_default_editor_state() -> Arc<ViziaState> {
    ViziaState::new(|| (200, 150))
}

pub fn create_editor(
    params: Arc<ReaClickParams>,
    display: Arc<Display>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        EditorData {
            params: params.clone(),
            display: display.clone(),
            count: 0,
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Button::new(
                cx,
                |cx| cx.emit(EditorEvent::Increment),
                |cx| Label::new(cx, "Test"),
            );
            Label::new(cx, EditorData::count)
                .width(Pixels(50.0))
                .live(Live::Polite);
            Label::new(cx, "Gain GUI")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Thin)
                .font_size(30.0)
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            Label::new(cx, "Gain").top(Pixels(10.0));
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));

        ResizeHandle::new(cx);
    })
}

struct DisplayStrings {
    title: String,
    song_position: String,
    tempo: Option<String>,
    big: Option<String>,
    error: Option<String>,
}

impl DisplayStrings {
    fn new(editor_state: &ViziaState, display: &Display) -> Self {
        let title = Self::format_title(editor_state);

        let error_code = display.error_code();
        let error = if error_code == isize::default() {
            None
        } else {
            Some(format!("Error: {}", error_code))
        };

        match (
            display.is_playing(),
            display.time_signature_top(),
            display.time_signature_bottom(),
        ) {
            (true, Some(time_signature_top), Some(time_signature_bottom)) => Self {
                title,
                song_position: format!(
                    "Song position: {:04}/{:05.2}/{:05.2}",
                    display.bar_number(),
                    display.bar_start_pos_crotchets(),
                    display.pos_crotchets(),
                ),
                tempo: Some(format!(
                    "Tempo: {:.1} qpm / {:.1} bpm",
                    display.tempo(),
                    display.tempo() * time_signature_bottom.as_number() as f64
                        / (4 * time_signature_top.basis()) as f64
                )),
                big: Some(format!(
                    "{} of {}/{}",
                    ((display.pos_crotchets() - display.bar_start_pos_crotchets())
                        * time_signature_bottom.as_number() as f64
                        / 4f64)
                        .trunc() as i32
                        + 1,
                    time_signature_top,
                    time_signature_bottom,
                )),
                error,
            },
            _ => Self {
                title,
                song_position: String::from("(Idle)"),
                tempo: None,
                big: None,
                error,
            },
        }
    }

    #[cfg(debug_assertions)]
    fn format_title(editor_state: &ViziaState) -> String {
        match PACKAGE_BUILD_VERSION {
            Some(ref build_version) => {
                format!(
                    "{} v{} ({}) [{:?}]",
                    PACKAGE_NAME,
                    PACKAGE_VERSION,
                    build_version,
                    editor_state.scaled_logical_size()
                )
            }
            None => format!(
                "{} v{} [{:?}]",
                PACKAGE_NAME,
                PACKAGE_VERSION,
                editor_state.scaled_logical_size()
            ),
        }
    }

    #[cfg(not(debug_assertions))]
    fn format_title(_editor_state: &IcedState) -> String {
        match PACKAGE_BUILD_VERSION {
            Some(ref build_version) => {
                format!("{} v{} ({})", PACKAGE_NAME, PACKAGE_VERSION, build_version)
            }
            None => format!("{} v{}", PACKAGE_NAME, PACKAGE_VERSION,),
        }
    }
}
