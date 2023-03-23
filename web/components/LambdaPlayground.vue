<script setup lang="ts">
import { data } from './playground.data'
// import { LambdaExp } from './lambda';
import LambdaInteractive from './LambdaInteractive.vue';
import { ref } from 'vue';
import { useDebounceFn } from '@vueuse/core'

const inputContent = ref('Y I')
const expStr = ref(inputContent.value)

const initWithStr = useDebounceFn((s: string) => {
  expStr.value = s
  console.log('init!', s)
}, 500)

const onInput = (event: Event) => {
  initWithStr(inputContent.value);
}
</script>

<template>
  <div class="input-wrapper">
    <input type="text" v-model="inputContent" placeholder="enter your lambda" @input="onInput" />
  </div>
  <LambdaInteractive :exp="expStr" :file="data" />
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