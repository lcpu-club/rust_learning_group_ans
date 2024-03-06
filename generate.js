// @ts-check
import 'zx/globals'
import { parse, stringify } from 'yaml'
import consola from 'consola'

function extractTemplate(code) {
  const regex = /\/\*- template-start -\*\/([\s\S]*?)\/\*- template-end -\*\//g
  let match
  let result = ''
  while ((match = regex.exec(code)) !== null) {
    if (match[1]) {
      result += match[1].trim() + '\n'
    }
  }
  const lines = result
    .split('\n')
    .filter((_, i) => !i || !result[i - 1].trim().startsWith('//- replace-with '))
    .map((line) => line.replace('//- replace-with ', ''))
  return lines.join('\n')
}

const mappings = parse(await fs.readFile(path.join(__dirname, 'mappings.yml'), 'utf8'))
/** @type {string} */
const name = argv.name
if (!name) {
  consola.fatal('Name is required')
  process.exit(1)
}
if (!mappings[name]) {
  consola.fatal(`No mapping found for ${name}`)
  process.exit(1)
}
const source = path.join(__dirname, 'src', 'bin', `${name}.rs`)
if (!(await fs.exists(source))) {
  consola.fatal(`No source found for ${name}`)
  process.exit(1)
}
const data = path.join(__dirname, 'fixtures', name)
if (!(await fs.exists(data))) {
  consola.fatal(`No data found for ${name}`)
  process.exit(1)
}
await $`rm -rf ${__dirname}/build/${name}`
await $`mkdir -p ${__dirname}/build/${name}`
await $`cp -r ${__dirname}/fixtures/common ${__dirname}/build/${name}/data`
await $`cp -r ${data}/* ${__dirname}/build/${name}/data`
await $`cp ${source} ${__dirname}/build/${name}/main.rs`

const problemConfig = parse(await fs.readFile(path.join(__dirname, 'problem.yml'), 'utf8'))
const code = await fs.readFile(source, 'utf8')
const template = extractTemplate(code)
if (template) {
  await fs.writeFile(path.join(__dirname, 'build', name, 'data', 'template.rs'), template)
  problemConfig.submit.form.files[0].default = template
}
await fs.writeFile(path.join(__dirname, 'build', name, 'problem.yml'), stringify(problemConfig))

const statement = code
  .split('\n')
  .filter((line) => line.startsWith('/// '))
  .map((line) => line.slice(4))
  .join('\n')
const title = name
  .split('_')
  .map((word) => word[0].toUpperCase() + word.slice(1))
  .join(' ')
const md = `---
title: ${title}
tags:
  - Rust
---
# ${title}

${statement}`
await fs.writeFile(path.join(__dirname, 'build', name, 'statement.md'), md)

const aoiConfig = `
type: "problem"
server: hpcgame
problemId: ${mappings[name]}
`
await fs.writeFile(path.join(__dirname, 'build', name, 'aoi.config.yml'), aoiConfig)
consola.success(`Generated ${name}`)
