use napi_derive::napi;
use minidump;

#[napi(object)]
pub struct JsMinidumpModuleList {
  pub modules: Vec<JsMinidumpModule>
}

#[napi(object)]
pub struct JsMinidumpModule {
  /// The original module text
  pub raw: String,
}

impl From<&minidump::MinidumpModule> for JsMinidumpModule {
  fn from(value: &minidump::MinidumpModule) -> Self {
    // just print, and parse the string lines...
    let mut bytes = Vec::new();
    value.print(&mut bytes).unwrap();
    let raw = String::from_utf8(bytes).unwrap();

    JsMinidumpModule {
      // TODO:
      raw,
    }
  }
}

impl From<&minidump::MinidumpModuleList> for JsMinidumpModuleList {
  fn from(value: &minidump::MinidumpModuleList) -> Self {
    // TODO:
    let modules = value.iter().map(|m| {JsMinidumpModule::from(m)}).collect();
    JsMinidumpModuleList {
      modules,
    }
  }
}
