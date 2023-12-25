import path from 'path'

import test from 'ava'

import { Minidump } from '../index'

function resolveDumpFile(name: string) {
  return path.join(__dirname, '../fixtures/', name)
}

test('should throw error if target file not exist', (t) => {
  const file = resolveDumpFile('not-existed-file.dmp')

  const error = t.throws(() => {
    new Minidump(file)
  })

  t.is(error?.message, 'read minidump file failed')
})

test('should get process type & pid from mac electron main process dump file correctly', (t) => {
  const file = resolveDumpFile('mac-electron-browser.dmp')

  const result = new Minidump(file).getCrashpadInfo()
  t.is(result.moduleList[0].annotationObjects['process_type'], 'browser')
  t.is(result.moduleList[0].annotationObjects['pid'], '11423')
})

test('should get process type & pid from electron renderer process dump file correctly', (t) => {
  const file = resolveDumpFile('mac-electron-renderer.dmp')

  const result = new Minidump(file).getCrashpadInfo()
  t.is(result.moduleList[0].annotationObjects['process_type'], 'renderer')
  t.is(result.moduleList[0].annotationObjects['pid'], '11795')
})

test('should get process type & pid from electron gpu process dump file correctly', (t) => {
  const file = resolveDumpFile('mac-electron-gpu.dmp')

  const result = new Minidump(file).getCrashpadInfo()
  t.is(result.moduleList[0].annotationObjects['process_type'], 'gpu-process')
  t.is(result.moduleList[0].annotationObjects['pid'], '11793')
})

test('should get process type & pid from electron node child process dump file correctly', (t) => {
  const file = resolveDumpFile('mac-electron-node.dmp')

  const result = new Minidump(file).getCrashpadInfo()
  t.is(result.moduleList[0].annotationObjects['process_type'], 'node')
  t.is(result.moduleList[0].annotationObjects['pid'], '12259')
})

test('should get system info correctly', (t) => {
  const file = resolveDumpFile('mac-electron-browser.dmp')

  const result = new Minidump(file).getSystemInfo()
  t.deepEqual(result, {
    os: 'macOs',
    cpu: 'x86_64',
  })
})

test('should get misc info correctly', (t) => {
  const file = resolveDumpFile('mac-electron-browser.dmp')

  const result = new Minidump(file).getMiscInfo()
  t.deepEqual(result, {})
})
