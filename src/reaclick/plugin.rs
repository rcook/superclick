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
use super::data::{TransportInfo, TransportInfoRef};
use super::editor::{create_default_state, create_editor};
use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use std::sync::Arc;

pub struct ReaClick {
    params: Arc<ReaClickParams>,
    transport_info: TransportInfoRef,
}

#[derive(Params)]
pub struct ReaClickParams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,
}

impl Default for ReaClick {
    fn default() -> Self {
        Self {
            params: Arc::new(ReaClickParams::default()),
            transport_info: TransportInfo::new(),
        }
    }
}

impl Default for ReaClickParams {
    fn default() -> Self {
        Self {
            editor_state: create_default_state(),
        }
    }
}

impl Plugin for ReaClick {
    const NAME: &'static str = "ReaClick";
    const VENDOR: &'static str = "Richard Cook";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "rcook@rcook.org";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        create_editor(
            self.params.clone(),
            self.transport_info.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        {
            if self.params.editor_state.is_open() {
                let mut transport_info = self.transport_info.lock().expect("TBD");
                if let Some(bar_number) = context.transport().bar_number() {
                    transport_info.bar_number = bar_number;
                }
                if let Some(bar_start_pos_beats) = context.transport().bar_start_pos_beats() {
                    transport_info.bar_start_pos_beats = bar_start_pos_beats;
                }
                if let Some(pos_beats) = context.transport().pos_beats() {
                    transport_info.pos_beats = pos_beats;
                }
                if let Some(time_sig_numerator) = context.transport().time_sig_numerator {
                    transport_info.time_sig_numerator = time_sig_numerator;
                }
                if let Some(time_sig_denominator) = context.transport().time_sig_denominator {
                    transport_info.time_sig_denominator = time_sig_denominator;
                }
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for ReaClick {
    const CLAP_ID: &'static str = "org.rcook.reaclick";
    const CLAP_DESCRIPTION: Option<&'static str> = Some(Self::NAME);
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}
