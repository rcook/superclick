mod audio_plugin;
mod other_audio_plugin;

use audio_plugin::AudioPlugin;
use nih_plug::nih_export_clap;
use other_audio_plugin::OtherAudioPlugin;

nih_export_clap!(AudioPlugin, OtherAudioPlugin);
