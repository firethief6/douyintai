//! 最小 BSD socket 子集（Linux/macOS/Windows 宿主）：够 mssdk 初始化探网点。
//! accept 非阻塞，回 EAGAIN 让 guest 自己重试，避免把模拟器挂死。
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream, UdpSocket};

use crate::{Emulator, Result};

const AF_INET: u16 = 2;

struct Tcp {
    kind: u64,
    bind: Option<SocketAddrV4>,
    listener: Option<TcpListener>,
    stream: Option<TcpStream>,
}
enum Sock {
    Tcp(Tcp),
    Udp(Option<UdpSocket>),
    /// AF_UNIX / netlink 等：不真连，按成功处理，日志走不出去无所谓
    #[allow(dead_code)]
    Other(u64),
}

struct Table {
    next: u64,
    fds: HashMap<u64, Sock>,
}
thread_local! {
    static TABLE: RefCell<Table> = RefCell::new(Table { next: 0x1000, fds: HashMap::new() });
}

fn errno(n: i64) -> u64 { (-n) as u64 }
fn io_err(e: &std::io::Error, fallback: i64) -> u64 {
    errno(e.raw_os_error().map(|x| x as i64).unwrap_or(fallback))
}

impl Emulator {
    /// pselect6 / ppoll：没有真实阻塞，按超时上限做一次短睡后回 0（超时）
    pub(crate) fn poll_timeout(&mut self, a: [u64; 6]) -> Result<u64> {
        let ts = if a[2] != 0 { self.read_bytes(a[2], 16).ok() } else { None };
        let wait = ts.map(|b| {
            let sec = i64::from_le_bytes(b[0..8].try_into().unwrap());
            let nsec = i64::from_le_bytes(b[8..16].try_into().unwrap());
            (sec.max(0) as u64) * 1000 + ((nsec.max(0) as u64) / 1_000_000)
        });
        let ms = wait.unwrap_or(1).clamp(1, 2);
        if std::env::var("DYSIGN_NET_TRACE").is_ok() { eprintln!("[poll] ms={ms}"); }
        let t = std::time::Instant::now();
        std::thread::sleep(std::time::Duration::from_millis(ms));
        crate::profile::on_sleep(t.elapsed());
        Ok(0)
    }

