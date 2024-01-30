use napi_derive::napi;
use std::collections::HashMap;
use minidump;

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
pub struct JsMinidumpCrashpadInfo {
  pub module_list: Vec<MinidumpModuleCrashpadInfo>,
}

impl From<&minidump::MinidumpCrashpadInfo> for JsMinidumpCrashpadInfo {
  fn from(crashpad_info: &minidump::MinidumpCrashpadInfo) -> Self {

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

    JsMinidumpCrashpadInfo { module_list }
  }
}
