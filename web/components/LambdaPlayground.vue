<script setup lang="ts">
import { LambdaExp } from './lambda';
import { ref } from 'vue';
import { useDebounceFn } from '@vueuse/core'

// 不能直接 import 函数，不然 wasm 初始化的部分会被 tree-shake 删掉
import * as lamcalc from 'lamcalc'
const { Calculator } = lamcalc;

const calc = new Calculator()

// const str = ref(`(\\x.\\y.x y x) (\\x.\\y.x)`)
const inputContent = ref('\\f. (\\x. f (x x)) \\x. f (x x)')
const startExp = ref(calc.init(inputContent.value));
const steps = ref([])

const error = ref('');

const initWithStr = useDebounceFn((s: string) => {
  startExp.value = calc.init(s)
}, 500)

const onInput = (event: Event) => {
  initWithStr(inputContent.value);
  steps.value = []
}

const onReduce = (step: number, id: number) => {
  console.log('reduce', step, id)
  let res = calc.beta_reduce(step, id)
  steps.value = steps.value.slice(0, step);
  steps.value.push(res)
}

</script>

<template>
  <div class="input-wrapper">
    <input type="text" v-model="inputContent" placeholder="enter your lambda" @input="onInput" />
  </div>
  <pre v-if="error" class="error">{{ error }}</pre>
  <LambdaExp :exp="startExp" @beta-reduce="id => onReduce(0, id)" />
  <template v-for="[exp, id], step_id in steps" :key="id">
    <LambdaExp :exp="exp" @beta-reduce="id => onReduce(step_id + 1, id)" />
  </template>
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