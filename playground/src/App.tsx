import * as BrowserFS from 'browserfs'
import { Show } from 'solid-js';
import { createSignal } from 'solid-js';
import { MonacoEditor } from 'solid-monaco';
import init, { run_code } from "./wasm/nushell_wasm"


function initBrowserFS() {
  return new Promise<void>((res, rej) => {
    BrowserFS.install(window)
    BrowserFS.configure({ fs: "LocalStorage", options: {} }, e => {
      if (e) {
        rej(e)
      }
      res()
    })
  })

}


function App() {
  const [ready, setReady] = createSignal(false)
  Promise.all([initBrowserFS(), init()]).then(() => {
    setReady(true)
  })
  return <Show when={ready}><Editor /></Show>
}

function Editor() {
  console.log(new Date())
  const [code, setCode] = createSignal("")
  const [result, setResult] = createSignal<string>()
  function runCode() {
    console.log(new Date())
    const runCodeResult = JSON.parse(run_code(code()));
    setResult(JSON.stringify(runCodeResult, null, 2))
  }

  return <div>
    <MonacoEditor language="shell" value={code()} onChange={v => setCode(v)} />
    <div class='pl-[68px]'>
      <button class="w-16 bg-blue-300 hover:bg-blue-400 cursor-pointer" onClick={runCode}>RUN</button>
      <pre>{result()}</pre>
    </div>
  </div>;
}

export default App
