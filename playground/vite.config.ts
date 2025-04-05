import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/postcss'
import Icons from 'unplugin-icons/vite'
import path from 'path'
import fs from 'fs-extra'
import mime from 'mime-types'

import postcssPresetEnv from 'postcss-preset-env'

const monacoBasePath = path.dirname(import.meta.resolve('monaco-editor/package.json')).replace(/^file:/, "")
const monacoMinPath = path.join(monacoBasePath, 'min/vs')

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    svelte(),
    Icons({ compiler: 'svelte' }),
    {
      name: 'monaco-editor-assets-dev',
      apply: 'serve',
      configureServer(server) {
        server.middlewares.use(async (req, res, next) => {
          try {
            const url = req.url ?? ''
            if (!url.startsWith("/nu-on-web/assets/vs/"))
              return next()
            const filePath = path.relative("/nu-on-web/assets/vs/", url)
            const monacoPath = path.join(monacoMinPath, filePath)

            if (!await fs.pathExists(monacoPath)) return next()

            const stat = await fs.stat(monacoPath)
            if (!stat.isFile()) return next()

            const etag = `"${stat.size}-${stat.mtime.getTime()}"`

            if (req.headers['if-none-match'] === etag) {
              res.statusCode = 304 // Not Modified
              res.end()
              return
            }

            res.setHeader('Content-Type', mime.lookup(monacoPath) || 'application/octet-stream')
            res.setHeader('Content-Length', stat.size)
            res.setHeader('Cache-Control', 'max-age=31536000,immutable') // Cache for 1 year
            res.setHeader('ETag', etag)
            fs.createReadStream(monacoPath).pipe(res)
          } catch (e) {
            next(e)
          }
        })
      }
    },
    {
      name: 'monaco-editor-assets-prod',
      apply: 'build',
      closeBundle: async () => {
        const monacoDestDir = path.resolve('./dist/assets/vs')

        if (await fs.pathExists(monacoMinPath)) {
          await fs.copy(monacoMinPath, monacoDestDir)
        }
      }
    }
  ],
  base: "/nu-on-web/",
  css: {
    transformer: 'postcss',
    postcss: {
      plugins: [
        postcssPresetEnv(),
        tailwindcss()
      ]
    }
  },
})
