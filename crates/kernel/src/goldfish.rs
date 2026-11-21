//! /dev/goldfish_pipe (a.k.a. qemu_pipe): the Android emulator's host
//! services, which the ranchu HALs in the ROM talk to. The first write names
//! the service ("pipe:qemud:sensors\0"); afterwards reads and writes go to it.
//! qemud services frame messages with a 4-hex-digit length prefix.

use crate::errno::*;
use crate::sync::WaitQueue;
use crate::vfs::*;
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;

pub struct GoldfishPipe {
    service: Mutex<Option<String>>,
    pending: Mutex<Vec<u8>>, // bytes before the service name was complete
    rx: Mutex<VecDeque<u8>>,
    inbuf: Mutex<Vec<u8>>, // qemud frames being assembled
    wq: Arc<WaitQueue>,
    id: u64,
    pub state: Mutex<SensorState>,
}

#[derive(Default)]
pub struct SensorState {
    enabled: u32,
    delay_ms: u64,
    ticker: bool,
}

/// Boot properties the emulator would provide (qemud:boot-properties).
const BOOT_PROPS: &[(&str, &str)] = &[
    ("qemu.sf.lcd_density", "440"),
    ("qemu.hw.mainkeys", "0"),
    ("qemu.sf.fake_camera", "none"),
    ("dalvik.vm.heapsize", "512m"),
    ("qemu.timezone", "Asia/Seoul"),
    ("vendor.qemu.dev.bootcomplete", "1"),
    ("qemu.gles", "1"),
    ("qemu.opengles.version", "196609"),
];

impl GoldfishPipe {
    pub fn new() -> Arc<GoldfishPipe> {
        Arc::new(GoldfishPipe {
            service: Mutex::new(None),
            pending: Mutex::new(Vec::new()),
            rx: Mutex::new(VecDeque::new()),
            inbuf: Mutex::new(Vec::new()),
            wq: WaitQueue::new(),
            id: new_id(),
            state: Mutex::new(SensorState::default()),
        })
    }

    fn reply_frame(&self, msg: &[u8]) {
        let mut rx = self.rx.lock();
        rx.extend(format!("{:04x}", msg.len()).bytes());
        rx.extend(msg.iter().copied());
        drop(rx);
        self.wq.notify();
    }

    fn qemud_message(self: &Arc<Self>, service: &str, msg: &[u8]) {
        let m = String::from_utf8_lossy(msg).to_string();
        match service {
            "qemud:boot-properties" => {
                if m == "list" {
                    for (k, v) in BOOT_PROPS {
                        self.reply_frame(format!("{k}={v}").as_bytes());
                    }
                    self.reply_frame(b"");
                }
            }
            "qemud:sensors" => {
                if m == "list-sensors" {
                    // Bit 0 = accelerometer.
                    self.reply_frame(b"1");
                } else if let Some(rest) = m.strip_prefix("set-delay:") {
                    self.state.lock().delay_ms = rest.trim().parse().unwrap_or(200);
                } else if let Some(rest) = m.strip_prefix("set:") {
                    let mut it = rest.split(':');
                    let name = it.next().unwrap_or("");
                    let on = it.next() == Some("1");
                    if name == "acceleration" {
                        let mut st = self.state.lock();
                        if on {
                            st.enabled |= 1;
                        } else {
                            st.enabled &= !1;
                        }
                        let start = on && !st.ticker;
                        if start {
                            st.ticker = true;
                        }
                        drop(st);
                        if start {
                            self.start_ticker();
                        }
                    }
                } else if m == "time" {
                    let t = crate::time::monotonic().as_nanos();
                    self.reply_frame(format!("{t}").as_bytes());
                }
            }
            "qemud:hw-control" | "qemud:gsm" | "qemud:gps" | "qemud:fingerprintlisten" => {}
            _ => {}
        }
    }

    /// Sends gravity (device upright) at the requested rate.
    fn start_ticker(self: &Arc<Self>) {
        let w = Arc::downgrade(self);
        std::thread::Builder::new()
            .name("qemud-sensors".into())
            .spawn(move || loop {
                let Some(p) = w.upgrade() else { break };
                let (on, delay) = {
                    let s = p.state.lock();
                    (s.enabled & 1 != 0, s.delay_ms.clamp(10, 1000))
                };
                if !on {
                    p.state.lock().ticker = false;
                    break;
                }
                p.reply_frame(b"acceleration:0.000000:9.810000:0.000000");
                let t = crate::time::monotonic().as_micros();
                p.reply_frame(format!("sync:{t}").as_bytes());
                drop(p);
                std::thread::sleep(std::time::Duration::from_millis(delay));
            })
            .ok();
    }
}

