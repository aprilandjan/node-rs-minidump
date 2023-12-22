#![deny(clippy::all)]
use std::{collections::HashMap};
use napi_derive::napi;
use memmap2::Mmap;

/// Additional Crashpad-specific information about a module carried within a minidump file.
#[napi(object)]
pub struct MinidumpModuleCrashpadInfo {
  /// Index of the corresponding module in the `MinidumpModuleList`.
  pub module_index: u32,
  pub list_annotations: Vec<String>,
  pub simple_annotations: HashMap<String, String>,
  pub annotation_objects: HashMap<String, String>,
}

#[napi(object)]
pub struct CrashpadInfo {
  pub module_list: Vec<MinidumpModuleCrashpadInfo>,
}

/// Information about the system that generated the minidump.
#[napi(object)]
pub struct SystemInfo {
  /// An x86 (not x64!) CPU vendor name that is stored in `raw` but in a way
  pub cpu: String,
  pub os: String,

}

#[napi]
pub struct Minidump {
  /// !IMPORTANT: should keep it `private` to prevent napi conversion
  dump: minidump::Minidump<'static, Mmap>,
}

#[napi]
impl Minidump {
  /// custom constructor for napi
  /// see https://napi.rs/docs/concepts/class#custom-constructor
  #[napi(constructor)]
  pub fn new(path: String) -> Result::<Self, napi::Error> {
    let dump_result = minidump::Minidump::read_path(path);

    let dump = match dump_result {
      Ok(dump) => dump,
      Err(_) => {
        return Err(napi::Error::new(
          napi::Status::InvalidArg,
          "read minidump file failed".to_owned(),
        ))
      }
    };

    Ok(Minidump {
      dump,
    })
  }

  /// instance method for napi
  /// see https://napi.rs/docs/concepts/class#class-method
  #[napi]
  pub fn get_crashpad_info(&self)-> napi::Result<CrashpadInfo> {
    let crashpad_info_result = &self.dump.get_stream::<minidump::MinidumpCrashpadInfo>();

    let crashpad_info = match crashpad_info_result {
      Ok(info) => info,
      Err(_) => {
        return Err(napi::Error::new(
          napi::Status::GenericFailure,
          "get crashpadInfo stream from dump file failed".to_owned(),
        ))
      }
    };

    let mut module_list: Vec<MinidumpModuleCrashpadInfo> = vec![];

    // iterate the module list
    for m in crashpad_info.module_list.iter() {
      let module_index: u32 = m.module_index as u32;
      let list_annotations: Vec<String> = m.list_annotations.to_owned();
      let mut simple_annotations: HashMap<String, String> = HashMap::new();
      let mut annotation_objects: HashMap<String, String> = HashMap::new();

      for (k, v) in &m.simple_annotations {
        simple_annotations.insert(k.to_string(), v.to_string());
      }

      for (k, v) in &m.annotation_objects {
        match v {
          minidump::MinidumpAnnotation::String(string) => {
            annotation_objects.insert(k.to_string(), string.to_string());
          }
          minidump::MinidumpAnnotation::Invalid => {
            annotation_objects.insert(k.to_string(), "<invalid>".to_string());
          }
          minidump::MinidumpAnnotation::UserDefined(_) => {
            annotation_objects.insert(k.to_string(), "<user defined>".to_string());
          }
          minidump::MinidumpAnnotation::Unsupported(_) => {
            annotation_objects.insert(k.to_string(), "<unsupported>".to_string());
          }
          _ => (),
        };
      }

      let module = MinidumpModuleCrashpadInfo {
        module_index,
        list_annotations,
        simple_annotations,
        annotation_objects,
      };
      module_list.push(module);
    }

    Ok(CrashpadInfo { module_list })
  }

  #[napi]
  pub fn get_system_info(&self)-> napi::Result<SystemInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpSystemInfo>();

    let system_info = match result {
      Ok(info) => info,
      Err(_) => {
        return Err(napi::Error::new(
          napi::Status::GenericFailure,
          "get systemInfo stream from dump file failed".to_owned(),
        ))
      }
    };

    println!("system_info: {:?}", system_info);

    let os = match system_info.os {
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
    let cpu = match system_info.cpu {
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
    Ok(SystemInfo {
      os: os.to_owned(),
      cpu: cpu.to_owned(),
    })
  }
}
