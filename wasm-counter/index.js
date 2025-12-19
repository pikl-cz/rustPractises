const wasm = require('./pkg/wasm_counter.js')

async function run() {
  await wasm.default() // inicializace
  const counter = new wasm.Counter()

  console.log('Počítadlo:', counter.get())
  counter.increment()
  console.log('Počítadlo po increment:', counter.get())
}

run()
