import fs from 'node:fs'
import { defineLoader } from 'vitepress'

export type Data = string
declare const data: Data
export { data }

export default defineLoader({
  watch: ['./playground.lambda'],
  load(watchedFiles) {
    // watchedFiles will be an array of absolute paths of the matched files.
    // generate an array of blog post metadata that can be used to render
    // a list in the theme layout
    return fs.readFileSync(watchedFiles[0], 'utf-8').toString()
  }
})