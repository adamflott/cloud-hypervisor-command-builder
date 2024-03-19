pub mod to_command;

use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::path::PathBuf;

use bytesize::ByteSize;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::to_command::ToCommand;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnOff {
    On,
    Off,
}

impl Display for OnOff {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OnOff::On => {
                write!(f, "on")
            }
            OnOff::Off => {
                write!(f, "off")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PathOrFileDescriptorOption {
    Path(PathBuf),
    Fd(usize),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct CloudHypervisorInstance {
    bin_path: PathBuf,
    cpus: Option<Cpus>,
    platform: Option<Platform>,
    memory: Option<Memory>,
    memory_zone: Option<MemoryZone>,
    firmware: Option<PathBuf>,
    kernel: Option<PathBuf>,
    initramfs: Option<PathBuf>,
    cmdline: Option<String>,
    rate_limit_group: Option<Vec<RateLimitGroup>>,
    disk: Option<Vec<Disk>>,
    net: Option<Vec<Net>>,
    rng: Option<Rng>,
    balloon: Option<Balloon>,
    fs: Option<Vec<Fs>>,
    pmem: Option<Vec<Pmem>>,
    serial: Option<Serial>,
    console: Option<Console>,
    device: Option<Vec<Device>>,
    user_device: Option<Vec<UserDevice>>,
    vdpa: Option<Vec<Vdpa>>,
    vsock: Option<Vsock>,
    pvpanic: Option<bool>,
    numa: Option<Vec<Numa>>,
    watchdog: Option<bool>,
    log_file: Option<PathBuf>,
    api_socket: Option<PathOrFileDescriptorOption>,
    event_monitor: Option<PathOrFileDescriptorOption>,
    restore: Option<Restore>,
    seccomp: Option<SecComp>,
    tpm: Option<PathBuf>,
    sgx_epc: Option<Vec<SgxEpc>>,
    debug_console: Option<DebugConsole>,
    v: Option<u8>,
}

impl CloudHypervisorInstance {
    pub fn new(bin_path: PathBuf) -> Self {
        CloudHypervisorInstance {
            bin_path,
            ..Self::default()
        }
    }

    pub fn cpus(&mut self, cpus: Cpus) -> &mut Self {
        self.cpus = Some(cpus);
        self
    }
    pub fn platform(&mut self, platform: Platform) -> &mut Self {
        self.platform = Some(platform);
        self
    }
    pub fn memory(&mut self, memory: Memory) -> &mut Self {
        self.memory = Some(memory);
        self
    }
    pub fn memory_zone(&mut self, memory_zone: MemoryZone) -> &mut Self {
        self.memory_zone = Some(memory_zone);
        self
    }
    pub fn firmware(&mut self, firmware: PathBuf) -> &mut Self {
        self.firmware = Some(firmware);
        self
    }
    pub fn kernel(&mut self, kernel: PathBuf) -> &mut Self {
        self.kernel = Some(kernel);
        self
    }
    pub fn initramfs(&mut self, initramfs: PathBuf) -> &mut Self {
        self.initramfs = Some(initramfs);
        self
    }
    pub fn cmdline(&mut self, cmdline: String) -> &mut Self {
        self.cmdline = Some(cmdline);
        self
    }
    pub fn rate_limit_group(&mut self, rate_limit_group: RateLimitGroup) -> &mut Self {
        match &mut self.rate_limit_group {
            None => {
                self.rate_limit_group = Some(vec![rate_limit_group]);
            }
            Some(rate_limit_groups) => {
                rate_limit_groups.push(rate_limit_group);
            }
        }
        self
    }
    pub fn disk(&mut self, disk: Disk) -> &mut Self {
        match &mut self.disk {
            None => {
                self.disk = Some(vec![disk]);
            }
            Some(disks) => {
                disks.push(disk);
            }
        }
        self
    }
    pub fn net(&mut self, net: Net) -> &mut Self {
        match &mut self.net {
            None => {
                self.net = Some(vec![net]);
            }
            Some(nets) => {
                nets.push(net);
            }
        }
        self
    }
    pub fn rng(&mut self, rng: Rng) -> &mut Self {
        self.rng = Some(rng);
        self
    }
    pub fn balloon(&mut self, balloon: Balloon) -> &mut Self {
        self.balloon = Some(balloon);
        self
    }
    pub fn fs(&mut self, fs: Fs) -> &mut Self {
        match &mut self.fs {
            None => {
                self.fs = Some(vec![fs]);
            }
            Some(fss) => {
                fss.push(fs);
            }
        }
        self
    }
    pub fn pmem(&mut self, pmem: Pmem) -> &mut Self {
        match &mut self.pmem {
            None => {
                self.pmem = Some(vec![pmem]);
            }
            Some(pmems) => {
                pmems.push(pmem);
            }
        }
        self
    }
    pub fn serial(&mut self, serial: Serial) -> &mut Self {
        self.serial = Some(serial);
        self
    }
    pub fn console(&mut self, console: Console) -> &mut Self {
        self.console = Some(console);
        self
    }
    pub fn device(&mut self, device: Device) -> &mut Self {
        match &mut self.device {
            None => {
                self.device = Some(vec![device]);
            }
            Some(devices) => {
                devices.push(device);
            }
        }

        self
    }
    pub fn user_device(&mut self, user_device: UserDevice) -> &mut Self {
        match &mut self.user_device {
            None => {
                self.user_device = Some(vec![user_device]);
            }
            Some(user_devices) => {
                user_devices.push(user_device);
            }
        }
        self
    }
    pub fn vdpa(&mut self, vdpa: Vdpa) -> &mut Self {
        match &mut self.vdpa {
            None => {
                self.vdpa = Some(vec![vdpa]);
            }
            Some(vdpas) => {
                vdpas.push(vdpa);
            }
        }
        self
    }
    pub fn vsock(&mut self, vsock: Vsock) -> &mut Self {
        self.vsock = Some(vsock);
        self
    }
    pub fn pvpanic(&mut self, pvpanic: bool) -> &mut Self {
        self.pvpanic = Some(pvpanic);
        self
    }
    pub fn numa(&mut self, numa: Numa) -> &mut Self {
        match &mut self.numa {
            None => {
                self.numa = Some(vec![numa]);
            }
            Some(numas) => {
                numas.push(numa);
            }
        }
        self
    }
    pub fn watchdog(&mut self, watchdog: bool) -> &mut Self {
        self.watchdog = Some(watchdog);
        self
    }
    pub fn log_file(&mut self, log_file: PathBuf) -> &mut Self {
        self.log_file = Some(log_file);
        self
    }
    pub fn api_socket(&mut self, api_socket: PathOrFileDescriptorOption) -> &mut Self {
        self.api_socket = Some(api_socket);
        self
    }
    pub fn event_monitor(&mut self, event_monitor: PathOrFileDescriptorOption) -> &mut Self {
        self.event_monitor = Some(event_monitor);
        self
    }
    pub fn restore(&mut self, restore: Restore) -> &mut Self {
        self.restore = Some(restore);
        self
    }
    pub fn seccomp(&mut self, seccomp: SecComp) -> &mut Self {
        self.seccomp = Some(seccomp);
        self
    }
    pub fn tpm(&mut self, tpm: PathBuf) -> &mut Self {
        self.tpm = Some(tpm);
        self
    }
    pub fn sgx_epc(&mut self, sgx_epc: SgxEpc) -> &mut Self {
        match &mut self.sgx_epc {
            None => {
                self.sgx_epc = Some(vec![sgx_epc]);
            }
            Some(sgx_epcs) => {
                sgx_epcs.push(sgx_epc);
            }
        }

        self
    }
    pub fn debug_console(&mut self, debug_console: DebugConsole) -> &mut Self {
        self.debug_console = Some(debug_console);
        self
    }
    pub fn v(&mut self) -> &mut Self {
        match &mut self.v {
            None => self.v = Some(1),
            Some(v) => {
                *v += 1;
            }
        }
        self
    }
}

impl ToCommand for CloudHypervisorInstance {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![self.bin_path.display().to_string()];

        if let Some(cpus) = &self.cpus {
            cmd.append(cpus.to_command().as_mut());
        }
        if let Some(platform) = &self.platform {
            cmd.append(platform.to_command().as_mut());
        }
        if let Some(memory) = &self.memory {
            cmd.append(memory.to_command().as_mut());
        }
        if let Some(memory_zone) = &self.memory_zone {
            cmd.append(memory_zone.to_command().as_mut());
        }
        if let Some(firmware) = &self.firmware {
            cmd.push("--firmware".to_string());
            cmd.push(firmware.display().to_string());
        }
        if let Some(kernel) = &self.kernel {
            cmd.push("--kernel".to_string());
            cmd.push(kernel.display().to_string());
        }
        if let Some(initramfs) = &self.initramfs {
            cmd.push("--initramfs".to_string());
            cmd.push(initramfs.display().to_string());
        }
        if let Some(cmdline) = &self.cmdline {
            cmd.push("--cmdline".to_string());
            cmd.push(cmdline.to_string());
        }
        if let Some(rate_limit_groups) = &self.rate_limit_group {
            let mut arg = vec![];
            for rate_limit_group in rate_limit_groups {
                if let Some(bw_size) = &rate_limit_group.bw_size {
                    arg.push(format!("bw_size={}", bw_size.0));
                }
                if let Some(bw_one_time_burst) = &rate_limit_group.bw_one_time_burst {
                    arg.push(format!("bw_one_time_burst={}", bw_one_time_burst));
                }
                if let Some(bw_refill_time) = &rate_limit_group.bw_refill_time {
                    arg.push(format!("bw_refill_time={}", bw_refill_time));
                }
                if let Some(ops_size) = &rate_limit_group.ops_size {
                    arg.push(format!("ops_size={}", ops_size));
                }
                if let Some(ops_one_time_burst) = &rate_limit_group.ops_one_time_burst {
                    arg.push(format!("ops_one_time_burst={}", ops_one_time_burst));
                }
                if let Some(ops_refill_time) = &rate_limit_group.ops_refill_time {
                    arg.push(format!("ops_refill_time={}", ops_refill_time));
                }
                if let Some(id) = &rate_limit_group.id {
                    arg.push(format!("id={}", id));
                }
                if !arg.is_empty() {
                    cmd.push("--rate_limit_group".to_string());
                    cmd.push(arg.join(","));
                }
            }
        }
        if let Some(disks) = &self.disk {
            let mut arg = vec![];
            for disk in disks {
                if let Some(path) = &disk.path {
                    arg.push(format!("path={}", path.display()));
                }
                if let Some(readonly) = &disk.readonly {
                    arg.push(format!("readonly={}", readonly));
                }
                if let Some(direct) = &disk.direct {
                    arg.push(format!("direct={}", direct));
                }
                if let Some(iommu) = &disk.iommu {
                    arg.push(format!("iommu={}", iommu));
                }
                if let Some(num_queues) = &disk.num_queues {
                    arg.push(format!("num_queues={}", num_queues));
                }
                if let Some(queue_size) = &disk.queue_size {
                    arg.push(format!("queue_size={}", queue_size));
                }
                if let Some(vhost_user) = &disk.vhost_user {
                    arg.push(format!("vhost_user={}", vhost_user));
                }
                if let Some(socket) = &disk.socket {
                    arg.push(format!("socket={}", socket.display()));
                }
                if let Some(bw_size) = &disk.bw_size {
                    arg.push(format!("bw_size={}", bw_size.0));
                }
                if let Some(bw_one_time_burst) = &disk.bw_one_time_burst {
                    arg.push(format!("bw_one_time_burst={}", bw_one_time_burst));
                }
                if let Some(bw_refill_time) = &disk.bw_refill_time {
                    arg.push(format!("bw_refill_time={}", bw_refill_time));
                }
                if let Some(ops_size) = &disk.ops_size {
                    arg.push(format!("ops_size={}", ops_size));
                }
                if let Some(ops_one_time_burst) = &disk.ops_one_time_burst {
                    arg.push(format!("ops_one_time_burst={}", ops_one_time_burst));
                }
                if let Some(ops_refill_time) = &disk.ops_refill_time {
                    arg.push(format!("ops_refill_time={}", ops_refill_time));
                }
                if let Some(id) = &disk.id {
                    arg.push(format!("id={}", id));
                }
                if let Some(pci_segment) = &disk.pci_segment {
                    arg.push(format!("pci_segment={}", pci_segment));
                }
                if let Some(rate_limit_group) = &disk.rate_limit_group {
                    arg.push(format!("rate_limit_group={}", rate_limit_group));
                }
                if let Some(queue_affinity) = &disk.queue_affinity {
                    arg.push(format!("queue_affinity={}", queue_affinity));
                }
                if !arg.is_empty() {
                    cmd.push("--disk".to_string());
                    cmd.push(arg.join(","));
                }
            }
        }
        if let Some(nets) = &self.net {
            let mut arg = vec![];
            for net in nets {
                if let Some(tap) = &net.tap {
                    arg.push(format!("tap={}", tap));
                }
                if let Some(ip) = &net.ip {
                    arg.push(format!("ip={}", ip));
                }
                if let Some(mask) = &net.mask {
                    arg.push(format!("mask={}", mask));
                }
                if let Some(mac) = &net.mac {
                    arg.push(format!("mac={}", mac));
                }
                if let Some(fd) = &net.fd {
                    arg.push(format!(
                        "fd={}",
                        fd.iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    ));
                }
                if let Some(iommu) = &net.iommu {
                    arg.push(format!("iommu={}", iommu));
                }
                if let Some(num_queues) = &net.num_queues {
                    arg.push(format!("num_queues={}", num_queues));
                }
                if let Some(queue_size) = &net.queue_size {
                    arg.push(format!("queue_size={}", queue_size));
                }
                if let Some(id) = &net.id {
                    arg.push(format!("id={}", id));
                }
                if let Some(vhost_user) = &net.vhost_user {
                    arg.push(format!("vhost_user={}", vhost_user));
                }
                if let Some(socket) = &net.socket {
                    arg.push(format!("socket={}", socket.display()));
                }
                if let Some(vhost_mode) = &net.vhost_mode {
                    arg.push(format!("vhost_mode={}", vhost_mode));
                }
                if let Some(bw_size) = &net.bw_size {
                    arg.push(format!("bw_size={}", bw_size.0));
                }
                if let Some(bw_one_time_burst) = &net.bw_one_time_burst {
                    arg.push(format!("bw_one_time_burst={}", bw_one_time_burst));
                }
                if let Some(bw_refill_time) = &net.bw_refill_time {
                    arg.push(format!("bw_refill_time={}", bw_refill_time));
                }
                if let Some(ops_size) = &net.ops_size {
                    arg.push(format!("ops_size={}", ops_size));
                }
                if let Some(ops_one_time_burst) = &net.ops_one_time_burst {
                    arg.push(format!("ops_one_time_burst={}", ops_one_time_burst));
                }
                if let Some(ops_refill_time) = &net.ops_refill_time {
                    arg.push(format!("ops_refill_time={}", ops_refill_time));
                }
                if let Some(pci_segment) = &net.pci_segment {
                    arg.push(format!("pci_segment={}", pci_segment));
                }
                if let Some(offload_tso) = &net.offload_tso {
                    arg.push(format!("offload_tso={}", offload_tso));
                }
                if let Some(offload_ufo) = &net.offload_ufo {
                    arg.push(format!("offload_ufo={}", offload_ufo));
                }
                if let Some(offload_csum) = &net.offload_csum {
                    arg.push(format!("offload_csum={}", offload_csum));
                }
                if !arg.is_empty() {
                    cmd.push("--net".to_string());
                    cmd.push(arg.join(","));
                }
            }
        }
        if let Some(rng) = &self.rng {
            cmd.push("--rng".to_string());
            match rng {
                Rng::Src(path) => {
                    cmd.push(format!("src={}", path.display()));
                }
                Rng::Iommu(state) => {
                    cmd.push(format!("iommu={}", state));
                }
            }
        }
        if let Some(balloon) = &self.balloon {
            let mut arg = vec![];
            if let Some(size) = &balloon.size {
                arg.push(format!("size={}", size.0));
            }
            if let Some(deflate_on_oom) = &balloon.deflate_on_oom {
                arg.push(format!("deflate_on_oom={}", deflate_on_oom));
            }
            if let Some(free_page_reporting) = &balloon.free_page_reporting {
                arg.push(format!("free_page_reporting={}", free_page_reporting));
            }
            if !arg.is_empty() {
                cmd.push("--balloon".to_string());
                cmd.push(arg.join(","));
            }
        }
        if let Some(fss) = &self.fs {
            let mut arg = vec![];
            for fs in fss {
                if let Some(tag) = &fs.tag {
                    arg.push(format!("tag={}", tag));
                }
                if let Some(socket) = &fs.socket {
                    arg.push(format!("socket={}", socket.display()));
                }
                if let Some(num_queues) = &fs.num_queues {
                    arg.push(format!("num_queues={}", num_queues));
                }
                if let Some(queue_size) = &fs.queue_size {
                    arg.push(format!("queue_size={}", queue_size));
                }
                if let Some(id) = &fs.id {
                    arg.push(format!("id={}", id));
                }
                if let Some(pci_segment) = &fs.pci_segment {
                    arg.push(format!("pci_segment={}", pci_segment));
                }
                if !arg.is_empty() {
                    cmd.push("--fs".to_string());
                    cmd.push(arg.join(","));
                }
            }
        }
        if let Some(pmems) = &self.pmem {
            let mut arg = vec![];
            for pmem in pmems {
                if let Some(path) = &pmem.file {
                    arg.push(format!("file={}", path.display()));
                }
                if let Some(size) = &pmem.size {
                    arg.push(format!("size={}", size));
                }
                if let Some(iommu) = &pmem.iommu {
                    arg.push(format!("iommu={}", iommu));
                }
                if let Some(discard_writes) = &pmem.discard_writes {
                    arg.push(format!("discard_writes={}", discard_writes));
                }
                if let Some(id) = &pmem.id {
                    arg.push(format!("id={}", id));
                }
                if let Some(pci_segment) = &pmem.pci_segment {
                    arg.push(format!("pci_segment={}", pci_segment));
                }
                if !arg.is_empty() {
                    cmd.push("--pmem".to_string());
                    cmd.push(arg.join(","));
                }
            }
        }
        if let Some(serial) = &self.serial {
            cmd.push("--serial".to_string());
            match serial {
                Serial::Off => {
                    cmd.push("off".to_string());
                }
                Serial::Null => {
                    cmd.push("null".to_string());
                }
                Serial::Pty => {
                    cmd.push("pty".to_string());
                }
                Serial::Tty => {
                    cmd.push("tty".to_string());
                }
                Serial::File(path) => cmd.push(format!("file={}", path.display())),
                Serial::Socket(path) => cmd.push(format!("socket={}", path.display())),
            }
        }
        if let Some(console) = &self.console {
            cmd.push("--console".to_string());
            match console {
                Console::Off => {
                    cmd.push("off".to_string());
                }
                Console::Null => {
                    cmd.push("null".to_string());
                }
                Console::Pty => {
                    cmd.push("pty".to_string());
                }
                Console::Tty => {
                    cmd.push("tty".to_string());
                }
                Console::File(path) => cmd.push(format!("file={}", path.display())),
                Console::Iommu(state) => {
                    cmd.push(format!("iommu={}", state));
                }
            }
        }
        if let Some(devices) = &self.device {
            for device in devices {
                let mut arg = vec![];
                if let Some(path) = &device.path {
                    arg.push(format!("path={}", path.display()));
                }
                if let Some(iommu) = &device.iommu {
                    arg.push(format!("iommu={}", iommu));
                }
                if let Some(id) = &device.id {
                    arg.push(format!("id={}", id));
                }
                if let Some(pci_segment) = &device.pci_segment {
                    arg.push(format!("pci_segment={}", pci_segment));
                }
                if !arg.is_empty() {
                    cmd.push("--device".to_string());
                    cmd.push(arg.join(","));
                }
            }
        }
        if let Some(user_devices) = &self.user_device {
            for user_device in user_devices {
                let mut arg = vec![];
                if let Some(path) = &user_device.socket {
                    arg.push(format!("socket={}", path.display()));
                }
                if let Some(id) = &user_device.id {
                    arg.push(format!("id={}", id));
                }
                if let Some(pci_segment) = &user_device.pci_segment {
                    arg.push(format!("pci_segment={}", pci_segment));
                }
                if !arg.is_empty() {
                    cmd.push("--user-device".to_string());
                    cmd.push(arg.join(","));
                }
            }
        }
        if let Some(vsock) = &self.vsock {
            let mut arg = vec![];
            if let Some(cid) = &vsock.cid {
                arg.push(format!("cid={}", cid));
            }
            if let Some(path) = &vsock.socket {
                arg.push(format!("socket={}", path.display()));
            }
            if let Some(iommu) = &vsock.iommu {
                arg.push(format!("iommu={}", iommu));
            }
            if let Some(id) = &vsock.id {
                arg.push(format!("id={}", id));
            }
            if let Some(pci_segment) = &vsock.pci_segment {
                arg.push(format!("pci_segment={}", pci_segment));
            }
            if !arg.is_empty() {
                cmd.push("--vsock".to_string());
                cmd.push(arg.join(","));
            }
        }
        if let Some(vdpas) = &self.vdpa {
            for vdpa in vdpas {
                let mut arg = vec![];
                if let Some(path) = &vdpa.path {
                    arg.push(format!("path={}", path.display()));
                }
                if let Some(num_queues) = &vdpa.num_queues {
                    arg.push(format!("num_queues={}", num_queues));
                }
                if let Some(iommu) = &vdpa.iommu {
                    arg.push(format!("iommu={}", iommu));
                }
                if let Some(id) = &vdpa.id {
                    arg.push(format!("id={}", id));
                }
                if let Some(pci_segment) = &vdpa.pci_segment {
                    arg.push(format!("pci_segment={}", pci_segment));
                }
                if !arg.is_empty() {
                    cmd.push("--vdpa".to_string());
                    cmd.push(arg.join(","));
                }
            }
        }
        if let Some(pvpanic) = self.pvpanic {
            if pvpanic {
                cmd.push("--pvpanic".to_string());
            }
        }
        if let Some(numas) = &self.numa {
            for numa in numas {
                let mut arg = vec![];
                if let Some(guest_numa_id) = &numa.guest_numa_id {
                    arg.push(format!("guest_numa_id={}", guest_numa_id));
                }
                if let Some(cpus) = &numa.cpus {
                    arg.push(format!(
                        "cpus={}",
                        cpus.iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    ));
                }
                if let Some(distances) = &numa.distances {
                    arg.push(format!(
                        "distances={}",
                        distances
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    ));
                }
                if let Some(memory_zones) = &numa.memory_zones {
                    arg.push(format!(
                        "memory_zones={}",
                        memory_zones
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    ));
                }
                if let Some(sgx_epc_sections) = &numa.sgx_epc_sections {
                    arg.push(format!(
                        "sgx_epc_sections={}",
                        sgx_epc_sections
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    ));
                }
                if let Some(pci_segments) = &numa.pci_segments {
                    arg.push(format!(
                        "pci_segments={}",
                        pci_segments
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    ));
                }
                if !arg.is_empty() {
                    cmd.push("--numa".to_string());
                    cmd.push(arg.join(","));
                }
            }
        }
        if let Some(watchdog) = self.watchdog {
            if watchdog {
                cmd.push("--watchdog".to_string());
            }
        }
        if let Some(log_file) = &self.log_file {
            cmd.push("--log-file".to_string());
            cmd.push(log_file.display().to_string());
        }
        if let Some(api_socket) = &self.api_socket {
            cmd.push("--api-socket".to_string());
            match api_socket {
                PathOrFileDescriptorOption::Path(path) => {
                    cmd.push(format!("path={}", path.display()));
                }
                PathOrFileDescriptorOption::Fd(fd) => {
                    cmd.push(format!("fd={}", fd));
                }
            }
        }
        if let Some(event_monitor) = &self.event_monitor {
            cmd.push("--event-monitor".to_string());
            match event_monitor {
                PathOrFileDescriptorOption::Path(path) => {
                    cmd.push(format!("path={}", path.display()));
                }
                PathOrFileDescriptorOption::Fd(fd) => {
                    cmd.push(format!("fd={}", fd));
                }
            }
        }
        if let Some(restore) = &self.restore {
            let mut arg = vec![];
            if let Some(source_url) = &restore.source_url {
                arg.push(format!("source_url={}", source_url));
            }
            if let Some(prefault) = &restore.prefault {
                arg.push(format!("prefault={}", prefault));
            }
            if !arg.is_empty() {
                cmd.push("--restore".to_string());
                cmd.push(arg.join(","));
            }
        }
        if let Some(seccomp) = &self.seccomp {
            cmd.push("--seccomp".to_string());
            match seccomp {
                SecComp::True => {
                    cmd.push("true".to_string());
                }
                SecComp::False => {
                    cmd.push("false".to_string());
                }
                SecComp::Log => {
                    cmd.push("log".to_string());
                }
            }
        }
        if let Some(tpm) = &self.tpm {
            cmd.push("--tpm".to_string());
            cmd.push(tpm.display().to_string());
        }
        if let Some(sgx_epcs) = &self.sgx_epc {
            for sgx_epc in sgx_epcs {
                let mut arg = vec![];
                if let Some(id) = &sgx_epc.id {
                    arg.push(format!("id={}", id));
                }
                if let Some(size) = &sgx_epc.size {
                    arg.push(format!("size={}", size));
                }
                if let Some(prefault) = &sgx_epc.prefault {
                    arg.push(format!("prefault={}", prefault));
                }
                if !arg.is_empty() {
                    cmd.push("--sgx-epc".to_string());
                    cmd.push(arg.join(","));
                }
            }
        }
        if let Some(debug_console) = &self.debug_console {
            let mut arg = vec![];
            if let Some(ctype) = &debug_console.console_type {
                match ctype {
                    DebugConsoleType::Off => {
                        arg.push("off".to_string());
                    }
                    DebugConsoleType::Pty => arg.push("pty".to_string()),
                    DebugConsoleType::Tty => arg.push("tty".to_string()),
                    DebugConsoleType::File(path) => {
                        arg.push(format!("file={}", path.display()));
                    }
                }
            }
            if let Some(iobase) = &debug_console.iobase {
                arg.push(format!("iobase={}", iobase));
            }
            if !arg.is_empty() {
                cmd.push("--debug-console".to_string());
                cmd.push(arg.join(","));
            }
        }
        if let Some(v) = self.v {
            for _ in 0..v {
                cmd.push("-v".to_string());
            }
        }
        cmd
    }
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option), default)]
pub struct CpuAffinity {
    pub vcpu: u8,
    pub host_cpus: Vec<usize>,
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option), default)]
pub struct CpuFeatures {
    pub amx: Option<bool>,
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option), default)]
pub struct CpuTopology {
    pub threads_per_core: u8,
    pub cores_per_die: u8,
    pub dies_per_package: u8,
    pub packages: u8,
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option), default)]
pub struct Cpus {
    pub boot: Option<u8>,
    pub max: Option<u8>,
    pub topology: Option<CpuTopology>,
    pub kvm_hyperv: Option<OnOff>,
    pub max_phys_bits: Option<u8>,
    pub affinity: Option<Vec<CpuAffinity>>,
    pub features: Option<CpuFeatures>,
}

