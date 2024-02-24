use nih_plug::prelude::*;
use std::sync::Arc;

pub struct MidiPlugin {
    params: Arc<MidiPluginParams>,
}

#[derive(Default, Params)]
struct MidiPluginParams {}

impl Default for MidiPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(MidiPluginParams::default()),
        }
    }
}

impl Plugin for MidiPlugin {
    const NAME: &'static str = "Richard's MIDI Plugin";
    const VENDOR: &'static str = "Richard Cook";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "rcook@rcook.org";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // This plugin doesn't have any audio IO
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
        while let Some(event) = context.next_event() {
            nih_log!("{event:?}");
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for MidiPlugin {
    const CLAP_ID: &'static str = "org.rcook.midi-plugin";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Richard's MIDI Plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::NoteEffect, ClapFeature::Utility];
}
