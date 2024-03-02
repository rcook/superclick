# ReaClick

ReaClick is a [CLAP][clap] click track generator plugin for CLAP hosts including
[REAPER][reaper]. I'm building this to generate signals for devices such as the
[Peterson Body Beat Pulse Solo][body-beat-pulse-solo].

[MIT License](LICENSE)

## Building

After installing [Rust][rustup], you can compile ReaClick as follows:

```bash
cargo xtask bundle reaclick --release
```

[body-beat-pulse-solo]: https://www.petersontuners.com/products/bodybeatpulse/
[clap]: https://cleveraudio.org/
[reaper]: https://reaper.fm/
[rustup]: (https://rustup.rs/)