impl ToCommand for Cpus {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        let mut arg: Vec<String> = vec![];
        if let Some(boot) = self.boot {
            arg.push(format!("boot={}", boot));
        }
        if let Some(max) = self.max {
            arg.push(format!("max={}", max));
        }
        if let Some(topology) = &self.topology {
            arg.push(format!(
                "topology={}:{}:{}:{}",
                topology.threads_per_core,
                topology.cores_per_die,
                topology.dies_per_package,
                topology.packages
            ));
        }
        if let Some(kvm_hyperv) = &self.kvm_hyperv {
            arg.push(format!("kvm_hyperv={}", kvm_hyperv));
        }
        if let Some(max_phys_bits) = self.max_phys_bits {
            arg.push(format!("max_phys_bits={}", max_phys_bits));
        }
        if let Some(affinity) = &self.affinity {
            if !affinity.is_empty() {
                let mut aarg: Vec<String> = vec![];
                for vcpu in affinity {
                    aarg.push(format!(
                        "{}@[{}]",
                        vcpu.vcpu,
                        vcpu.host_cpus
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    ));
                }
                arg.push(format!("affinity=[{}]", aarg.join(",")));
            }
        }
        if let Some(features) = &self.features {
            let mut farg = vec![];

            if let Some(amx) = features.amx {
                if amx {
                    farg.push("amx");
                }
            }
            if !farg.is_empty() {
                arg.push(format!("features={}", farg.join(",")));
            }
        }
        if !arg.is_empty() {
            cmd.push("--cpus".to_string());
            cmd.push(arg.join(","));
        }

