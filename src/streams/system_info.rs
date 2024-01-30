use napi_derive::napi;
use minidump;

/// Information about the system that generated the minidump.
#[napi(object)]
pub struct JsMinidumpSystemInfo {
  /// The CPU on which the minidump was generated
  pub cpu: String,
  /// The operating system that generated the minidump
  pub os: String,
}

impl From<&minidump::MinidumpSystemInfo> for JsMinidumpSystemInfo {
  fn from(value: &minidump::MinidumpSystemInfo) -> Self {
    let os = match value.os {
      minidump::system_info::Os::Windows => "windows",
      minidump::system_info::Os::MacOs => "macOs",
      minidump::system_info::Os::Ios => "iOS",
      minidump::system_info::Os::Linux => "linux",
      minidump::system_info::Os::Solaris => "solaris",
      minidump::system_info::Os::Android => "android",
      minidump::system_info::Os::Ps3 => "ps3",
      minidump::system_info::Os::NaCl => "naCl",
      _ => "unknown",
    };
    let cpu = match value.cpu {
      minidump::system_info::Cpu::X86 => "x86",
      minidump::system_info::Cpu::X86_64 => "x86_64",
      minidump::system_info::Cpu::Ppc => "ppc",
      minidump::system_info::Cpu::Ppc64 => "ppc64",
      minidump::system_info::Cpu::Sparc => "sparc",
      minidump::system_info::Cpu::Arm => "arm",
      minidump::system_info::Cpu::Arm64 => "arm64",
      minidump::system_info::Cpu::Mips => "mips",
      minidump::system_info::Cpu::Mips64 => "mips64",
      _ => "unknown",
    };
    JsMinidumpSystemInfo {
      os: os.to_owned(),
      cpu: cpu.to_owned(),
    }
  }
}
