import { defineConfig } from 'vitepress'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import markdownItFootnote from 'markdown-it-footnote'

// 进行文本替换
const macros = [{
  pattern: /{L}/g,
  replacer: () => 'λ-Calculus'
}, {
  pattern: /L`(.*?)`/g,
  replacer: ($, $1) => `<code>${$1}</code>`
}]

// https://vitepress.dev/reference/site-config
export default defineConfig({
  description: "Learn lambda calculus interactively!",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      // { text: 'Examples', link: '/markdown-examples' },
      { text: 'Playground', link: '/playground' },
    ],


    socialLinks: [
      { icon: 'github', link: 'https://github.com/sshwy/lamcalc' }
    ],
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2023-present Weiyao Huang'
    }
  },
  vite: {
    plugins: [
      wasm(),
      topLevelAwait()
    ]
  },
  locales: {
    root: {
      title: "λ-Calculus: 道生万物",
      titleTemplate: ":title | λ-Calculus: 道生万物",
      label: '简体中文',
      lang: 'zh',
      themeConfig: {
        sidebar: [
          {
            text: '章节列表',
            items: [
              { text: '梦的开始', link: '/ch01-introduction' },
              { text: '函数与符号', link: '/ch02-function' },
            ]
          }
        ],
      }
    },
    en: {
      title: "Road to λ-Calculus",
      titleTemplate: ":title | Road to λ-Calculus",
      label: 'English',
      lang: 'en', // optional, will be added  as `lang` attribute on `html` tag
      link: '/en/',
      themeConfig: {
        sidebar: [
          {
            text: 'Examples',
            items: [
              { text: 'Introduction', link: '/ch01-introduction' },
              { text: 'Runtime API Examples', link: '/api-examples' }
            ]
          }
        ],
      }
    }
  },
  markdown: {
    config: function (md) {

      md.use(markdownItFootnote)
      md.renderer.rules.footnote_anchor = function (tokens, idx, options, env, slf) {
        var id = slf.rules.footnote_anchor_name(tokens, idx, options, env, slf);

        if (tokens[idx].meta.subId > 0) {
          id += ':' + tokens[idx].meta.subId;
        }

        return ' <a href="#fnref' + id + '" class="footnote-backref">#</a>';
      }

      md.core.ruler.before('normalize', 'replace_macro', (state) => {
        macros.forEach(({ pattern, replacer }) => {
          state.src = state.src.replace(pattern, replacer)
        })
      })
    }
  }
})