        cmd
    }
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct Platform {
    pub num_pci_segments: Option<u8>,
    pub iommu_segments: Option<u8>,
    pub serial_number: Option<String>,
    pub uuid: Option<String>,
    pub oem_strings: Option<Vec<String>>,
}
impl ToCommand for Platform {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        let mut arg: Vec<String> = vec![];
        if let Some(num_pci_segments) = &self.num_pci_segments {
            arg.push(format!("num_pci_segments={}", num_pci_segments));
        }

        if let Some(iommu_segments) = &self.iommu_segments {
            arg.push(format!("iommu_segments={}", iommu_segments));
        }
        if let Some(serial_number) = &self.serial_number {
            arg.push(format!("serial_number={}", serial_number));
        }
        if let Some(uuid) = &self.uuid {
            arg.push(format!("uuid={}", uuid));
        }
        if let Some(oem_strings) = &self.oem_strings {
            if !oem_strings.is_empty() {
                arg.push(format!("oem_strings=[{}]", oem_strings.join(",")));
            }
        }

        if !arg.is_empty() {
            cmd.push("--platform".to_string());
            cmd.push(arg.join(","));
        }

        cmd
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum MemoryHotplugMethod {
    Acpi,
    VirtioMem,
}

impl Display for MemoryHotplugMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryHotplugMethod::Acpi => {
                write!(f, "acpi")
            }
            MemoryHotplugMethod::VirtioMem => {
                write!(f, "virtio-mem")
            }
        }
    }
}
#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct Memory {
    pub size: Option<ByteSize>,
    pub mergeable: Option<OnOff>,
    pub shared: Option<OnOff>,
    pub hugepages: Option<OnOff>,
    pub hugepage_size: Option<ByteSize>,
    pub hotplug_method: Option<MemoryHotplugMethod>,
    pub hotplug_size: Option<ByteSize>,
    pub hotplugged_size: Option<ByteSize>,
    pub prefault: Option<OnOff>,
    pub thp: Option<OnOff>,
}

