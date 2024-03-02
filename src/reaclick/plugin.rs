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
use super::editor::{create_default_state, create_editor};
use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use std::f32::consts;
use std::sync::Arc;

const BAR_FREQUENCY: f32 = 400f32;
const ACCENT_FREQUENCY: f32 = 800f32;
const NORMAL_FREQUENCY: f32 = 1600f32;
const CLICK_LENGTH: f64 = 0.125f64;

pub struct ReaClick {
    params: Arc<ReaClickParams>,
    info: DisplayDataRef,
    sample_rate: f32,
    phase: f32,
}

impl ReaClick {
    fn calculate_sine(&mut self, frequency: f32) -> f32 {
        let phase_delta = frequency / self.sample_rate;
        let sine = (self.phase * consts::TAU).sin();

        self.phase += phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sine
    }
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
            info: DisplayData::new(),
            sample_rate: 0f32,
            phase: 0f32,
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
            self.info.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;

        let mut info = self.info.lock().expect("TBD");
        info.sample_rate = buffer_config.sample_rate;
        info.min_buffer_size = buffer_config.min_buffer_size;
        info.max_buffer_size = buffer_config.max_buffer_size;
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let transport = context.transport();
        let bar_start_pos_crotchets = transport.bar_start_pos_beats();
        let pos_crotchets = transport.pos_beats();
        let time_sig_numerator = transport.time_sig_numerator;

        if self.params.editor_state.is_open() {
            let mut info = self.info.lock().expect("TBD");

            info.samples = buffer.samples();

            if let Some(tempo) = transport.tempo {
                info.tempo = tempo;
            }
            if let Some(bar_number) = transport.bar_number() {
                info.bar_number = bar_number;
            }
            if let Some(value) = bar_start_pos_crotchets {
                info.bar_start_pos_crotchets = value;
            }
            if let Some(value) = pos_crotchets {
                info.pos_crotchets = value;
            }
            if let Some(time_sig_numerator) = time_sig_numerator {
                info.time_sig_numerator = time_sig_numerator;
            }
            if let Some(time_sig_denominator) = transport.time_sig_denominator {
                info.time_sig_denominator = time_sig_denominator;
            }
        }

        if transport.playing {
            fn beat_crotchets(time_sig_numerator: i32) -> f64 {
                match time_sig_numerator {
                    3 => 1f64,
                    4 => 1f64,
                    6 => 0.5f64,
                    _ => 1f64, /* TBD */
                }
            }

            fn is_accent(time_sig_numerator: i32, i: i32) -> bool {
                match time_sig_numerator {
                    3 => false,
                    4 => i == 1 || i == 3,
                    6 => i == 3,
                    _ => false, /* TBD */
                }
            }

            let x = pos_crotchets.expect("TBD") - bar_start_pos_crotchets.expect("TBD");
            let time_sig_numerator = time_sig_numerator.expect("TBD");
            let y = beat_crotchets(time_sig_numerator);
            for i in 0..time_sig_numerator {
                let f = if i == 0 {
                    BAR_FREQUENCY
                } else if is_accent(time_sig_numerator, i) {
                    ACCENT_FREQUENCY
                } else {
                    NORMAL_FREQUENCY
                };
                let temp = (i as f64) * y;
                if x >= temp && x <= temp + CLICK_LENGTH {
                    for channel_samples in buffer.iter_samples() {
                        let value = self.calculate_sine(f);
                        for sample in channel_samples {
                            *sample = value;
                        }
                    }
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
