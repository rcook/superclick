# ReaClick

[![CI](https://github.com/rcook/reaclick/actions/workflows/ci.yaml/badge.svg)][ci-workflow]
[![Release](https://github.com/rcook/reaclick/actions/workflows/release.yaml/badge.svg)][release-workflow]

[GitHub Pages documentation][github-pages]

ReaClick is a [CLAP][clap] click track generator plugin for CLAP hosts including
[REAPER][reaper]. I'm building this to generate signals for devices such as the
[Peterson Body Beat Pulse Solo][body-beat-pulse-solo]. This project is at the
"proof of concept" stage: it generates an audible click in time with the song etc.

Released under [MIT License](LICENSE)

## Installation

See [GitHub Pages documentation][github-pages] for installation instructions.

## Development

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

[body-beat-pulse-solo]: https://www.petersontuners.com/products/bodybeatpulse/
[ci-workflow]: https://github.com/rcook/reaclick/actions/workflows/ci.yaml
[clap]: https://cleveraudio.org/
[github-pages]: https://rcook.github.io/reaclick/
[reaper]: https://reaper.fm/
[release-workflow]: https://github.com/rcook/reaclick/actions/workflows/release.yaml
[rustup]: (https://rustup.rs/)
