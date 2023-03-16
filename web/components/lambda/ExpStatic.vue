<script setup lang="ts">
import Ident from './Ident.vue';
import { computed } from 'vue';

const props = defineProps<{
  marked: boolean
  parentheses: boolean
  bracketLevel: number
  inner: any
}>()

const nextLevel = computed(() => props.bracketLevel + (props.parentheses ? 1 : 0))

</script>

<template>
  <span class="lambda">
    <span v-if="parentheses" :class="`lambda-bracket-${bracketLevel % 3}`">(</span>

    <span class="lambda-abs" v-if="inner.Abs">
      <span class="lambda-lambda">λ</span>
      <Ident :ident="inner.Abs.ident" :de="0" />
      <span class="lambda-dot">.</span>
      <ExpStatic v-bind="inner.Abs.body" :bracket-level="nextLevel" />
    </span>
    <span class="lambda-app" v-else-if="inner.App">
      <ExpStatic v-bind="inner.App.func" :bracket-level="nextLevel" />
      <span class="lambda-blank"> {{ " " }} </span>
      <ExpStatic v-bind="inner.App.body" :bracket-level="nextLevel" />
    </span>
    <span v-else-if="inner.Var" class="lambda-var">
      <Ident :ident="inner.Var.ident" :de="inner.Var.code" />
    </span>

    <span v-if="parentheses" :class="`lambda-bracket-${bracketLevel % 3}`">)</span>
  </span>
</template>

<style>
.lambda-lambda {
  color: var(--vp-c-lambda-lambda);
}

.lambda-dot {
  color: var(--vp-c-lambda-dot);
}

.lambda-bracket-0 {
  color: var(--vp-c-lambda-bracket-0)
}

.lambda-bracket-1 {
  color: var(--vp-c-lambda-bracket-1)
}

.lambda-bracket-2 {
  color: var(--vp-c-lambda-bracket-2)
}

.lambda-ref {
  color: var(--vp-c-lambda-ref)
}

.lambda {
  transition: .1s ease-in-out;
  transition-property: padding, border;
}

.lambda-highlight {
  border: 2px solid var(--vp-c-lambda-lambda);
  display: inline-block;
  padding: 2px 4px 0px;
  border-radius: 6px;
}

.lambda-highlight:hover {
  background-color: var(--vp-c-lambda-hlbg);
}


.lambda-dragover {
  background-color: var(--vp-c-lambda-hlbg);
  border: 2px solid var(--vp-c-lambda-lambda);
  display: inline-block;
  padding: 2px 4px 0px;
  border-radius: 6px;
}
</style>