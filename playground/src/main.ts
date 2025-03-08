import { mount } from 'svelte'
import './app.css'
import App from './App.svelte'
import * as BrowserFS from 'browserfs'
import type { FSModule } from 'browserfs/dist/node/core/FS'
import type FS from 'browserfs/dist/node/core/FS'
import { merge, pick } from 'lodash-es'
import lineColumn from 'line-column'

console.log(lineColumn("l").fromIndex(0))
console.log(lineColumn("l").fromIndex(1))


globalThis.readfile = (path: string): string[] => {
  const fs = BrowserFS.BFSRequire('fs');
  return fs.readFileSync(path).toString('utf-8')
}

globalThis.readdir = (path: string): string[] => {
  const fs = BrowserFS.BFSRequire('fs');
  return fs.readdirSync(path)
}

globalThis.stat = (path: string) => {
  const fs = BrowserFS.BFSRequire('fs');
  const stats = fs.statSync(path)
  return merge(pick(stats, ['size']), { isDirectory: stats.isDirectory() })
}

globalThis.unlink = (path: string) => {
  const fs = BrowserFS.BFSRequire('fs');
  fs.unlinkSync(path);
}

const app = mount(App, {
  target: document.getElementById('app')!,
})

export default app
