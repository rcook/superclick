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
use super::data::{DisplayData, DisplayDataRef, Playhead};
use super::editor::{create_default_state, create_editor};
use crate::music_theory::TimeSignatureTop;
use anyhow::{Context, Result};
use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use std::f32::consts;
use std::sync::Arc;

pub enum Channel {
    Left,
    Right,
    Both,
}

pub struct Click {
    channel: Channel,
    frequency: f32,
    length: f64,
}

// Body Beat Pulse Solo settings

// Body Beat Pulse Solo accent (high-intensity) click
const ACCENT_CLICK: Click = Click {
    channel: Channel::Right,
    frequency: 400f32,
    length: 0.125f64,
};

// Body Beat Pulse Solo subaccent (medium-intensity) click
const SUBACCENT_CLICK: Click = Click {
    channel: Channel::Left,
    frequency: 800f32,
    length: 0.125f64,
};

// Body Beat Pulse Solo normal (low-intensity) click
const NORMAL_CLICK: Click = Click {
    channel: Channel::Both,
    frequency: 1_600f32,
    length: 0.125f64,
};

pub struct ReaClick {
    params: Arc<ReaClickParams>,
    info: DisplayDataRef,
    sample_rate: f32,
    phase: f32,
}

impl ReaClick {
    fn process_inner(
        &mut self,
        buffer: &mut Buffer,
        context: &mut impl ProcessContext<Self>,
    ) -> Result<()> {
        let playhead = self.get_playhead(context.transport())?;

        if let Some(ref playhead) = playhead {
            self.write_samples(playhead, buffer);
        }

        if self.params.editor_state.is_open() {
            let mut info = self.info.lock().expect("lock poisoned");
            info.samples = buffer.samples();
            info.channels = buffer.channels();
            info.playhead = playhead;
        }

        Ok(())
    }

    fn get_playhead(&self, transport: &Transport) -> Result<Option<Playhead>> {
        Ok(if transport.playing {
            let tempo = transport.tempo.context("tempo is unavailable")?;
            let bar_number = transport
                .bar_number()
                .context("bar number is unavailable")?;
            let bar_start_pos_crotchets = transport
                .bar_start_pos_beats()
                .context("bar start position is unavailable")?;
            let pos_crotchets = transport.pos_beats().context("position is unavailable")?;
            let time_signature_top = transport
                .time_sig_numerator
                .context("time signature numerator is unavailable")?
                .try_into()?;
            let time_signature_bottom = transport
                .time_sig_denominator
                .context("time signature denominator is unavailable")?
                .try_into()?;

            Some(Playhead {
                tempo,
                bar_number,
                bar_start_pos_crotchets,
                pos_crotchets,
                time_signature_top,
                time_signature_bottom,
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
        fn get_click(time_signature_top: TimeSignatureTop, note_index: i32) -> Click {
            if note_index == 0 {
                ACCENT_CLICK
            } else if time_signature_top.is_subaccent(note_index) {
                SUBACCENT_CLICK
            } else {
                NORMAL_CLICK
            }
        }

        let x = playhead.pos_crotchets - playhead.bar_start_pos_crotchets;
        let beat = playhead.time_signature_top.beat();
        for i in 0..playhead.time_signature_top.as_number() {
            let click = get_click(playhead.time_signature_top, i);
            let temp = (i as f64) * beat;
            if x >= temp && x <= temp + click.length {
                for channel_samples in buffer.iter_samples() {
                    let value = self.calculate_sine(click.frequency);

                    // There's probably a more efficient way to do this...
                    for (channel_id, sample) in channel_samples.into_iter().enumerate() {
                        let is_audible = match click.channel {
                            Channel::Left => channel_id == 0,
                            Channel::Right => channel_id == 1,
                            Channel::Both => channel_id == 0 || channel_id == 1,
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
        let mut info = self.info.lock().expect("lock poisoned");
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
        match self.process_inner(buffer, context) {
            Ok(()) => ProcessStatus::Normal,
            Err(_) => ProcessStatus::Error("something went wrong"),
        }
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
