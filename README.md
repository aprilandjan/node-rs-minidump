# `node-rs-minidump`

[![ci](https://github.com/aprilandjan/node-rs-minidump/actions/workflows/CI.yml/badge.svg)](https://github.com/aprilandjan/node-rs-minidump/actions/workflows/CI.yml)
[![npm-version](https://img.shields.io/npm/v/node-rs-minidump
)](https://www.npmjs.com/package/node-rs-minidump)
[![npm-dw](https://img.shields.io/npm/dw/node-rs-minidump)](https://www.npmjs.com/package/node-rs-minidump)

Node.js native binding for [rust-minidump](https://github.com/rust-minidump/rust-minidump). Useful if you want to parse minidump files in node.js(or electron app) environment.

Currently it is still in early development, only some of the functionalities supported, see `Usage` section below.

## Install

Most of the cases, you can install `node-rs-minidump` directly, the package manager will choose a suitable prebuilt binary for you:

```bash
$ yarn add node-rs-minidump
```

However, if you want to ensure that all prebuilt binaries are included, for example, when building electron app that the development environment is not the same with the distribution environment, you can use `node-rs-minidump-combined` for replacement:

```bash
$ yarn add node-rs-minidump-combined
```

## Usage

```ts
import { getCrashpadInfo } from 'node-rs-minidump';

const dumpFile = '/path/to/some/minidump-file.dmp';
const info = getCrashpadInfo(dumpFile);
console.log(info);
```

## Support matrix

|             | node14 | node16 | node18 |
| ----------- | ------ | ------ | ------ |
| Windows x64 | ✓      | ✓      | ✓      |
| Windows x32 | ✓      | ✓      | ✓      |
| macOS x64   | ✓      | ✓      | ✓      |
| macOS arm64 | ✓      | ✓      | ✓      |

## Develop requirements

- Install the latest `Rust`
- Install `Node.js@18`
- Install `yarn@4`

## Test in local

```bash
$ yarn
$ yarn build
$ yarn test
```

## Release package

```bash
$ npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]
$ git push
```

## TODO

- [x] export a [class](https://napi.rs/docs/concepts/class)
- [ ] add `get_system_info`
- [ ] add `get_misc_info`
- [ ] add `get_exception`
- [ ] add `get_module_list`
- [ ] add `get_thread_list`
- [ ] add `get_memory_list`

## References

- <https://github.com/rust-minidump/rust-minidump/tree/main/minidump>
- <https://github.com/rust-minidump/minidump-debugger>
- <https://nick.groenen.me/posts/rust-error-handling/>
- <https://github.com/dtolnay/thiserror>
