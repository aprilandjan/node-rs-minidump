# `node-rs-minidump`

Parse minidump files in node.js, in use of [rust-minidump](https://github.com/rust-minidump/rust-minidump).

# Usage

> yarn add node-rs-minidump

## Support matrix

### Operating Systems

|             | node14 | node16 | node18 |
| ----------- | ------ | ------ | ------ |
| Windows x64 | ✓      | ✓      | ✓      |
| Windows x32 | ✓      | ✓      | ✓      |
| macOS x64   | ✓      | ✓      | ✓      |
| macOS arm64 | ✓      | ✓      | ✓      |

### CI

With GitHub Actions, each commit and pull request will be built and tested automatically in [`node@14`, `node@16`, `@node18`] x [`macOS`, `Linux`, `Windows`] matrix. You will never be afraid of the native addon broken in these platforms.

### Release

Release native package is very difficult in old days. Native packages may ask developers who use it to install `build toolchain` like `gcc/llvm`, `node-gyp` or something more.

With `GitHub actions`, we can easily prebuild a `binary` for major platforms. And with `N-API`, we should never be afraid of **ABI Compatible**.

The other problem is how to deliver prebuild `binary` to users. Downloading it in `postinstall` script is a common way that most packages do it right now. The problem with this solution is it introduced many other packages to download binary that has not been used by `runtime codes`. The other problem is some users may not easily download the binary from `GitHub/CDN` if they are behind a private network (But in most cases, they have a private NPM mirror).

In this package, we choose a better way to solve this problem. We release different `npm packages` for different platforms. And add it to `optionalDependencies` before releasing the `Major` package to npm.

`NPM` will choose which native package should download from `registry` automatically. You can see [npm](./npm) dir for details. And you can also run `yarn add node-rs-minidump` to see how it works.

## Develop requirements

- Install the latest `Rust`
- Install `Node.js@18`
- Install `yarn@4`

## Test in local

- yarn
- yarn build
- yarn test

And you will see:

```bash
$ ava --verbose

  ✔ sync function from native code
  ✔ sleep function from native code (201ms)
  ─

  2 tests passed
✨  Done in 1.12s.
```

## Release package

Ensure you have set your **NPM_TOKEN** in the `GitHub` project setting.

In `Settings -> Secrets`, add **NPM_TOKEN** into it.

When you want to release the package:

```
npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]

git push
```

GitHub actions will do the rest job for you.
