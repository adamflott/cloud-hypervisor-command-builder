use bytesize::ByteSize;
use std::path::PathBuf;

use cloud_hypervisor_command_builder::to_command::ToCommand;
use cloud_hypervisor_command_builder::{
    Balloon, CloudHypervisorInstance, Console, Cpus, DebugConsole, Device, Fs, Memory,
    MemoryHotplugMethod, OnOff, PathOrFileDescriptorOption, Platform, Rng, SecComp, Serial, SgxEpc,
    UserDevice, Vdpa, Vsock,
};

#[test]
fn full_command_line() {
    let mut ch = CloudHypervisorInstance::new(PathBuf::from("/cloud-hypervisor"));

    let cpus = Cpus {
        boot_vcpus: None,
        max_vcpus: None,
        topology: None,
        kvm_hyperv: None,
        max_phys_bits: None,
        affinity: None,
        features: None,
    };
    ch.cpus(cpus);

    let platform = Platform {
        num_pci_segments: Some(10),
        iommu_segments: Some(8),
        serial_number: Some("some_serial".to_string()),
        uuid: Some("uuid".to_string()),
        oem_strings: Some(vec!["oem_string".to_string()]),
    };
    ch.platform(platform);

    ch.memory(Memory {
        size: Some(ByteSize::gb(4)),
        mergeable: Some(OnOff::On),
        shared: Some(OnOff::Off),
        hugepages: Some(OnOff::On),
        hugepage_size: Some(ByteSize::mb(2)),
        hotplug_method: Some(MemoryHotplugMethod::VirtioMem),
        hotplug_size: Some(ByteSize::mb(64)),
        hotplugged_size: Some(ByteSize::mb(24)),
        prefault: Some(OnOff::Off),
        thp: Some(OnOff::On),
    });

    ch.firmware(PathBuf::from("/firmware"));
    ch.kernel(PathBuf::from("/kernel"));
    ch.initramfs(PathBuf::from("/initramfs"));
    ch.cmdline("--whatever".to_string());

    ch.rng(Rng::Src(PathBuf::from("/dev/urandom")));

    ch.balloon(Balloon {
        size: Some(ByteSize::mb(100)),
        deflate_on_oom: Some(OnOff::On),
        free_page_reporting: Some(OnOff::Off),
    });

    ch.fs(Fs {
        tag: None,
        socket: None,
        num_queues: None,
        queue_size: None,
        id: None,
        pci_segment: None,
    });
    ch.serial(Serial::Null);
    ch.console(Console::Pty);

    ch.device(Device {
        path: Some(PathBuf::from("/dev/something")),
        iommu: Some(OnOff::Off),
        id: Some("dev_id_1".to_string()),
        pci_segment: Some("pci1".to_string()),
    });

    ch.user_device(UserDevice {
        socket: None,
        id: None,
        pci_segment: None,
    });

    ch.pvpanic(true);

    ch.watchdog(true);

    ch.log_file(PathBuf::from("/ch.log"));
    ch.api_socket(PathOrFileDescriptorOption::Path(PathBuf::from(
        "/api.socket",
    )));
    ch.event_monitor(PathOrFileDescriptorOption::Fd(3));

    ch.seccomp(SecComp::Log);

    ch.tpm(PathBuf::from("/tpm"));

    ch.vsock(Vsock {
        cid: None,
        socket: Some(PathBuf::from("/dev/vsock")),
        iommu: None,
        id: None,
        pci_segment: None,
    });

    ch.vdpa(Vdpa {
        path: Some(PathBuf::from("/dev/vdpa1")),
        num_queues: None,
        iommu: None,
        id: None,
        pci_segment: None,
    });

    ch.sgx_epc(SgxEpc {
        id: None,
        size: None,
        prefault: None,
    });

    ch.debug_console(DebugConsole {
        console_type: None,
        iobase: None,
    });

    ch.v();
    ch.v();

    let expected = [
        "/cloud-hypervisor",
        "--platform", "num_pci_segments=10,iommu_segments=8,serial_number=some_serial,uuid=uuid,oem_strings=[oem_string]",
        "--memory", "size=4000000000,mergeable=on,shared=off,hugepages=on,hugepage_size=2000000,hotplug_method=virtio-mem,hotplug_size=64000000,hotplugged_size=24000000,prefault=off,thp=on",
        "--firmware", "/firmware",
        "--kernel", "/kernel",
        "--initramfs", "/initramfs",
        "--cmdline", "--whatever",
        "--rng", "src=/dev/urandom",
        "--balloon", "size=100000000,deflate_on_oom=on,free_page_reporting=off",
        "--serial", "null",
        "--console", "pty",
        "--device", "path=/dev/something,iommu=off,id=dev_id_1,pci_segment=pci1",
        "--vsock", "socket=/dev/vsock",
        "--vdpa", "path=/dev/vdpa1",
        "--pvpanic",
        "--watchdog",
        "--log-file", "/ch.log",
        "--api-socket", "path=/api.socket",
        "--event-monitor", "fd=3",
        "--seccomp", "log",
        "--tpm", "/tpm",
        "-v", "-v",
    ];

    assert_eq!(ch.to_command(), expected);
    assert_eq!(ch.to_single_command(), expected.join(" "));
}
