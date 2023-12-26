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
  /// The CPU on which the minidump was generated
  pub cpu: String,
  /// The operating system that generated the minidump
  pub os: String,
}

#[napi(object)]
pub struct MiscInfo {
  // MISC_INFO_1
  pub size_of_info: Option<u32>,
  // pub flags1: u32,
  // pub process_id: u32,
  // pub process_create_time: u32,
  // pub process_user_time: u32,
  // pub process_kernel_time: u32,
  // // MISC_INFO_2
  // pub processor_max_mhz: u32,
  // pub processor_current_mhz: u32,
  // pub processor_mhz_limit: u32,
  // pub processor_max_idle_state: u32,
  // pub processor_current_idle_state: u32,
  // // MISC_INFO_3
  // pub process_integrity_level: u32,
  // pub process_execute_flags: u32,
  // pub protected_process: u32,
  // pub time_zone_id: u32,
  // // pub time_zone: minidump::format::TIME_ZONE_INFORMATION,
  // // MISC_INFO_4
  // // pub build_string: [u16; 260], // MAX_PATH
  // // pub dbg_bld_str: [u16; 40],
  // // MISC_INFO_5
  // // pub xstate_data: XSTATE_CONFIG_FEATURE_MSC_INFO,
  // pub process_cookie: u32,
}

#[napi]
pub struct Minidump {
  /// !IMPORTANT: should keep it `private` to prevent napi convertion
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

  #[napi]
  pub fn get_misc_info(&self)-> napi::Result<MiscInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpMiscInfo>();

    let misc_info = match result {
      Ok(info) => info,
      Err(_) => {
        return Err(napi::Error::new(
          napi::Status::GenericFailure,
          "get miscInfo stream from dump file failed".to_owned(),
        ))
      }
    };

    println!("misc info: {:?}", misc_info);

    let mut output = MiscInfo {
      size_of_info: None,
    };

    match &misc_info.raw {
      minidump::RawMiscInfo::MiscInfo(info) => {output.size_of_info = Some(info.size_of_info)},
      minidump::RawMiscInfo::MiscInfo2(info) => {output.size_of_info = Some(info.size_of_info)},
      minidump::RawMiscInfo::MiscInfo3(info) => {output.size_of_info = Some(info.size_of_info)},
      minidump::RawMiscInfo::MiscInfo4(info) => {output.size_of_info = Some(info.size_of_info)},
      minidump::RawMiscInfo::MiscInfo5(info) => {output.size_of_info = Some(info.size_of_info)},
    }

    Ok(output)
  }
}
