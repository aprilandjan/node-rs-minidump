import path from 'path'

import b from 'benny'

import { getCrashpadInfo } from '../index'

function resolveDumpFile(name: string) {
  return path.join(__dirname, '../fixtures/', name)
}

async function run() {
  await b.suite(
    'getCrashpadInfo',

    b.add('parse mac electron main process dump file', () => {
      const file = resolveDumpFile('mac-electron-browser.dmp')
      getCrashpadInfo(file)
    }),

    b.add('parse mac electron renderer process dump file', () => {
      const file = resolveDumpFile('mac-electron-renderer.dmp')
      getCrashpadInfo(file)
    }),

    b.add('parse mac electron gpu process dump file', () => {
      const file = resolveDumpFile('mac-electron-gpu.dmp')
      getCrashpadInfo(file)
    }),

    b.add('parse mac electron node child process dump file', () => {
      const file = resolveDumpFile('mac-electron-node.dmp')
      getCrashpadInfo(file)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
