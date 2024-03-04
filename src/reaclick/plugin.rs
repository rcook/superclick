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
use super::click::{Channel, Click};
use super::display::{Display, Playhead};
use super::editor::create_editor;
use super::params::ReaClickParams;
use crate::error::Error;
use crate::music_theory::TimeSignatureTop;
use crate::package::{PACKAGE_HOME_PAGE, PACKAGE_VERSION};
use crate::result::{GetOr, Result};
use nih_plug::prelude::*;
use std::f32::consts;
use std::sync::Arc;

const LEFT_CHANNEL_ID: usize = 0;
const RIGHT_CHANNEL_ID: usize = 1;

pub struct ReaClick {
    params: Arc<ReaClickParams>,
    display: Arc<Display>,
    sample_rate: f32,
    phase: f32,
}

impl ReaClick {
    fn process_inner(
        &mut self,
        buffer: &mut Buffer,
        context: &mut impl ProcessContext<Self>,
    ) -> Result<Option<Playhead>> {
        let playhead = self.get_playhead(context.transport())?;

        if let Some(ref playhead) = playhead {
            self.write_samples(playhead, buffer);
        }

        Ok(playhead)
    }

    fn update_display(&self, result: Result<Option<Playhead>>) {
        if self.params.editor_state.is_open() {
            match result {
                Ok(playhead) => self.display.update(None, &playhead),
                Err(e) => self.display.update(Some(e), &None),
            }
        }
    }

    fn get_playhead(&self, transport: &Transport) -> Result<Option<Playhead>> {
        Ok(if transport.playing {
            let tempo = transport.tempo.get_or(Error::TempoUnavailable)?;
            let bar_number = transport.bar_number().get_or(Error::BarNumberUnavailable)?;
            let bar_start_pos_crotchets = transport
                .bar_start_pos_beats()
                .get_or(Error::BarStartPosBeatsUnavailable)?;
            let pos_crotchets = transport.pos_beats().get_or(Error::PosBeatsUnavailable)?;
            let time_sig_numerator = transport
                .time_sig_numerator
                .get_or(Error::TimeSigNumeratorUnavailable)?;
            let time_sig_denominator = transport
                .time_sig_denominator
                .get_or(Error::TimeSignDenominatorUnavailable)?;
            Some(Playhead {
                tempo,
                bar_number,
                bar_start_pos_crotchets,
                pos_crotchets,
                time_signature_top: time_sig_numerator.try_into()?,
                time_signature_bottom: time_sig_denominator.try_into()?,
            })
        } else {
            None
        })
    }

    fn calculate_sine(&mut self, frequency: f32) -> f32 {
        let phase_delta = frequency / self.sample_rate;
        let sine = (self.phase * consts::TAU).sin();

        self.phase += phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sine
    }

    fn write_samples(&mut self, playhead: &Playhead, buffer: &mut Buffer) {
        fn get_click(time_signature_top: TimeSignatureTop, index: i32) -> Click {
            if index == 0 {
                Click::ACCENT
            } else if time_signature_top.is_accented(index) {
                Click::SUBACCENT
            } else {
                Click::NORMAL
            }
        }

        let pos_in_bar_crotchets = playhead.pos_crotchets - playhead.bar_start_pos_crotchets;
        for i in 0..playhead.time_signature_top.as_number() {
            let click = get_click(playhead.time_signature_top, i);
            let temp = i as f64 * 4f64 / playhead.time_signature_bottom.as_number() as f64;
            if pos_in_bar_crotchets >= temp && pos_in_bar_crotchets <= temp + click.length {
                for channel_samples in buffer.iter_samples() {
                    let value = self.calculate_sine(click.frequency);

                    // There's probably a more efficient way to do this...
                    for (channel_id, sample) in channel_samples.into_iter().enumerate() {
                        let is_audible = match click.channel {
                            Channel::Left => channel_id == LEFT_CHANNEL_ID,
                            Channel::Right => channel_id == RIGHT_CHANNEL_ID,
                            Channel::Both => {
                                channel_id == LEFT_CHANNEL_ID || channel_id == RIGHT_CHANNEL_ID
                            }
                        };
                        if is_audible {
                            *sample = value;
                        }
                    }
                }
                break;
            }
        }
    }
}

impl Default for ReaClick {
    fn default() -> Self {
        Self {
            params: Arc::new(ReaClickParams::default()),
            display: Arc::new(Display::default()),
            sample_rate: 0f32,
            phase: 0f32,
        }
    }
}

impl Plugin for ReaClick {
    const NAME: &'static str = "ReaClick";
    const VENDOR: &'static str = "Richard Cook";
    const URL: &'static str = PACKAGE_HOME_PAGE;
    const EMAIL: &'static str = "rcook@rcook.org";
    const VERSION: &'static str = PACKAGE_VERSION;

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
            self.display.clone(),
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
        self.display.update(None, &None);
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let result = self.process_inner(buffer, context);
        self.update_display(result);
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
