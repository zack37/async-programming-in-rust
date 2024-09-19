use crate::ffi;
use std::{
    io::{self, Result},
    net::TcpStream,
    os::fd::AsRawFd,
};

type Events = Vec<ffi::Event>;

pub struct Poll {
    registry: Registry,
}

impl Poll {
    pub fn new() -> Result<Self> {
        let res = ffi::create_epoll()?;

        Ok(Self {
            registry: Registry { raw_fd: res },
        })
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        let fd = self.registry.raw_fd;
        let timeout = timeout.unwrap_or(-1);
        let max_events = events.capacity() as i32;
        let res = ffi::wait_epoll(fd, events.as_mut_ptr(), max_events, timeout)?;

        unsafe { events.set_len(res as usize) };

        Ok(())
    }
}

pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    pub fn register(&self, source: &TcpStream, token: usize, interests: i32) -> Result<()> {
        let mut event = ffi::Event {
            events: interests as u32,
            epoll_data: token,
        };

        let op = ffi::EPOLL_CTL_ADD;
        let _ = ffi::ctl_epoll(self.raw_fd, op, source.as_raw_fd(), &mut event)?;

        Ok(())
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        if let Err(err) = ffi::close_epoll(self.raw_fd) {
            eprintln!("ERROR: {err:?}");
        }
    }
}
