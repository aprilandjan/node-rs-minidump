#![deny(clippy::all)]
use std::collections::HashMap;

use napi::{Error, Status};
use napi_derive::napi;

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

/// Get crashpad-specific information carried within a minidump file.
/// For example: process type, process pid, etc.
#[napi]
pub fn get_crashpad_info(path: String) -> napi::Result<CrashpadInfo> {
  // Read the minidump from a file
  let dump_result = minidump::Minidump::read_path(path);

  let dump = match dump_result {
    Ok(dump) => dump,
    Err(_) => {
      return Err(Error::new(
        Status::InvalidArg,
        "read minidump file failed".to_owned(),
      ))
    }
  };

  let crashpad_info_result = dump.get_stream::<minidump::MinidumpCrashpadInfo>();

  let crashpad_info = match crashpad_info_result {
    Ok(info) => info,
    Err(_) => {
      return Err(Error::new(
        Status::GenericFailure,
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
