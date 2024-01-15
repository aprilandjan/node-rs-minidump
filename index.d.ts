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
export interface MinidumpCrashpadInfo {
  moduleList: Array<MinidumpModuleCrashpadInfo>
}
/** Information about the system that generated the minidump. */
export interface MinidumpSystemInfo {
  /** The CPU on which the minidump was generated */
  cpu: string
  /** The operating system that generated the minidump */
  os: string
}
/**
 * Miscellaneous process information
 *
 * This struct matches the [Microsoft struct][msdn] of the same name.
 *
 * [msdn]: https://docs.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_misc_info
 */
export interface MinidumpMiscInfo {
  sizeOfInfo: number
  flags1: number
  processId: number
  processCreateTime: number
  processUserTime: number
  processKernelTime: number
  processorMaxMhz?: number
  processorCurrentMhz?: number
  processorMhzLimit?: number
  processorMaxIdleState?: number
  processorCurrentIdleState?: number
  processIntegrityLevel?: number
  processExecuteFlags?: number
  protectedProcess?: number
  timeZoneId?: number
  timeZone?: MinidumpMiscInfoTimeZone
  buildString?: Array<number>
  dbgBldStr?: Array<number>
  xstateData?: MinidumpMiscInfoXStateConfigFeature
  processCookie?: number
}
/**
 * Settings for a time zone
 *
 * This struct matches the [Microsoft struct][msdn] of the same name.
 *
 * [msdn]: https://docs.microsoft.com/en-us/windows/win32/api/timezoneapi/ns-timezoneapi-time_zone_information
 */
export interface MinidumpMiscInfoTimeZone {
  bias: number
  standardName: Array<number>
  standardDate: MinidumpMiscInfoSystemTime
  standardBias: number
  daylightName: Array<number>
  daylightDate: MinidumpMiscInfoSystemTime
  daylightBias: number
}
/**
 * A date and time
 *
 * This struct matches the [Microsoft struct][msdn] of the same name.
 *
 * [msdn]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms724950(v=vs.85).aspx
 */
export interface MinidumpMiscInfoSystemTime {
  year: number
  month: number
  dayOfWeek: number
  day: number
  hour: number
  minute: number
  second: number
  milliseconds: number
}
export interface MinidumpMiscInfoXStateFeature {
  offset: number
  size: number
}
export interface MinidumpMiscInfoXStateConfigFeature {
  sizeOfInfo: number
  contextSize: number
  enabledFeatures: bigint
  features: Array<MinidumpMiscInfoXStateFeature>
}
export class MinidumpException {
  context(): void
  getCrashAddress(): void
  getCrashReason(): void
  getCrashingThreadId(): void
}
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
  getCrashpadInfo(): MinidumpCrashpadInfo
  getSystemInfo(): MinidumpSystemInfo
  getMiscInfo(): MinidumpMiscInfo
  getException(): MinidumpException
}
