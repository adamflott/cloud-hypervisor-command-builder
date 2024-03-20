use bytesize::ByteSize;
use std::path::PathBuf;

use cloud_hypervisor_command_builder::to_command::ToCommand;
use cloud_hypervisor_command_builder::{
    BalloonBuilder, CloudHypervisorInstance, Console, CpusBuilder, DebugConsole, DeviceBuilder,
    DiskBuilder, FsBuilder, MemoryBuilder, MemoryHotplugMethod, OnOff, PathOrFileDescriptorOption,
    PlatformBuilder, Rng, SecComp, Serial, SgxEpc, UserDeviceBuilder, VdpaBuilder, Vsock,
};

#[test]
fn full_command_line() {
    let mut ch = CloudHypervisorInstance::new(PathBuf::from("/cloud-hypervisor"));

    let cpus = CpusBuilder::default().build().unwrap();
    ch.cpus(cpus);

    let platform = PlatformBuilder::default()
        .num_pci_segments(10)
        .iommu_segments(8)
        .serial_number("some_serial")
        .uuid("uuid")
        .oem_strings(vec!["oem_string".to_string()])
        .build()
        .unwrap();
    ch.platform(platform);

    ch.memory(
        MemoryBuilder::default()
            .size(ByteSize::gb(4))
            .mergeable(OnOff::On)
            .shared(OnOff::Off)
            .hugepages(OnOff::On)
            .hugepage_size(ByteSize::mb(2))
            .hotplug_method(MemoryHotplugMethod::VirtioMem)
            .hotplug_size(ByteSize::mb(64))
            .hotplugged_size(ByteSize::mb(24))
            .prefault(OnOff::Off)
            .thp(OnOff::On)
            .build()
            .unwrap(),
    );

    ch.firmware(PathBuf::from("/firmware"));
    ch.kernel(PathBuf::from("/kernel"));
    ch.initramfs(PathBuf::from("/initramfs"));
    ch.cmdline("--whatever".to_string());

    let disk0 = DiskBuilder::default()
        .path(PathBuf::from("/dev/disk0"))
        .build()
        .unwrap();
    ch.disk(disk0);
    let disk1 = DiskBuilder::default()
        .path(PathBuf::from("/dev/disk1"))
        .id("1")
        .build()
        .unwrap();
    ch.disk(disk1);

    ch.rng(Rng::Src(PathBuf::from("/dev/urandom")));

    ch.balloon(
        BalloonBuilder::default()
            .size(ByteSize::mb(100))
            .deflate_on_oom(OnOff::On)
            .free_page_reporting(OnOff::Off)
            .build()
            .unwrap(),
    );

    ch.fs(FsBuilder::default().build().unwrap());
    ch.serial(Serial::Null);
    ch.console(Console::Pty);

    ch.device(
        DeviceBuilder::default()
            .path(PathBuf::from("/dev/something"))
            .iommu(OnOff::Off)
            .id("dev_id_1".to_string())
            .pci_segment("pci1".to_string())
            .build()
            .unwrap(),
    );

    ch.user_device(UserDeviceBuilder::default().build().unwrap());

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

    ch.vdpa(
        VdpaBuilder::default()
            .path(PathBuf::from("/dev/vdpa1"))
            .num_queues(1usize)
            .iommu(OnOff::On)
            .id("id1")
            .pci_segment("01")
            .build()
            .unwrap(),
    );

    ch.vdpa(
        VdpaBuilder::default()
            .path(PathBuf::from("/dev/vdpa2"))
            .build()
            .unwrap(),
    );

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
        "--disk", "path=/dev/disk0", "path=/dev/disk1,id=1",
        "--rng", "src=/dev/urandom",
        "--balloon", "size=100000000,deflate_on_oom=on,free_page_reporting=off",
        "--serial", "null",
        "--console", "pty",
        "--device", "path=/dev/something,iommu=off,id=dev_id_1,pci_segment=pci1",
        "--vsock", "socket=/dev/vsock",
        "--vdpa", "path=/dev/vdpa1,num_queues=1,iommu=on,id=id1,pci_segment=01", "path=/dev/vdpa2",
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
