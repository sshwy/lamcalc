<script setup lang="ts">
import { parse_def } from 'lamcalc';
import { ref } from 'vue';
import ExpStatic from './lambda/ExpStatic.vue'

const props = defineProps<{
  block?: boolean
  exp: string
}>()

const data = ref(parse_def(props.exp))

</script>
<template>
  <template v-if="block">
    <div class="lambda-exp-static-block">
      <div class="lambda-inner">
        <span class="lambda-const"><span class="lambda-ident">{{ data[0] }}</span></span>
        <span class="lambda-eq"> = </span>
        <ExpStatic v-bind="data[1]" :bracket-level="0" />
      </div>
    </div>
  </template>
  <template v-else>
    <span class="lambda-inline">
        <span class="lambda-const"><span class="lambda-ident">{{ data[0] }}</span></span>
      <span class="lambda-eq"> = </span>
      <ExpStatic v-bind="data[1]" :bracket-level="0" />
    </span>
  </template>
</template>