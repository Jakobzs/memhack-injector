fn main() {
    windows::build!(
        Windows::Win32::Foundation::CloseHandle,
        Windows::Win32::System::Threading::{OpenProcess, CreateRemoteThread},
        Windows::Win32::System::Memory::VirtualAllocEx,
        Windows::Win32::System::Diagnostics::Debug::WriteProcessMemory,
        Windows::Win32::System::LibraryLoader::{GetProcAddress, GetModuleHandleA}
    );
}
