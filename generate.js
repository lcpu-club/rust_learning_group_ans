// @ts-check
import 'zx/globals'
import { parse, stringify } from 'yaml'
import consola from 'consola'

/**
 * @param {string} code
 * @returns {{ template: string, ojMerge: boolean }}
 */
function extractTemplate(code) {
  const lines = code.split('\n')
  let parsing = false
  let ojMerge = false
  /** @type {string[]} */
  const result = []
  for (const line of lines) {
    if (line === "#![cfg(not(oj_no_merge))]") {
      ojMerge = true
    }
    if (!parsing && line === '/// ```no_run') {
      parsing = true
      continue
    }
    if (parsing && line === '/// ```') {
      parsing = false
      return {
        template: result.join('\n').trimEnd() + '\n',
        ojMerge,
      }
    }
    if (parsing) {
      result.push(line.slice(4).trimEnd())
    }
  }
  consola.fatal('No template found')
  process.exit(1)
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
const { template, ojMerge: ojMergeInCode } = extractTemplate(code)

// Check if the .oj-merge file exists
const ojMergeFilePath = path.join(__dirname, 'fixtures', name, '.oj-merge')
const ojMergeInData = await fs.exists(ojMergeFile);

const ojMerge = ojMergeInCode || ojMergeInData;
if (ojMerge) {
  consola.info(`Merging enabled for ${name}`)
  // Create the .oj-merge file in the data directory
  await $`touch ${__dirname}/build/${name}/data/.oj-merge`

  // Copy the source file to .source.rs in the data directory
  const mergeSourcePath = path.join(__dirname, 'fixtures', name, '.source.rs')
  if (!(await fs.exists(mergeSourcePath))) {
    await $`cp ${source} ${__dirname}/build/${name}/data/.source.rs`
  }
}

if (template) {
  if (template.includes('// FIX ME')) {
    await fs.writeFile(path.join(__dirname, 'build', name, 'data', 'template.rs'), template)
    problemConfig.submit.form.files[0].description =
      '注意：只允许修改标有 `// FIX ME` 的行，否则直接计0分。'
  } else {
    problemConfig.submit.form.files[0].description = '请基于如下Rust代码模板修改并提交。'
  }

  if (ojMerge) {
    problemConfig.submit.form.files[0].description += '\n注意：本题另包含未公开的评测代码，且保留 `judge` 模块。你的作答将会被拼接在评测代码之后。'
  }
  problemConfig.submit.form.files[0].default = template
}
await fs.writeFile(path.join(__dirname, 'build', name, 'problem.yml'), stringify(problemConfig))

const statement = code
  .split('\n')
  .filter((line) => line.startsWith('///'))
  .map((line) => line.slice(4))
  .join('\n')
  .replace(/```no_run/g, '```rust')
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