impl ToCommand for Memory {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        let mut arg: Vec<String> = vec![];

        if let Some(size) = &self.size {
            arg.push(format!("size={}", size.0));
        }

        if let Some(mergeable) = &self.mergeable {
            arg.push(format!("mergeable={}", mergeable));
        }

        if let Some(shared) = &self.shared {
            arg.push(format!("shared={}", shared));
        }

        if let Some(hugepages) = &self.hugepages {
            arg.push(format!("hugepages={}", hugepages));
        }

        if let Some(hugepage_size) = &self.hugepage_size {
            arg.push(format!("hugepage_size={}", hugepage_size.0));
        }

        if let Some(hotplug_method) = &self.hotplug_method {
            arg.push(format!("hotplug_method={}", hotplug_method));
        }

        if let Some(hotplug_size) = &self.hotplug_size {
            arg.push(format!("hotplug_size={}", hotplug_size.0));
        }

        if let Some(hotplugged_size) = &self.hotplugged_size {
            arg.push(format!("hotplugged_size={}", hotplugged_size.0));
        }

        if let Some(prefault) = &self.prefault {
            arg.push(format!("prefault={}", prefault));
        }

        if let Some(thp) = &self.thp {
            arg.push(format!("thp={}", thp));
        }

        if !arg.is_empty() {
            cmd.push("--memory".to_string());
            cmd.push(arg.join(","));
        }

