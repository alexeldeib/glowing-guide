use epoll::{ControlOptions, Event, Events};
use eyre::Result;
use rustix::fd::AsRawFd;
use std::fs::{OpenOptions};
use std::io::{Write};

fn main() -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/proc/pressure/cpu")?;

    // proc/pressure/cpu is system-wide, can also use cgroups
    // .open("/sys/fs/cgroup/user.slice/user-0.slice/session-15.scope/cpu.pressure")?;

    // trigger an event when any process waits for cpu
    // for more than 100ms in 1 second.
    file.write_all(b"some 100000 1000000\0")?;

    let raw_fd = file.as_raw_fd();

    let close_on_exec = true; // does this even matter

    let epoll_fd = epoll::create(close_on_exec)?;

    let event = Event::new(Events::EPOLLPRI, 1);

    epoll::ctl(epoll_fd, ControlOptions::EPOLL_CTL_ADD, raw_fd, event)?;

    let mut event_buf = [Event { events: 0, data: 0 }];

    let n = epoll::wait(epoll_fd, -1, &mut event_buf)?;

    assert_eq!(n, 1);

    let event = event_buf[0];

    println!("events: {:#?}", event);

    Ok(())
}
