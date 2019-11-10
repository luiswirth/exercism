use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    byte_count: usize,
    read_count: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            byte_count: 0,
            read_count: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.byte_count
    }

    pub fn reads(&self) -> usize {
        self.read_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let byte_count = self.wrapped.read(buf)?;
        self.byte_count += byte_count;
        self.read_count += 1;
        Ok(byte_count)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    byte_count: usize,
    write_count: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            byte_count: 0,
            write_count: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.byte_count
    }

    pub fn writes(&self) -> usize {
        self.write_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let byte_count = self.wrapped.write(buf)?;
        self.byte_count += byte_count;
        self.write_count += 1;
        Ok(byte_count)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
