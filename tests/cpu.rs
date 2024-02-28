use std::path::PathBuf;

use cloud_hypervisor_command_builder::to_command::ToCommand;
use cloud_hypervisor_command_builder::{
    CloudHypervisorInstance, CpuAffinity, CpuFeatures, CpuTopology, Cpus, OnOff,
};

#[test]
fn cpu() {
    let mut ch = CloudHypervisorInstance::new(PathBuf::from("/cloud-hypervisor"));

    let mut cpus = Cpus::new();
    cpus.boot(2);
    cpus.max(4);
    cpus.topology(CpuTopology {
        threads_per_core: 1,
        cores_per_die: 2,
        dies_per_package: 3,
        packages: 4,
    });
    cpus.kvm_hyperv(OnOff::On);
    cpus.max_phys_bits(8);
    cpus.affinity(vec![
        CpuAffinity {
            vcpu: 0,
            host_cpus: vec![1, 2],
        },
        CpuAffinity {
            vcpu: 1,
            host_cpus: vec![3, 4, 5],
        },
    ]);
    cpus.features(vec![CpuFeatures::Amx]);

    ch.cpus(cpus);

    let expected = [
        "/cloud-hypervisor",
        "--cpus", "boot=2,max=4,topology=1:2:3:4,kvm_hyperv=on,max_phys_bits=8,affinity=[0@[1,2],1@[3,4,5]],features=amx"
    ];

    assert_eq!(ch.to_command(), expected);
    assert_eq!(ch.to_single_command(), expected.join(" "));
}
