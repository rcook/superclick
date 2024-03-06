# ReaClick

[![CI](https://github.com/rcook/reaclick/actions/workflows/ci.yaml/badge.svg)][ci-workflow]
[![Release](https://github.com/rcook/reaclick/actions/workflows/release.yaml/badge.svg)][release-workflow]

[GitHub Pages documentation][github-pages]

ReaClick is a [CLAP][clap] click track generator plugin for CLAP hosts
including [REAPER][reaper]. I'm building this to generate signals for
devices such as the [Peterson Body Beat Pulse Solo][body-beat-pulse-solo].
This project is at the "proof of concept" stage: it generates an audible
click in time with the song etc.

This project makes uses of the [NIH-plug][nih-plug] framework. For those
interested in developing Rust applications for audio, please check out
[Rust Audio][rust-audio] and the [Rust Audio Discord][rust-audio-discord].

Released under [MIT License](LICENSE)

## Installation

See [GitHub Pages documentation][github-pages] for installation instructions.

## Development

After installing [Rust][rustup], you can use all of the usual [Cargo][cargo]
commands. You can also build a bundle as follows:
follows:

```bash
cargo xtask bundle reaclick
```

This will build the target and bundle it correctly as CLAP plugin&mdash;by
renaming the target binary to `ReaClick.clap` etc.

This command will report the location of the `.clap` file at the end,
e.g.

```text
Created a CLAP bundle at 'target/bundled/ReaClick.clap'
```

You can also bundle a release build as follows:

```bash
cargo xtask bundle reaclick --release
```

The absolute path to the bundle directory (i.e. `/path/to/target/bundled`
or `X:\path\to\target\bundled`) is the CLAP plugin path you will need to
add to the set of paths scanned by your host DAW.

[body-beat-pulse-solo]: https://www.petersontuners.com/products/bodybeatpulse/
[cargo]: https://doc.rust-lang.org/cargo/
[ci-workflow]: https://github.com/rcook/reaclick/actions/workflows/ci.yaml
[clap]: https://cleveraudio.org/
[github-pages]: https://rcook.github.io/reaclick/
[nih-plug]: https://github.com/robbert-vdh/nih-plug
[reaper]: https://reaper.fm/
[release-workflow]: https://github.com/rcook/reaclick/actions/workflows/release.yaml
[rust-audio]: https://rust.audio/
[rust-audio-discord]: https://discord.gg/8qW6q2k
[rustup]: (https://rustup.rs/)
