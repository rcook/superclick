use nih_plug::prelude::*;
use std::sync::Arc;
use std::time::SystemTime;

// Inspired by https://github.com/AlexW00/clockwork/blob/master/src/lib.rs#L245

enum State {
    Idle,
    NoteOnSent,
    NoteOffSent,
}

pub struct ClickTrackPlugin {
    params: Arc<ClickTrackPluginParams>,
    state: State,
    note_on_at: SystemTime,
}

#[derive(Params)]
struct ClickTrackPluginParams {
    timing: u32,
    voice_id: Option<i32>,
    channel: u8,
    note: u8,
    velocity: f32,
    note_interval_millis: u128,
    note_length: u128,
}

impl Default for ClickTrackPluginParams {
    fn default() -> Self {
        Self {
            timing: 0,
            voice_id: None,
            channel: 1,
            note: 64,
            velocity: 1.0f32,
            note_interval_millis: 1000,
            note_length: 500,
        }
    }
}

impl ClickTrackPlugin {
    fn new_note_on(&self) -> NoteEvent<()> {
        NoteEvent::NoteOn {
            timing: self.params.timing,
            voice_id: self.params.voice_id,
            channel: self.params.channel,
            note: self.params.note,
            velocity: self.params.velocity,
        }
    }

    fn new_note_off(&self) -> NoteEvent<()> {
        NoteEvent::NoteOff {
            timing: self.params.timing,
            voice_id: self.params.voice_id,
            channel: self.params.channel,
            note: self.params.note,
            velocity: self.params.velocity,
        }
    }
}

impl Default for ClickTrackPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(ClickTrackPluginParams::default()),
            state: State::Idle,
            note_on_at: SystemTime::UNIX_EPOCH,
        }
    }
}

impl Plugin for ClickTrackPlugin {
    const NAME: &'static str = "Richard's Click Track Plugin";
    const VENDOR: &'static str = "Richard Cook";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "rcook@rcook.org";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];
    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let now = SystemTime::now();

        match self.state {
            State::Idle => {
                context.send_event(self.new_note_on());
                self.state = State::NoteOnSent;
                self.note_on_at = now;
            }
            State::NoteOnSent => {
                let millis = now
                    .duration_since(self.note_on_at)
                    .map(|d| d.as_millis())
                    .unwrap_or(0);
                if millis > self.params.note_length {
                    context.send_event(self.new_note_off());
                    self.state = State::NoteOffSent;
                }
            }
            State::NoteOffSent => {
                let millis = now
                    .duration_since(self.note_on_at)
                    .map(|d| d.as_millis())
                    .unwrap_or(0);
                if millis > self.params.note_interval_millis {
                    context.send_event(self.new_note_on());
                    self.state = State::NoteOnSent;
                    self.note_on_at = now;
                }
            }
        };

        ProcessStatus::Normal
    }
}

impl ClapPlugin for ClickTrackPlugin {
    const CLAP_ID: &'static str = "org.rcook.click-track-plugin";
    const CLAP_DESCRIPTION: Option<&'static str> = Some(Self::NAME);
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::NoteEffect, ClapFeature::Utility];
}
