import { mount } from 'svelte'
import './app.css'
import App from './App.svelte'
import * as BrowserFS from 'browserfs'
import { merge, pick } from 'lodash-es'
import lineColumn from 'line-column'

console.log(lineColumn("l").fromIndex(0))
console.log(lineColumn("l").fromIndex(1))


const exportedFunctions = {
  readfile(path: string): string {
    const fs = BrowserFS.BFSRequire('fs');
    return fs.readFileSync(path).toString('utf-8')
  },
  readdir(path: string): string[] {
    const fs = BrowserFS.BFSRequire('fs');
    return fs.readdirSync(path)
  },
  stat(path: string) {
    const fs = BrowserFS.BFSRequire('fs');
    const stats = fs.statSync(path)
    return merge(pick(stats, ['size']), { isDirectory: stats.isDirectory() })
  },
  unlink(path: string) {
    const fs = BrowserFS.BFSRequire('fs');
    fs.unlinkSync(path);
  }
}

Object.defineProperties<typeof exportedFunctions | typeof globalThis>(globalThis, exportedFunctions);

const app = mount(App, {
  target: document.getElementById('app')!,
})

export default app
