<script setup lang="ts">
import Ident from './Ident.vue';
import AbsHead from './AbsHead.vue';
import { computed, inject, ref } from 'vue';
import { useDebounceFn } from '@vueuse/shared';
import { betaReduceKey, replaceNameKey } from '../LambdaInteractive.vue';
import { decorKey } from './LambdaExp.vue';

const props = defineProps<{
  parentheses: boolean
  bracketLevel: number
  inner: any
  redexId?: number
}>()

const nextLevel = computed(() => props.bracketLevel + (props.parentheses ? 1 : 0))

const self = ref<HTMLElement | null>(null)
const param = ref<HTMLElement | null>(null)


const deco = inject(decorKey);
// console.log('deco', deco)
const replaceTrigger = inject(replaceNameKey)
const reduceTrigger = inject(betaReduceKey)

const onVarClick = (name: string) => {
  if (deco.value.names.includes(name)) {
    if (replaceTrigger) replaceTrigger(name, deco.value.step_id)
  }
}
const onDropReduce = useDebounceFn((redex: number) => {
  console.log('drop!', redex)
  reduceTrigger(redex, deco.value.step_id)
}, 50)
</script>

<template>
  <span ref="self" class="lambda">
    <span v-if="parentheses" :class="`lambda-bracket-${bracketLevel % 3}`">(</span>

    <span v-if="inner.Abs" class="lambda-abs">
      <AbsHead @beta-reduce="onDropReduce" :redex-id="redexId">
        <span class="lambda-lambda">λ</span>
        <Ident :ident="inner.Abs.ident" :de="0" />
      </AbsHead>
      <span class="lambda-dot">.</span>
      <Exp v-bind="inner.Abs.body" :bracket-level="nextLevel" />
    </span>
    <span v-else-if="inner.App"
      :class="['lambda-app', deco.lastRedex && inner.App.beta_redex === deco.lastRedex ? 'lambda-redex' : '']">
      <Exp class="lambda-app-func" :redex-id="inner.App.beta_redex" v-bind="inner.App.func" :bracket-level="nextLevel" />
      <span class="lambda-blank"> {{ " " }} </span>
      <Exp class="lambda-app-body" ref="param" v-bind="inner.App.body" :bracket-level="nextLevel" />
    </span>
    <span v-else-if="inner.Var" :class="[deco.names.includes(inner.Var.ident) ?
      'lambda-const' : 'lambda-var']" @click="onVarClick(inner.Var.ident)">
      <Ident :ident="inner.Var.ident" :de="inner.Var.code" />
    </span>

    <span v-if="parentheses" :class="`lambda-bracket-${bracketLevel % 3}`">)</span>
  </span>
</template>