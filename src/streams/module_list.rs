use napi_derive::napi;
use minidump;

#[napi(object)]
pub struct JsMinidumpModuleList {
  //
}

impl From<&minidump::MinidumpModuleList> for JsMinidumpModuleList {
  fn from(value: &minidump::MinidumpModuleList) -> Self {
    // TODO:
    JsMinidumpModuleList {
      //
    }
  }
}
