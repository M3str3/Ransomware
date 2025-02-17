extern crate winapi;

use std::ptr::null_mut;
use winapi::{
    shared::minwindef::HMODULE,
    um::{

        debugapi::IsDebuggerPresent,
        handleapi::CloseHandle,
        processthreadsapi::OpenProcess,
        psapi::{EnumProcessModules, EnumProcesses, GetModuleBaseNameW},
        winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
};


/// ===================================================================================
/// Anti-Reversing Techniques
/// ===================================================================================
/// This module performs basic anti-debugging and anti-reversing checks.
/// It scans for debuggers and known suspicious processes that indicate virtualization or monitoring tools.
pub fn anti_reversing() {
    #[cfg(debug_assertions)]
    {
        println!("==============================");
        println!("Starting anti-reversing checks");
        println!("==============================");
    }

    if !debugger_present() && !check_suspicious_processes() {
        #[cfg(debug_assertions)]
        {
            println!("Debugger or suspicious processes found");
        }
        std::process::exit(0);
    }

    #[cfg(debug_assertions)]
    {
        println!("==============================");
        println!("Finished anti-reversing checks");
        println!("==============================");
    }
}

/// Detects if a debugger is attached using the Windows API.
fn debugger_present() -> bool {
    unsafe { IsDebuggerPresent() != 0 }
}

/// Scans running processes and checks for known suspicious applications that might indicate a VM or reverse engineering tools.
fn check_suspicious_processes() -> bool {
    let mut processes = vec![0u32; 1024];
    let mut needed = 0u32;

    if unsafe {
        EnumProcesses(
            processes.as_mut_ptr(),
            (processes.len() * std::mem::size_of::<u32>()) as u32,
            &mut needed,
        )
    } == 0
    {
        return false;
    }

    let num_processes = needed as usize / std::mem::size_of::<u32>();
    processes[..num_processes].iter().any(|&pid| {
        if pid == 0 {
            return false;
        }
        let process_name = get_process_name(pid);
        process_name.is_some() && is_process_suspicious(&process_name.unwrap())
    })
}

/// Retrieves the name of a process by its process ID (PID).
fn get_process_name(pid: u32) -> Option<String> {
    unsafe {
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if process_handle.is_null() {
            return None;
        }

        let mut main_module = null_mut();
        let mut needed = 0;
        if EnumProcessModules(
            process_handle,
            &mut main_module,
            std::mem::size_of::<HMODULE>() as u32,
            &mut needed,
        ) == 0
        {
            CloseHandle(process_handle);
            return None;
        }

        let mut process_name = vec![0u16; needed as usize / 2];
        if GetModuleBaseNameW(
            process_handle,
            main_module,
            process_name.as_mut_ptr(),
            process_name.len() as u32,
        ) > 0
        {
            CloseHandle(process_handle);
            process_name.retain(|&c| c != 0);
            return Some(String::from_utf16(&process_name).unwrap());
        }

        CloseHandle(process_handle);
        None
    }
}

/// Checks if the process name matches a known suspicious application.
/// These processes are commonly associated with reverse engineering, VM detection, and monitoring tools.
fn is_process_suspicious(name: &str) -> bool {
    let suspicious_processes = [
        "vmsrvc", "tcpview", "wireshark", "fiddler", "vmware", "VirtualBox",
        "procexp", "autoit", "vboxtray", "vmtoolsd", "vmrawdsk", "vmusbmouse",
        "df5serv", "vboxservice",
    ];
    suspicious_processes
        .iter()
        .any(|&proc| name.to_lowercase().contains(proc))
}