impl Node for GoldfishPipe {
    fn id(&self) -> u64 {
        self.id
    }
    fn stat(&self) -> Stat {
        Stat {
            mode: S_IFCHR | 0o666,
            ino: self.id,
            rdev: 0x0a3e,
            nlink: 1,
            ..Default::default()
        }
    }
    fn is_stream(&self) -> bool {
        true
    }
    fn write_at(&self, _o: u64, buf: &[u8], _nb: bool) -> Result<usize> {
        // Needs Arc<Self> for services; reconstruct via a weak self pointer
        // stored in the device table is overkill: handle synchronously.
        let mut svc = self.service.lock();
        if svc.is_none() {
            let mut p = self.pending.lock();
            p.extend_from_slice(buf);
            if let Some(z) = p.iter().position(|&b| b == 0) {
                let name = String::from_utf8_lossy(&p[..z]).to_string();
                let name = name.strip_prefix("pipe:").unwrap_or(&name).to_string();
                let rest = p[z + 1..].to_vec();
                p.clear();
                *svc = Some(name);
                drop(svc);
                drop(p);
                if !rest.is_empty() {
                    self.feed(&rest);
                }
            }
            return Ok(buf.len());
        }
        drop(svc);
        self.feed(buf);
        Ok(buf.len())
    }
    fn read_at(&self, _o: u64, buf: &mut [u8], nonblock: bool) -> Result<usize> {
        loop {
            let seen = self.wq.epoch();
            {
                let mut rx = self.rx.lock();
                if !rx.is_empty() {
                    let n = buf.len().min(rx.len());
                    for (i, b) in rx.drain(..n).enumerate() {
                        buf[i] = b;
                    }
                    return Ok(n);
                }
            }
            if nonblock {
                return Err(EAGAIN);
            }
            if crate::task::current_signal_pending() {
                return Err(EINTR);
            }
            self.wq.wait_change(seen, None);
        }
    }
    fn poll(&self) -> u32 {
        if self.rx.lock().is_empty() {
            POLLOUT
        } else {
            POLLIN | POLLOUT
        }
    }
    fn wait_queue(&self) -> Option<Arc<WaitQueue>> {
        Some(self.wq.clone())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl GoldfishPipe {
    fn feed(&self, data: &[u8]) {
        let svc = self.service.lock().clone().unwrap_or_default();
        if !svc.starts_with("qemud:") {
            return; // non-qemud services (opengles, GLProcessPipe...): sink
        }
        let mut inb = self.inbuf.lock();
        inb.extend_from_slice(data);
        let mut msgs = Vec::new();
        while inb.len() >= 4 {
            let Ok(len) =
                usize::from_str_radix(std::str::from_utf8(&inb[..4]).unwrap_or("zzzz"), 16)
            else {
                // Unframed client write: treat the whole buffer as one message.
                msgs.push(std::mem::take(&mut *inb));
                break;
            };
            if inb.len() < 4 + len {
                break;
            }
            msgs.push(inb[4..4 + len].to_vec());
            inb.drain(..4 + len);
        }
        drop(inb);
        // SAFETY of the Arc requirement: services that spawn threads need an
        // Arc; GOLDFISH keeps the Arc for each open pipe.
        if let Some(me) = REGISTRY
            .lock()
            .iter()
            .filter_map(|w| w.upgrade())
            .find(|p| p.id == self.id)
        {
            for m in msgs {
                me.qemud_message(&svc, &m);
            }
        }
    }
}

static REGISTRY: std::sync::LazyLock<Mutex<Vec<std::sync::Weak<GoldfishPipe>>>> =
    std::sync::LazyLock::new(|| Mutex::new(Vec::new()));

/// Device factory for /dev/goldfish_pipe.
pub fn open() -> FileRef {
    let p = GoldfishPipe::new();
    let mut r = REGISTRY.lock();
    r.retain(|w| w.strong_count() > 0);
    r.push(Arc::downgrade(&p));
    p
}
