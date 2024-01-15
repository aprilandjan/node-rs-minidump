use napi_derive::napi;
use minidump;

#[napi(object)]
pub struct MinidumpException {

}

impl From<&minidump::MinidumpException<'_>> for MinidumpException {
  fn from(value: &minidump::MinidumpException) -> Self {
    MinidumpException {
      // TODO:
    }
  }
}
