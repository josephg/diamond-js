const {Console} = require('console')
global.console = new Console({
  stdout: process.stdout,
  stderr: process.stderr,
  inspectOptions: {depth: null}
})

const {Doc} = require('./pkg/diamond_js.js')
const d = new Doc("aaaa")
d.ins(0, "hi there")
d.ins(3, "aaa")

d.del(3, 1) // "hi here"
console.log(d.get())

const version = d.get_version()
console.log('version', version)

const all_txns = d.get_txn_since()
console.log('all txns', all_txns)

// const d2 = new Doc('bbbb')
// d2.ins(0, 'yoooo')
// d2.merge_remote_txns(all_txns)
// console.log(`resulting document '${d2.get()}'`) // 'hi there'

const ot_changes = d.get_ops_since(0)
console.log('OT changes', ot_changes)
