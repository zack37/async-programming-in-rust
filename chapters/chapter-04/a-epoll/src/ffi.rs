pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLLIN: i32 = 0x1;
pub const EPOLLET: i32 = 1 << 31;

#[link(name = "c")]
extern "C" {
    pub fn epoll_create(size: i32) -> i32;
    pub fn close(fd: i32) -> i32;
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;
    pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}

// A nicer api over the extern functions
pub fn create_epoll() -> std::io::Result<i32> {
    let res = unsafe { epoll_create(1) };

    if res < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

pub fn wait_epoll(
    epfd: i32,
    events: *mut Event,
    maxevents: i32,
    timeout: i32,
) -> std::io::Result<i32> {
    let res = unsafe { epoll_wait(epfd, events, maxevents, timeout) };

    if res < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

pub fn ctl_epoll(epfd: i32, op: i32, fd: i32, event: *mut Event) -> std::io::Result<i32> {
    let res = unsafe { epoll_ctl(epfd, op, fd, event) };

    if res < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

pub fn close_epoll(fd: i32) -> std::io::Result<i32> {
    let res = unsafe { close(fd) };

    if res < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct Event {
    pub(crate) events: u32,
    pub(crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}
