use super::super::error::{Error, ErrorCodes};
use nix::ioctl_readwrite;
use std::fs::{self, DirEntry, File, OpenOptions};
use std::io;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[repr(C)]
#[derive(Debug)]
pub struct NvmePassthruCmd {
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
pub struct NvmePassthruCmd64 {
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

ioctl_readwrite!(nvme_ioctl_admin_cmd, b'N', 0x41, NvmePassthruCmd);
ioctl_readwrite!(nvme_ioctl_admin64_cmd, b'N', 0x47, NvmePassthruCmd64);

// Scan should return recipes which get built by our factory as many times as needed
// because we don't want devices to remain open, so user can just hold onto
// these recipes and build them when they need them
pub fn scan() -> Vec<DeviceRecipe> {
    let mut res: Vec<DeviceRecipe> = Vec::new();

    if let Ok(entries) = fs::read_dir(Path::new("/sys/class/nvme")) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(devname) = path.file_name() {
                    let mut devpath = PathBuf::new();
                    devpath.push("/dev");
                    devpath.push(devname);
                    res.push(DeviceRecipe::new(devpath))
                }
            }
        }
    }

    println!("kitfoxi::linux::nvme::scan result {:?}", res);

    res
}

#[derive(Debug)]
pub struct DeviceRecipe {
    pub path: PathBuf,
}

impl DeviceRecipe {
    pub fn new(path: PathBuf) -> DeviceRecipe {
        DeviceRecipe { path: path }
    }

    pub fn make_device(&self) -> Device {
        Device::new(self)
    }
}

// A device will always keep the file for the device open.
// If you want to close it, simply let the device go out of scope.
#[derive(Debug)]
pub struct Device {
    pub f: std::fs::File,
}

impl Device {
    pub fn new(recipe: &DeviceRecipe) -> Device {
        Device {
            f: File::options()
                .read(true)
                .open(recipe.path.clone())
                .unwrap(),
        }
    }

    pub fn identify_controller(&self) -> Result<IdentifyControllerBuffer, Error> {
        let mut data: [u8; 4096] = [0; 4096];
        let mut cmd = NvmePassthruCmd {
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
        self.execute(&mut cmd)?;
        Ok(IdentifyControllerBuffer { data: data })
    }

    pub fn get_smart_log(&self) -> Result<SmartHealthInformationLogBuffer, Error> {
        let mut data: [u8; 4096] = [0; 4096];
        let mut cmd = NvmePassthruCmd {
            opcode: 0x02,
            flags: 0,
            rsvd1: 0,
            nsid: 0,
            cdw2: 0,
            cdw3: 0,
            metadata: 0,
            addr: &mut data as *mut u8 as u64,
            metadata_len: 0,
            data_len: 4096,
            cdw10: 0x2 | 1024u32 << 16,
            cdw11: 0,
            cdw12: 0,
            cdw13: 0,
            cdw14: 0,
            cdw15: 0,
            timeout_ms: 5000,
            result: 0,
        };
        self.execute(&mut cmd)?;
        Ok(SmartHealthInformationLogBuffer { data: data })
    }

    pub fn execute(&self, cmd: &mut NvmePassthruCmd) -> Result<i32, Error> {
        unsafe {
            let fd = self.f.as_raw_fd();
            match nvme_ioctl_admin_cmd(fd, cmd) {
                Ok(result) => Ok(result),
                Err(error) => match error {
                    nix::Error::InvalidPath => Err(Error {
                        code: ErrorCodes::IoctlErrorInvalidPath,
                        subcode: 0,
                        message: format!("Failed to execute IOCTL. Invalid path."),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                    }),
                    nix::Error::InvalidUtf8 => Err(Error {
                        code: ErrorCodes::IoctlErrorInvalidUtf8,
                        subcode: 0,
                        message: format!("Failed to execute IOCTL. Invalid UTF8."),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                    }),
                    nix::Error::UnsupportedOperation => Err(Error {
                        code: ErrorCodes::IoctlErrorUnsupportedOperation,
                        subcode: 0,
                        message: format!("Failed to execute IOCTL. Unsupported operation."),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                    }),
                    nix::Error::Sys(errno) => Err(Error {
                        code: ErrorCodes::IoctlError,
                        subcode: errno as i32,
                        message: format!("Failed to execute IOCTL. Errno {:?}.", errno),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                    }),
                },
            }
        }
    }

    pub fn execute64(&self, cmd: &mut NvmePassthruCmd64) -> Result<i32, Error> {
        unsafe {
            let fd = self.f.as_raw_fd();
            match nvme_ioctl_admin64_cmd(fd, cmd) {
                Ok(result) => Ok(result),
                Err(error) => match error {
                    nix::Error::InvalidPath => Err(Error {
                        code: ErrorCodes::IoctlErrorInvalidPath,
                        subcode: 0,
                        message: format!("Failed to execute IOCTL. Invalid path."),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                    }),
                    nix::Error::InvalidUtf8 => Err(Error {
                        code: ErrorCodes::IoctlErrorInvalidUtf8,
                        subcode: 0,
                        message: format!("Failed to execute IOCTL. Invalid UTF8."),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                    }),
                    nix::Error::UnsupportedOperation => Err(Error {
                        code: ErrorCodes::IoctlErrorUnsupportedOperation,
                        subcode: 0,
                        message: format!("Failed to execute IOCTL. Unsupported operation."),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                    }),
                    nix::Error::Sys(errno) => Err(Error {
                        code: ErrorCodes::IoctlError,
                        subcode: errno as i32,
                        message: format!("Failed to execute IOCTL. Errno {:?}.", errno),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                    }),
                },
            }
        }
    }
}

pub struct IdentifyControllerBuffer {
    data: [u8; 4096],
}

impl IdentifyControllerBuffer {
    fn parse_string(&self, start: usize, end: usize, trim: bool) -> Result<String, Error> {
        match String::from_utf8(self.data[start..end].to_vec()) {
            Ok(v) => {
                if trim {
                    Ok(String::from(v.trim()))
                } else {
                    Ok(v)
                }
            }
            Err(e) => Err(Error {
                code: ErrorCodes::ParsingError,
                subcode: (start as i32) << 16 | end as i32,
                message: e.to_string(),
                file: String::from(file!()),
                line: line!(),
                column: column!(),
            }),
        }
    }

