use nix::ioctl_readwrite;
use std::fs::OpenOptions;
use std::os::fd::AsRawFd;

#[repr(C)]
#[derive(Debug)]
pub struct nvme_passthru_cmd {
    opcode: u8,
    flags: u8,
    rsvd1: u16,
    nsid: u32,
    cdw2: u32,
    cdw3: u32,
    metadata: u64,
    addr: u64,
    metadata_len: u32,
    data_len: u32,
    cdw10: u32,
    cdw11: u32,
    cdw12: u32,
    cdw13: u32,
    cdw14: u32,
    cdw15: u32,
    timeout_ms: u32,
    result: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct nvme_passthru_cmd_64 {
    opcode: u8,
    flags: u8,
    rsvd1: u16,
    nsid: u32,
    cdw2: u32,
    cdw3: u32,
    metadata: u64,
    addr: u64,
    metadata_len: u64,
    data_len_or_vec_cnt: u32,
    cdw10: u32,
    cdw11: u32,
    cdw12: u32,
    cdw13: u32,
    cdw14: u32,
    cdw15: u32,
    timeout_ms: u32,
    rsvd2: u32,
    result: u64,
}

ioctl_readwrite!(nvme_ioctl_admin_cmd, b'N', 0x41, nvme_passthru_cmd);
ioctl_readwrite!(nvme_ioctl_admin64_cmd, b'N', 0x47, nvme_passthru_cmd_64);

fn do_work() {
    let file = OpenOptions::new().read(true).open("/dev/nvme0n1").unwrap();
    println!("{:?}", file);

    let mut data: [u8; 4096] = [0; 4096];
    let mut cmd = nvme_passthru_cmd {
        opcode: 0x06,
        flags: 0,
        rsvd1: 0,
        nsid: 0,
        cdw2: 0,
        cdw3: 0,
        metadata: 0,
        addr: &mut data as *mut u8 as u64,
        metadata_len: 0,
        data_len: 4096,
        cdw10: 0x01,
        cdw11: 0,
        cdw12: 0,
        cdw13: 0,
        cdw14: 0,
        cdw15: 0,
        timeout_ms: 5000,
        result: 0,
    };

    println!("{:x}", cmd.addr);
    println!("{:?}", cmd);

    unsafe {
        let fd = file.as_raw_fd();
        nvme_ioctl_admin_cmd(fd, &mut cmd).unwrap();
    }

    println!("{:?}", data);
}