        cmd
    }
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct MemoryZone {
    pub size: Option<ByteSize>,
    pub file: Option<PathBuf>,
    pub shared: Option<OnOff>,
    pub hugepages: Option<OnOff>,
    pub hugepage_size: Option<ByteSize>,
    pub host_numa_node: Option<usize>,
    pub id: Option<String>,
    pub hotplug_size: Option<ByteSize>,
    pub hotplugged_size: Option<ByteSize>,
    pub prefault: Option<OnOff>,
}

impl ToCommand for MemoryZone {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        let mut arg: Vec<String> = vec![];

        if let Some(size) = &self.size {
            arg.push(format!("size={}", size.0));
        }

        if let Some(path) = &self.file {
            arg.push(format!("file={}", path.display()));
        }

        if let Some(shared) = &self.shared {
            arg.push(format!("shared={}", shared));
        }

        if let Some(hugepages) = &self.hugepages {
            arg.push(format!("hugepages={}", hugepages));
        }

        if let Some(hugepage_size) = &self.hugepage_size {
            arg.push(format!("hugepage_size={}", hugepage_size.0));
        }

        if let Some(host_numa_node) = &self.host_numa_node {
            arg.push(format!("host_numa_node={}", host_numa_node));
        }

        if let Some(id) = &self.id {
            arg.push(format!("id={}", id));
        }

