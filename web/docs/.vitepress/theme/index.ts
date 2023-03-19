import type { EnhanceAppContext } from 'vitepress/dist/client'
import DefaultTheme from 'vitepress/theme'
import './custom.css'
import 'katex/dist/katex.min.css'

import LambdaStatic from '../../../components/LambdaStatic.vue'
import LambdaRaw from '../../../components/LambdaRaw.vue'
import LambdaInteractive from '../../../components/LambdaInteractive.vue'
import LambdaDef from '../../../components/LambdaDef.vue'

export default {
  extends: DefaultTheme,
  enhanceApp(ctx: EnhanceAppContext) {
    // register your custom global components
    ctx.app.component('LambdaStatic', LambdaStatic)
    ctx.app.component('LambdaRaw', LambdaRaw)
    ctx.app.component('LambdaInteractive', LambdaInteractive)
    ctx.app.component('LambdaDef', LambdaDef)
  }
}