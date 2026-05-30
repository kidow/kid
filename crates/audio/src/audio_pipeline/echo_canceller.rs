//! Echo cancellation is disabled in this fork: it only mattered for audio
//! calls, which are not supported here. This is a no-op on all platforms,
//! avoiding the heavy `libwebrtc`/`webrtc-sys` native dependency.

#[derive(Clone, Default)]
pub struct EchoCanceller;

impl EchoCanceller {
    pub fn process_reverse_stream(&mut self, _buf: &mut [i16]) {}
    pub fn process_stream(&mut self, _buf: &mut [i16]) -> anyhow::Result<()> {
        Ok(())
    }
}
