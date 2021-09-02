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
//
d.del(3, 1) // "hi here"
console.log(d.get())

const version = d.get_vector_clock()
console.log('vector clock', version)


const all_txns = d.get_txn_since()
console.log('all txns', all_txns)

// const d2 = new Doc('bbbb')
// d2.ins(0, 'yoooo')
// d2.merge_remote_txns(all_txns)
// console.log(`resulting document '${d2.get()}'`) // 'hi there'

// const frontier = d.get_frontier()
// console.log('frontier', frontier)

// console.log('positional changes', d.positional_ops_since(0))
// console.log('all traversal changes', d.traversal_ops_since())
// console.log('some traversal changes', d.traversal_ops_since([
//   {agent: 'aaaa', seq: 8}
// ]))


console.log('order', d.get_next_order())
const patches = d.attributed_patches_since_order(0)
console.log('patches since 0', patches)