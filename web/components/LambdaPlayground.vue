<script setup lang="ts">
import { LambdaExp } from './lambda';
import LambdaInteractive from './LambdaInteractive.vue';
import { ref } from 'vue';
import { useDebounceFn } from '@vueuse/core'

// 不能直接 import 函数，不然 wasm 初始化的部分会被 tree-shake 删掉
import * as lamcalc from 'lamcalc'
const { Calculator } = lamcalc;

const inputContent = ref('Y I')
const expStr = ref(inputContent.value)
const calc = new Calculator()
calc.init(inputContent.value);

const steps = ref(calc.history())
const error = ref('');
const file = `
Y = \\f. (\\x. f (x x)) \\x. f (x x)
I = \\z. z
`

const initWithStr = useDebounceFn((s: string) => {
  expStr.value = s
  console.log('init!', s)
  try {
    calc.init(s)
    steps.value = calc.history()
    error.value = ''
  } catch (e) {
    error.value = e
  }
}, 500)

const onInput = (event: Event) => {
  initWithStr(inputContent.value);
}

const onReduce = (step: number, id: number) => {
  console.log('reduce', step, id)
  calc.beta_reduce(step, id)
  steps.value = calc.history()
}

</script>

<template>
  <div class="input-wrapper">
    <input type="text" v-model="inputContent" placeholder="enter your lambda" @input="onInput" />
  </div>
  <LambdaInteractive :exp="expStr" :file="file" />
</template>

<style>
.input-wrapper {
  border: 1px solid var(--vp-c-bg-soft-mute);
  background-color: var(--vp-c-bg-soft);
  padding: 8px 16px;
  margin: 16px 0;
  border-radius: 4px;
  transition: all .2s ease-in-out;
}

.input-wrapper:hover {
  border: 1px solid var(--vp-c-brand);
}

.input-wrapper input {
  width: 100%;
  font-size: 16px;
  font-family: 'Courier New', Courier, monospace;
}

.input-wrapper .input-area {
  font-size: 16px;
  font-family: 'Courier New', Courier, monospace;
  min-height: 16px;
}
</style>