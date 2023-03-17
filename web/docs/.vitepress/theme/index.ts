import type { EnhanceAppContext } from 'vitepress/dist/client'
import DefaultTheme from 'vitepress/theme'
import './custom.css'
import 'katex/dist/katex.min.css'

import LambdaStatic from '../../../components/lambda/LambdaStatic.vue'
import LambdaRaw from '../../../components/lambda/LambdaRaw.vue'

export default {
  extends: DefaultTheme,
  enhanceApp(ctx: EnhanceAppContext) {
    // register your custom global components
    ctx.app.component('LambdaStatic', LambdaStatic)
    ctx.app.component('LambdaRaw', LambdaRaw)
  }
}