    pub(crate) fn net_syscall(&mut self, nr: u64, a: [u64; 6]) -> Result<u64> {
        if std::env::var("DYSIGN_NET_TRACE").is_ok() {
            let sa = self.read_sockaddr(a[1]).map(|x| x.to_string()).unwrap_or_default();
            eprintln!("[net] nr={nr} fd={} sa={sa}", a[0]);
        }
        match nr {
            198 => {
                let kind = a[1] & 0xf;
                let fd = TABLE.with(|t| {
                    let mut t = t.borrow_mut();
                    let fd = t.next;
                    t.next += 1;
                    let dom = a[0] & 0xff;
                    if kind == 2 { t.fds.insert(fd, Sock::Udp(None)); }
                    else if dom != 2 { t.fds.insert(fd, Sock::Other(dom)); }
                    else { t.fds.insert(fd, Sock::Tcp(Tcp { kind, bind: None, listener: None, stream: None })); }
                    fd
                });
                Ok(fd)
            }
            199 => Ok(errno(38)),
            200 => {
                let Some(addr) = self.read_sockaddr(a[1]) else { return Ok(errno(22)) };
                let mut out = errno(98);
                TABLE.with(|t| {
                    let mut t = t.borrow_mut();
                    match t.fds.get_mut(&a[0]) {
                        Some(Sock::Udp(slot)) => match UdpSocket::bind(addr) {
                            Ok(u) => { let _ = u.set_nonblocking(true); *slot = Some(u); out = 0; }
                            Err(e) => out = io_err(&e, 98),
                        },
                        Some(Sock::Other(_)) => { out = 0; }
                        Some(Sock::Tcp(s)) if s.bind.is_none() => match TcpListener::bind(addr) {
                            Ok(l) => {
                                let _ = l.set_nonblocking(true);
                                s.bind = l.local_addr().ok().and_then(|x| match x { SocketAddr::V4(v) => Some(v), _ => None });
                                s.listener = Some(l);
                                out = 0;
                            }
                            Err(e) => out = io_err(&e, 98),
                        },
                        _ => {}
                    }
                });
                if out == 0 {
                    let local = TABLE.with(|t| match t.borrow().fds.get(&a[0]) {
                        Some(Sock::Tcp(s)) => s.bind,
                        Some(Sock::Udp(Some(u))) => match u.local_addr() { Ok(SocketAddr::V4(v)) => Some(v), _ => None },
                        _ => None,
                    });
                    self.write_sockaddr(a[1], a[2], local);
                }
                Ok(out)
            }
            201 => Ok(0),
            202 | 242 => {
                let mut err = errno(11);
                let mut peer = None;
                let fd = TABLE.with(|t| {
                    let mut t = t.borrow_mut();
                    let accepted = match t.fds.get(&a[0]) {
                        Some(Sock::Tcp(s)) => match &s.listener {
                            Some(l) => Some((s.kind, l.accept())),
                            None => None,
                        },
                        _ => None,
                    };
                    if let Some((kind, result)) = accepted {
                        {
                            match result {
                                Ok((stream, p)) => {
                                    let _ = stream.set_nonblocking(true);
                                    let nfd = t.next;
                                    t.next += 1;
                                    t.fds.insert(nfd, Sock::Tcp(Tcp { kind, bind: None, listener: None, stream: Some(stream) }));
                                    peer = match p { SocketAddr::V4(v) => Some(v), _ => None };
                                    err = 0;
                                    return nfd;
                                }
                                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => err = errno(11),
                                Err(e) => err = io_err(&e, 11),
                            }
                        }
                    }
                    0
                });
                if err == 0 {
                    self.write_sockaddr(a[1], a[2], peer);
                    Ok(fd)
                } else { Ok(err) }
            }
            203 => {
                let other = TABLE.with(|t| matches!(t.borrow().fds.get(&a[0]), Some(Sock::Other(_))));
                if other { return Ok(0); }
                let Some(addr) = self.read_sockaddr(a[1]) else { return Ok(0) };
                let res = TcpStream::connect_timeout(&addr.into(), std::time::Duration::from_secs(3));
                let mut out = errno(111);
                TABLE.with(|t| {
                    let mut t = t.borrow_mut();
                    if let Some(Sock::Tcp(s)) = t.fds.get_mut(&a[0]) {
                        match res {
                            Ok(st) => { let _ = st.set_nonblocking(true); s.stream = Some(st); out = 0; }
                            Err(e) => out = io_err(&e, 111),
                        }
                    }
                });
                Ok(out)
            }
            204 => {
                let local = TABLE.with(|t| {
                    let t = t.borrow();
                    match t.fds.get(&a[0]) {
                        Some(Sock::Tcp(s)) => s.bind,
                        Some(Sock::Udp(Some(u))) => u.local_addr().ok().and_then(|x| match x { SocketAddr::V4(v) => Some(v), _ => None }),
                        _ => None,
                    }
                });
                match local { Some(x) => { self.write_sockaddr(a[1], a[2], Some(x)); Ok(0) } None => Ok(errno(22)) }
            }
            205 => Ok(errno(22)),
            206 => {
                let buf = self.read_bytes(a[1], a[2] as usize)?;
                let mut out = errno(9);
                TABLE.with(|t| {
                    let mut t = t.borrow_mut();
                    match t.fds.get_mut(&a[0]) {
                        Some(Sock::Tcp(s)) => if let Some(st) = &mut s.stream {
                            out = match st.write(&buf) { Ok(n) => n as u64, Err(e) => io_err(&e, 11) };
                        },
                        Some(Sock::Udp(slot)) => {
                            let dst = self.read_sockaddr(a[4]);
                            if let Some(u) = slot.as_ref() {
                                out = match dst {
                                    Some(d) => match u.send_to(&buf, d) { Ok(n) => n as u64, Err(e) => io_err(&e, 11) },
                                    None => match u.send(&buf) { Ok(n) => n as u64, Err(e) => io_err(&e, 11) },
                                };
                            }
                        }
                        Some(Sock::Other(_)) => out = buf.len() as u64,
                        None => out = errno(9),
                    }
                });
                Ok(out)
            }
            207 => {
                let len = a[2] as usize;
                let mut out = errno(11);
                let mut data: Vec<u8> = Vec::new();
                let mut from: Option<SocketAddrV4> = None;
                TABLE.with(|t| {
                    let mut t = t.borrow_mut();
                    match t.fds.get_mut(&a[0]) {
                        Some(Sock::Tcp(s)) => {
                            if let Some(st) = &mut s.stream {
                                let mut b = vec![0u8; len];
                                out = match st.read(&mut b) {
                                    Ok(n) => { b.truncate(n); data = b; n as u64 }
                                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => errno(11),
                                    Err(e) => io_err(&e, 11),
                                };
                            }
                        }
                        Some(Sock::Other(_)) => out = errno(11),
                        Some(Sock::Udp(slot)) => {
                            if let Some(u) = slot.as_ref() {
                                let mut b = vec![0u8; len];
                                if let Ok((n, p)) = u.recv_from(&mut b) {
                                    b.truncate(n); data = b; out = n as u64;
                                    if let SocketAddr::V4(v) = p { from = Some(v); }
                                }
                            }
                        }
                        None => out = errno(9),
                    }
                });
                if !data.is_empty() { self.write_bytes(a[1], &data)?; }
                if from.is_some() { self.write_sockaddr(a[4], a[5], from); }
                Ok(out)
            }
            208 | 209 | 210 => Ok(0),
            211 | 212 => Ok(errno(38)),
            _ => Ok(0),
        }
    }

    fn read_sockaddr(&mut self, ptr: u64) -> Option<SocketAddrV4> {
        if ptr == 0 { return None; }
        let b = self.read_bytes(ptr, 16).ok()?;
        if u16::from_le_bytes([b[0], b[1]]) != AF_INET { return None; }
        Some(SocketAddrV4::new(Ipv4Addr::new(b[4], b[5], b[6], b[7]), u16::from_be_bytes([b[2], b[3]])))
    }

    fn write_sockaddr(&mut self, ptr: u64, len_ptr: u64, addr: Option<SocketAddrV4>) {
        if ptr == 0 { return; }
        let mut b = [0u8; 16];
        let Some(a) = addr else { return };
        b[0..2].copy_from_slice(&AF_INET.to_le_bytes());
        b[2..4].copy_from_slice(&a.port().to_be_bytes());
        b[4..8].copy_from_slice(&a.ip().octets());
        let _ = self.write_bytes(ptr, &b);
        if len_ptr != 0 { let _ = self.write_bytes(len_ptr, &16u32.to_le_bytes()); }
    }
}
