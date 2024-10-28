/// An implementation which can obtain information about the available system memory.
///
/// This is used by [MmapGraph][crate::MmapGraph] to unmap subgraphs which have not been
/// used recently.
pub trait MemoryInfo {
    fn available_virtual_memory(&self) -> Result<u64, String>;
    fn total_virtual_memory(&self) -> Result<u64, String>;
}

#[cfg(unix)]
use libc::{sysconf, _SC_AVPHYS_PAGES, _SC_PAGESIZE, _SC_PHYS_PAGES};

#[cfg(unix)]
pub struct UnixMemoryInfo;

#[cfg(unix)]
impl MemoryInfo for UnixMemoryInfo {
    fn available_virtual_memory(&self) -> Result<u64, String> {
        unsafe {
            let pages = sysconf(_SC_AVPHYS_PAGES);
            let page_size = sysconf(_SC_PAGESIZE);

            if pages == -1 || page_size == -1 {
                Err("Failed to get available virtual memory".into())
            } else {
                Ok((pages as u64) * (page_size as u64))
            }
        }
    }

    fn total_virtual_memory(&self) -> Result<u64, String> {
        unsafe {
            let total_pages = sysconf(_SC_PHYS_PAGES);
            let page_size = sysconf(_SC_PAGESIZE);

            if total_pages == -1 || page_size == -1 {
                Err("Failed to get total virtual memory".into())
            } else {
                Ok((total_pages as u64) * (page_size as u64))
            }
        }
    }
}

#[cfg(windows)]
use std::mem;
#[cfg(windows)]
use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

#[cfg(windows)]
pub struct WindowsMemoryInfo;

#[cfg(windows)]
impl MemoryInfo for WindowsMemoryInfo {
    fn available_virtual_memory(&self) -> Result<u64, String> {
        unsafe {
            let mut memory_status = mem::zeroed::<MEMORYSTATUSEX>();
            memory_status.dwLength = mem::size_of::<MEMORYSTATUSEX>() as u32;
            if GlobalMemoryStatusEx(&mut memory_status) != 0 {
                Ok(memory_status.ullAvailVirtual)
            } else {
                Err("Failed to get available virtual memory.".into())
            }
        }
    }

    fn total_virtual_memory(&self) -> Result<u64, String> {
        unsafe {
            let mut memory_status = mem::zeroed::<MEMORYSTATUSEX>();
            memory_status.dwLength = mem::size_of::<MEMORYSTATUSEX>() as u32;
            if GlobalMemoryStatusEx(&mut memory_status) != 0 {
                Ok(memory_status.ullTotalVirtual)
            } else {
                Err("Failed to get total virtual memory.".into())
            }
        }
    }
}

pub fn get_memory_info() -> Box<dyn MemoryInfo> {
    #[cfg(unix)]
    {
        Box::new(UnixMemoryInfo)
    }

    #[cfg(windows)]
    {
        Box::new(WindowsMemoryInfo)
    }
}
