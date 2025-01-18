import { createEffect, createSignal } from 'solid-js'
import solidLogo from './assets/solid.svg'
import init, { greet } from './wasm/nushell_wasm'
import './App.css'

function App() {
  const [code, setCode] = createSignal("")

  return (
    <div>
      <textarea rows={10} cols={30} value={code()} onChange={e => setCode(e.target.value)} />
    </div>
  )
}

export default App
