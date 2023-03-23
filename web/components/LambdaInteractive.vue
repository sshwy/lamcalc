<!-- 可以自己拖动进行化简的 Lambda 表达式. Custom Only! -->
<script setup lang="ts">
import { Calculator } from 'lamcalc';
import { onMounted, provide, reactive, watch } from 'vue';
import { LambdaExp } from './lambda';

const props = defineProps<{
  exp: string
  file?: string
}>()

const data = reactive({
  last_exp: '',
  error: '',
  steps: [],
  defs: new Map(),
})

const calc = new Calculator()

const resetExp = (exp: string) => {
  try {
    data.last_exp = exp
    calc.init(exp)
    if (props.file?.trim()) {
      calc.add_defs(props.file);
    }
    data.steps = calc.history()
    data.error = ''
    data.defs = calc.get_defs()
  } catch (e) {
    console.warn(e)
    data.error = e
  }
}

onMounted(() => {
  resetExp(props.exp)
})

watch([props], ([cur]) => {
  console.log('watch!', cur)
  if (cur.exp !== data.last_exp) {
    resetExp(cur.exp)
  }
})

provide(replaceNameKey, (name, alpha, step_id) => {
  console.log('replace', step_id, name)
  calc.replace_def_alpha(step_id, name, alpha)
  data.steps = calc.history()
})
provide(betaReduceKey, (id, step_id) => {
  console.log('reduce', step_id, id)
  calc.beta_reduce(step_id, id)
  data.steps = calc.history()
})
provide(etaReduceKey, (id, step_id) => {
  console.log('reduce', step_id, id)
  calc.eta_reduce(step_id, id)
  data.steps = calc.history()
})

</script>

<script lang="ts">
import type { InjectionKey } from 'vue'
export const replaceNameKey = Symbol() as InjectionKey<(name: string, alpha_id: number, step_id: number) => void>;
export const betaReduceKey = Symbol() as InjectionKey<(redex_id: number, step_id: number) => void>;
export const etaReduceKey = Symbol() as InjectionKey<(redex_id: number, step_id: number) => void>;
</script>

<template>
  <pre v-if="data.error" class="error">{{ data.error }}</pre>
  <TransitionGroup v-else name="lams">
    <LambdaExp v-for="step, step_id in data.steps" :key="step.id" :decoration="{
      betaRedex: step.last_action?.BetaReduce,
      etaRedex: step.last_action?.EtaReduce,
      replacedName: step.last_action?.SubstUnbounded,
      names: [...data.defs.keys()],
      step_id,
    }" :exp="step.display_exp" />
  </TransitionGroup>
</template>