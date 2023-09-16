use crate::std::cmp::{max, min};
use crate::std::collections::VecDeque;
use crate::std::io;
use crate::std::io::*;

struct ShortReader {
    cap: usize,
    read_size: usize,
    observed_buffer: usize,
}

impl Read for ShortReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = min(self.cap, self.read_size).min(buf.len());
        self.cap -= bytes;
        self.observed_buffer = max(self.observed_buffer, buf.len());
        Ok(bytes)
    }
}

struct WriteObserver {
    observed_buffer: usize,
}

impl Write for WriteObserver {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.observed_buffer = max(self.observed_buffer, buf.len());
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
