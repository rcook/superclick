# ReaClick (proof of concept)

ReaClick is a [CLAP][clap] click track generator plugin for CLAP hosts including
[REAPER][reaper]. I'm building this to generate signals for devices such as the
[Peterson Body Beat Pulse Solo][body-beat-pulse-solo]. This project is at the
"proof of concept" stage: it generates an audible click in time with the song etc.

[MIT License](LICENSE)

## Building

After installing [Rust][rustup], you can bundle a release build of ReaClick as
follows:

```bash
cargo xtask bundle reaclick --release
```

Or a debug build as follows:

```bash
cargo xtask bundle reaclick
```

These commands will build the project if necessary and create a CLAP bundle under `X:\path\to\reaclick\target\bundled` on Windows.

## Running inside REAPER (Windows)

This plugin has been developed primarily to target Windows x64.

0. Start REAPER
1. Go to _Options | Preferences&hellip;_
2. Go to _Plug-ins | LV2/CLAP_
3. Add the path to the bundle (e.g. `X:\path\to\reaclick\target\bundled`) to the list
of directories under _CLAP plug-in paths (can be multiple paths separated by
semicolons)_
4. Click _Re-scan | Re-scan CLAP paths for new/modified plug-ins_
5. Double-click in the track panel to create a new, empty track
6. Click on the _FX_ button and add an instance of _CLAP: ReaClick (Richard Cook)_
7. Profit!

## Running inside REAPER (macOS/Linux)

This plugin has not been tested on either macOS or Linux. I do not have a macOS machine
to test but I do know that it currently crashes. There is no reason in principle why these platforms should not work.

## Running inside other DAWs

Any reasonable CLAP host application should be able to run this plugin.

[body-beat-pulse-solo]: https://www.petersontuners.com/products/bodybeatpulse/
[clap]: https://cleveraudio.org/
[reaper]: https://reaper.fm/
[rustup]: (https://rustup.rs/)
