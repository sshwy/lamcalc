<script setup lang="ts">
import { parse_def } from 'lamcalc';
import ExpStatic from './lambda/ExpStatic.vue'

const props = defineProps<{
  file: string
}>()

const data: { name: string, exp: any }[] = [];
props.file.split('\n').forEach(line => {
  try {
    let res = parse_def(line)
    data.push({
      name: res[0],
      exp: res[1]
    })
  } catch { }
})
// console.log(data)

</script>
<template>
  <div class="lambda-exp-static-block lambda-defs">
    <div class="lambda-inner">
      <div class="lambda-def-line" v-for="{ name, exp } in data" :key="name">
        <span class="lambda-const"><span class="lambda-ident">{{ name }}</span></span>
        <span class="lambda-eq"> = </span>
        <ExpStatic v-bind="exp" :bracket-level="0" />
      </div>
    </div>
  </div>
</template>