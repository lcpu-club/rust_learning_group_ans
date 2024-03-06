// @ts-check
import 'zx/globals'
import { parse, stringify } from 'yaml'
import consola from 'consola'

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
await $`cp ${__dirname}/problem.yml ${__dirname}/build/${name}/`
await $`cp -r ${__dirname}/fixtures/common ${__dirname}/build/${name}/data`
await $`cp -r ${data}/* ${__dirname}/build/${name}/data`
await $`cp ${source} ${__dirname}/build/${name}/main.rs`

const code = await fs.readFile(source, 'utf8')
const statement = code
  .split('\n')
  .filter((line) => line.startsWith('/// '))
  .map((line) => line.slice(4))
  .join('\n')
const title = name
  .split('_')
  .map((word) => word[0].toUpperCase() + word.slice(1))
  .join(' ')
const md = `# ${title}

${statement}`
await fs.writeFile(path.join(__dirname, 'build', name, 'statement.md'), md)

const aoiConfig = `
type: "problem"
server: hpcgame
problemId: ${mappings[name]}
`
await fs.writeFile(path.join(__dirname, 'build', name, 'aoi.config.yml'), aoiConfig)
consola.success(`Generated ${name}`)