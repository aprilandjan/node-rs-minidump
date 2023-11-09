const path = require('path')

const fs = require('fs-extra')

const pkg = require('./package.json')

const targetsPath = path.join(__dirname, './npm')
const combinedPath = path.join(__dirname, './npm-combined')

// create the temp publish folder
fs.ensureDirSync(combinedPath)
fs.emptyDirSync(combinedPath)

// copy each target binaries
const targets = fs.readdirSync(targetsPath)
const binaries = []
targets.forEach((target) => {
  const targetPkg = path.join(targetsPath, target, './package.json')
  const mainFileName = fs.readJsonSync(targetPkg).main
  const mainFilePath = path.join(targetsPath, target, mainFileName)
  if (fs.existsSync(mainFilePath)) {
    binaries.push(mainFileName)
    fs.copyFileSync(mainFilePath, path.join(combinedPath, mainFileName))
  }
})

// modify pkg name & content
pkg.name = pkg.name + '-combined'
pkg.optionalDependencies = {}
pkg.devDependencies = {}
pkg.scripts = {}
pkg.files = {
  ...pkg.files,
  ...binaries,
}

// copy needed files
fs.writeJsonSync(path.join(combinedPath, 'package.json'), pkg)
fs.copyFileSync(path.join(__dirname, 'index.js'), path.join(combinedPath, 'index.js'))
fs.copyFileSync(path.join(__dirname, 'index.d.ts'), path.join(combinedPath, 'index.d.ts'))
fs.copyFileSync(path.join(__dirname, 'LICENSE'), path.join(combinedPath, 'LICENSE'))
fs.copyFileSync(path.join(__dirname, 'README-combined.md'), path.join(combinedPath, 'README.md'))
