const path = require('path')

const fs = require('fs-extra')

const pkg = require('./package.json')

const publishPath = path.join(__dirname, './npm-combined')

// create the temp publish folder
fs.ensureDirSync(publishPath)
fs.emptyDirSync(publishPath)

// modify pkg name & content
pkg.name = pkg.name + '-combined'
pkg.dependencies = {
  ...pkg.optionalDependencies,
}
pkg.optionalDependencies = {}
pkg.devDependencies = {}
pkg.scripts = {}

// write needed file
fs.writeJsonSync(path.join(publishPath, 'package.json'), pkg)
fs.copyFileSync(path.join(__dirname, 'index.js'), path.join(publishPath, 'index.js'))
fs.copyFileSync(path.join(__dirname, 'index.d.ts'), path.join(publishPath, 'index.d.ts'))
fs.copyFileSync(path.join(__dirname, 'LICENSE'), path.join(publishPath, 'LICENSE'))
fs.copyFileSync(path.join(__dirname, 'README-combined.md'), path.join(publishPath, 'README.md'))
