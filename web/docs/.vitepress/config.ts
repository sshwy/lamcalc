import { defineConfig } from 'vitepress'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import markdownItFootnote from 'markdown-it-footnote'
import markdownItKatex from './theme/katex'

// 进行文本替换
const macros: {
  pattern: RegExp,
  replacer: (substring: string, ...args: any[]) => string
}[] = [{
  pattern: /{L}/g,
  replacer: () => 'λ-Calculus'
}, {
  pattern: /{l}/g,
  replacer: () => 'λ'
}, {
  pattern: /L`(.*?)`/g,
  replacer: (_, $1) => `<code>${$1}</code>`
}]

// https://vitepress.dev/reference/site-config
export default defineConfig({
  description: "Learn lambda calculus interactively!",
  title: "λ-Calculus: 道生万物",
  titleTemplate: ":title | λ-Calculus: 道生万物",
  lastUpdated: true,
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: '首页', link: '/' },
      { text: '在线演绎 λ-Calculus', link: '/playground' },
    ],
    sidebar: [
      {
        text: '章节列表',
        items: [
          { text: '梦的开始', link: '/ch01-intro' },
          { text: '函数与符号', link: '/ch02-func' },
        ],
      },
      { text: '在线演绎 λ-Calculus', link: '/playground' },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/sshwy/lamcalc' }
    ],

    footer: {
      message: '本站内容遵循 MIT 许可协议',
      copyright: 'Copyright © 2023-present Weiyao Huang'
    },

    editLink: {
      pattern: 'https://github.com/sshwy/lamcalc/tree/master/web/docs/:path',
      text: '在 GitHub 上编辑此页',
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
      label: '简体中文',
      lang: 'zh',
    },
    en: {
      title: "Road to λ-Calculus",
      titleTemplate: ":title | Road to λ-Calculus",
      label: 'English',
      lang: 'en', // optional, will be added  as `lang` attribute on `html` tag
      link: '/en/',
      themeConfig: {
        siteTitle: "Road to λ-Calculus",
        nav: [
          { text: 'Home', link: '/en/' },
          { text: 'Playground', link: '/en/playground' },
        ],
        sidebar: [
          {
            text: 'Chapters',
            items: [
              { text: 'Introduction', link: '/en/ch01-intro' },
            ]
          },
          { text: 'Playground', link: '/en/playground' },
        ],
        footer: {
          message: 'Released under the MIT License.',
          copyright: 'Copyright © 2023-present Weiyao Huang'
        },
        editLink: {
          pattern: 'https://github.com/sshwy/lamcalc/tree/master/web/docs/:path',
          text: 'Edit this page on Github',
        }
      },
    }
  },
  markdown: {
    config: function (md) {

      md.use(markdownItFootnote)
      md.use(markdownItKatex)

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
