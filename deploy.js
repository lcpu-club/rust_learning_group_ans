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
cd(`${__dirname}/build/${name}`)
$.env.FORCE_COLOR = '1'
await $`yarn aoi problem deploy -s`
