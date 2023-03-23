---
outline: deep
prev: false
next: false
---

# λ-Calculus Online Playground

Try writing your lambda expression! We've defined some basic expressions for you:

<script setup>
import LambdaPlayground from '../../components/LambdaPlayground.vue'
import { data } from '../../components/playground.data'
</script>

<LambdaDefs :file="data" />

You can click on underlined part to make beta reduction,
or click on predefined named variables to expand them.

<ClientOnly>
  <LambdaPlayground />
</ClientOnly>