        if let Some(hotplug_size) = &self.hotplug_size {
            arg.push(format!("hotplug_size={}", hotplug_size.0));
        }

        if let Some(hotplugged_size) = &self.hotplugged_size {
            arg.push(format!("hotplugged_size={}", hotplugged_size.0));
        }

        if let Some(prefault) = &self.prefault {
            arg.push(format!("prefault={}", prefault));
        }

        if !arg.is_empty() {
            cmd.push("--memory-zone".to_string());
            cmd.push(arg.join(","));
        }

        cmd
    }
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct RateLimitGroup {
    pub bw_size: Option<ByteSize>,
    pub bw_one_time_burst: Option<ByteSize>,
    pub bw_refill_time: Option<usize>,
    pub ops_size: Option<usize>,
    pub ops_one_time_burst: Option<usize>,
    pub ops_refill_time: Option<usize>,
    pub id: Option<String>,
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct Disk {
    pub path: Option<PathBuf>,
    pub readonly: Option<OnOff>,
    pub direct: Option<OnOff>,
    pub iommu: Option<OnOff>,
    pub num_queues: Option<usize>,
    pub queue_size: Option<usize>,
    pub vhost_user: Option<OnOff>,
    pub socket: Option<PathBuf>,
    pub bw_size: Option<ByteSize>,
    pub bw_one_time_burst: Option<ByteSize>,
    pub bw_refill_time: Option<usize>,
    pub ops_size: Option<usize>,
    pub ops_one_time_burst: Option<usize>,
    pub ops_refill_time: Option<usize>,
    pub id: Option<String>,
    pub pci_segment: Option<String>,
    pub rate_limit_group: Option<String>,
    pub queue_affinity: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum VhostMode {
    Client,
    Server,
}

impl Display for VhostMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VhostMode::Client => {
                write!(f, "client")
            }
            VhostMode::Server => {
                write!(f, "server")
            }
        }
    }
}
#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct Net {
    pub tap: Option<String>,
    pub ip: Option<IpAddr>,
    pub mask: Option<u8>,
    pub mac: Option<String>,
    pub fd: Option<Vec<usize>>,
    pub iommu: Option<OnOff>,
    pub num_queues: Option<usize>,
    pub queue_size: Option<usize>,
    pub id: Option<String>,
    pub vhost_user: Option<OnOff>,
    pub socket: Option<PathBuf>,
    pub vhost_mode: Option<VhostMode>,
    pub bw_size: Option<ByteSize>,
    pub bw_one_time_burst: Option<ByteSize>,
    pub bw_refill_time: Option<usize>,
    pub ops_size: Option<usize>,
    pub ops_one_time_burst: Option<usize>,
    pub ops_refill_time: Option<usize>,
    pub pci_segment: Option<String>,
    pub offload_tso: Option<OnOff>,
    pub offload_ufo: Option<OnOff>,
    pub offload_csum: Option<OnOff>,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Rng {
    Src(PathBuf),
    Iommu(OnOff),
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct Balloon {
    pub size: Option<ByteSize>,
    pub deflate_on_oom: Option<OnOff>,
    pub free_page_reporting: Option<OnOff>,
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct Fs {
    pub tag: Option<String>,
    pub socket: Option<PathBuf>,
    pub num_queues: Option<usize>,
    pub queue_size: Option<usize>,
    pub id: Option<String>,
    pub pci_segment: Option<String>,
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct Pmem {
    pub file: Option<PathBuf>,
    pub size: Option<usize>,
    pub iommu: Option<OnOff>,
    pub discard_writes: Option<OnOff>,
    pub id: Option<String>,
    pub pci_segment: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Serial {
    Off,
    Null,
    Pty,
    Tty,
    File(PathBuf),
    Socket(PathBuf),
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Console {
    Off,
    Null,
    Pty,
    Tty,
    File(PathBuf),
    Iommu(OnOff),
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct Device {
    pub path: Option<PathBuf>,
    pub iommu: Option<OnOff>,
    pub id: Option<String>,
    pub pci_segment: Option<String>,
}
#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct UserDevice {
    pub socket: Option<PathBuf>,
    pub id: Option<String>,
    pub pci_segment: Option<String>,
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct Vdpa {
    pub path: Option<PathBuf>,
    pub num_queues: Option<usize>,
    pub iommu: Option<OnOff>,
    pub id: Option<String>,
    pub pci_segment: Option<String>,
}
#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct Vsock {
    pub cid: Option<String>,
    pub socket: Option<PathBuf>,
    pub iommu: Option<OnOff>,
    pub id: Option<String>,
    pub pci_segment: Option<String>,
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct Numa {
    pub guest_numa_id: Option<String>,
    pub cpus: Option<Vec<usize>>,
    pub distances: Option<Vec<usize>>,
    pub memory_zones: Option<Vec<String>>,
    pub sgx_epc_sections: Option<Vec<String>>,
    pub pci_segments: Option<Vec<String>>,
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct Restore {
    pub source_url: Option<String>,
    pub prefault: Option<OnOff>,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum SecComp {
    True,
    False,
    Log,
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct SgxEpc {
    pub id: Option<String>,
    pub size: Option<String>,
    pub prefault: Option<OnOff>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum DebugConsoleType {
    Off,
    Pty,
    Tty,
    File(PathBuf),
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[builder(setter(strip_option, into), default)]
pub struct DebugConsole {
    pub console_type: Option<DebugConsoleType>,
    pub iobase: Option<String>,
}
