/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

/** Additional Crashpad-specific information about a module carried within a minidump file. */
export interface MinidumpModuleCrashpadInfo {
  /** Index of the corresponding module in the `MinidumpModuleList`. */
  moduleIndex: number
  listAnnotations: Array<string>
  simpleAnnotations: Record<string, string>
  annotationObjects: Record<string, string>
}
export interface CrashpadInfo {
  moduleList: Array<MinidumpModuleCrashpadInfo>
}
/** Information about the system that generated the minidump. */
export interface SystemInfo {}
export class Minidump {
  /**
   * custom constructor for napi
   * see https://napi.rs/docs/concepts/class#custom-constructor
   */
  constructor(path: string)
  /**
   * instance method for napi
   * see https://napi.rs/docs/concepts/class#class-method
   */
  getCrashpadInfo(): CrashpadInfo
  getSystemInfo(): SystemInfo
}
