import type { EnhanceAppContext } from 'vitepress/dist/client'
import DefaultTheme from 'vitepress/theme'
import LambdaStatic from '../../../components/lambda/LambdaStatic.vue'
import './custom.css'

export default {
  extends: DefaultTheme,
  enhanceApp(ctx: EnhanceAppContext) {
    // register your custom global components
    ctx.app.component('LambdaStatic', LambdaStatic)
  }
}