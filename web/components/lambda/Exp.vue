<script setup lang="ts">
import Ident from './Ident.vue';
import AbsHead from './AbsHead.vue';
import { computed, inject, ref } from 'vue';
import { useDebounceFn } from '@vueuse/core';
import { betaReduceKey, etaReduceKey, replaceNameKey } from '../LambdaInteractive.vue';
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
const betaReduceTrigger = inject(betaReduceKey)
const etaReduceTrigger = inject(etaReduceKey)

const onVarClick = (v: { ident: string, alpha_id: number }) => {
  if (deco.value.names.includes(v.ident)) {
    if (replaceTrigger) replaceTrigger(v.ident, v.alpha_id, deco.value.step_id)
  }
}
const onBetaReduce = useDebounceFn((redex: number) => {
  console.log('beta!', redex)
  betaReduceTrigger(redex, deco.value.step_id)
}, 50)
const onEtaReduce = useDebounceFn((redex: number) => {
  console.log('eta!', redex)
  etaReduceTrigger(redex, deco.value.step_id)
}, 50)
</script>

<template>
  <span ref="self" class="lambda">
    <span v-if="parentheses" :class="`lambda-bracket-${bracketLevel % 3}`">(</span>

    <span v-if="inner.Abs" :class="['lambda-abs',
      deco.etaRedex && (deco.etaRedex.redex === inner.Abs.eta_redex) ? 'lambda-eta-redex' : ''
    ]">
      <AbsHead @beta-reduce="onBetaReduce" @eta-reduce="onEtaReduce" :redex-id="redexId"
        :eta-redex-id="deco.allowEtaReduce ? inner.Abs.eta_redex : undefined">
        <span class="lambda-lambda">λ</span>
        <Ident :alpha="inner.Abs.alpha_id" :ident="inner.Abs.ident" :de="0" />
      </AbsHead>
      <span class="lambda-dot">.</span>
      <Exp v-bind="inner.Abs.body" :bracket-level="nextLevel" />
    </span>
    <span v-else-if="inner.App" :class="['lambda-app',
      deco.betaRedex && (deco.betaRedex.redex === inner.App.beta_redex) ? 'lambda-beta-redex' : ''
    ]">
      <Exp class="lambda-app-func" :redex-id="inner.App.beta_redex" v-bind="inner.App.func" :bracket-level="nextLevel" />
      <span class="lambda-blank"> {{ " " }} </span>
      <Exp class="lambda-app-body" ref="param" v-bind="inner.App.body" :bracket-level="nextLevel" />
    </span>
    <span v-else-if="inner.Var" :class="[deco.names.includes(inner.Var.ident) ?
      'lambda-const' : 'lambda-var']" @click="onVarClick(inner.Var)">
      <Ident :alpha="inner.Var.alpha_id" :ident="inner.Var.ident" :de="inner.Var.code" />
    </span>

    <span v-if="parentheses" :class="`lambda-bracket-${bracketLevel % 3}`">)</span>
  </span>
</template>