    pub fn model(&self) -> Result<String, Error> {
        self.parse_string(24, 63, true)
    }
    pub fn serial(&self) -> Result<String, Error> {
        self.parse_string(4, 23, true)
    }
    pub fn firmware(&self) -> Result<String, Error> {
        self.parse_string(64, 71, true)
    }
}

pub struct SmartHealthInformationLogBuffer {
    data: [u8; 4096],
}

impl SmartHealthInformationLogBuffer {
    fn parse_u128(&self, start: usize, end: usize) -> Result<u128, Error> {
        match self.data[start..end].try_into() {
            Ok(v) => Ok(u128::from_le_bytes(v)), //TODO: LE or BE?
            Err(e) => Err(Error {
                code: ErrorCodes::ParsingError,
                subcode: (start as i32) << 16 | end as i32,
                message: e.to_string(),
                file: String::from(file!()),
                line: line!(),
                column: column!(),
            }),
        }
    }
    pub fn critical_warning(&self) -> Result<u8, Error> {
        Ok(self.data[0])
    }
    pub fn cw_spare_below_threshold(&self) -> Result<bool, Error> {
        Ok(self.critical_warning().unwrap() & 1 > 0)
    }
    pub fn cw_temperature(&self) -> Result<bool, Error> {
        Ok(self.critical_warning().unwrap() & 2 > 0)
    }
    pub fn cw_subsystem_degraded(&self) -> Result<bool, Error> {
        Ok(self.critical_warning().unwrap() & 4 > 0)
    }
    pub fn cw_read_only(&self) -> Result<bool, Error> {
        Ok(self.critical_warning().unwrap() & 8 > 0)
    }
    pub fn cw_backup(&self) -> Result<bool, Error> {
        Ok(self.critical_warning().unwrap() & 16 > 0)
    }
    pub fn cw_persistent_memory_region(&self) -> Result<bool, Error> {
        Ok(self.critical_warning().unwrap() & 32 > 0)
    }

    pub fn percentage_used(&self) -> Result<u8, Error> {
        Ok(self.data[5])
    }

    pub fn data_units_read(&self) -> Result<u128, Error> {
        self.parse_u128(32, 47)
    }

    pub fn data_units_written(&self) -> Result<u128, Error> {
        self.parse_u128(48, 63)
    }
}
