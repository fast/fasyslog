// Copyright 2024 FastLabs Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::ffi::OsString;

/// Get the standard host name for the current machine.
pub(crate) fn hostname() -> Option<OsString> {
    #[cfg(unix)]
    {
        nix::unistd::gethostname().ok()
    }

    #[cfg(windows)]
    {
        // This code snippet is derived from https://github.com/swsnr/gethostname.rs/blob/main/src/lib.rs.

        use std::os::windows::ffi::OsStringExt;

        // The DNS host name of the local computer. If the local computer is a node
        // in a cluster, lpBuffer receives the DNS host name of the local computer,
        // not the name of the cluster virtual server.
        pub const COMPUTER_NAME_PHYSICAL_DNS_HOSTNAME: i32 = 5;

        // https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getcomputernameexw
        ::windows_targets::link!("kernel32.dll" "system" fn GetComputerNameExW(nametype: i32, lpbuffer: *mut u16, nsize: *mut u32) -> i32);

        let mut buffer_size: u32 = 0;

        unsafe {
            // This call always fails with ERROR_MORE_DATA, because we pass NULL to
            // get the required buffer size.  GetComputerNameExW then fills buffer_size with the
            // size of the host name string plus a trailing zero byte.
            GetComputerNameExW(
                COMPUTER_NAME_PHYSICAL_DNS_HOSTNAME,
                std::ptr::null_mut(),
                &mut buffer_size,
            )
        };
        assert!(
            0 < buffer_size,
            "GetComputerNameExW did not provide buffer size"
        );

        let mut buffer = vec![0_u16; buffer_size as usize];
        unsafe {
            if GetComputerNameExW(
                COMPUTER_NAME_PHYSICAL_DNS_HOSTNAME,
                buffer.as_mut_ptr(),
                &mut buffer_size,
            ) == 0
            {
                panic!(
                    "GetComputerNameExW failed to read hostname.
        Please report this issue to <https://github.com/swsnr/gethostname.rs/issues>!"
                );
            }
        }
        assert!(
            // GetComputerNameExW returns the size _without_ the trailing zero byte on the second
            // call
            buffer_size as usize == buffer.len() - 1,
            "GetComputerNameExW changed the buffer size unexpectedly"
        );

        let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
        Some(OsString::from_wide(&buffer[0..end]))
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_hostname_matches_system_hostname() {
        let output = Command::new("hostname")
            .output()
            .expect("failed to get hostname");
        if output.status.success() {
            let system_hostname = String::from_utf8_lossy(&output.stdout);
            let system_hostname = system_hostname.trim_end().to_lowercase();
            assert!(!system_hostname.is_empty());

            let hostname = super::hostname().unwrap();
            let hostname = hostname.into_string().unwrap().to_lowercase();
            println!("system_hostname={system_hostname}, hostname={hostname}");
            assert_eq!(system_hostname, hostname);
        } else {
            panic!(
                "failed to get hostname: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
