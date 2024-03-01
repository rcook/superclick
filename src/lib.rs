mod audio_plugin;
mod click_track;
mod gui_plugin;
mod midi_plugin;
mod other_audio_plugin;

use audio_plugin::AudioPlugin;
use click_track::ClickTrackPlugin;
use gui_plugin::GuiPlugin;
use midi_plugin::MidiPlugin;
use nih_plug::nih_export_clap;
use other_audio_plugin::OtherAudioPlugin;

nih_export_clap!(
    AudioPlugin,
    ClickTrackPlugin,
    GuiPlugin,
    MidiPlugin,
    OtherAudioPlugin
);
