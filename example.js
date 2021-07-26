const {Console} = require('console')
global.console = new Console({
  stdout: process.stdout,
  stderr: process.stderr,
  inspectOptions: {depth: null}
})

const {Doc} = require('./pkg/diamond_js.js')
const d = new Doc("aaaa")
d.ins(0, "hi there")

const version = d.get_version()
console.log('version', version)

const all_txns = d.get_txn_since([])
console.log('all txns', all_txns)

const d2 = new Doc()
d2.merge_remote_txns(all_txns)
console.log(`resulting document '${d2.get()}'`) // 'hi there'
