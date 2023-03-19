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

const onReduce = (step: number, id: number) => {
  console.log('reduce', step, id)
  calc.beta_reduce(step, id)
  data.steps = calc.history()
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

provide(replaceNameKey, (name, step_id) => {
  console.log('replace', step_id, name)
  calc.replace_def_occurrance(step_id, name)
  data.steps = calc.history()
})

</script>

<script lang="ts">
import type { InjectionKey } from 'vue'
export const replaceNameKey = Symbol() as InjectionKey<(name: string, step_id: number) => void>;
</script>

<template>
  <pre v-if="data.error" class="error">{{ data.error }}</pre>
  <TransitionGroup v-else name="lams">
    <LambdaExp v-for="[exp, betaRedex, replacedName, id], step_id in data.steps" :key="id" :decoration="{
      lastRedex: betaRedex,
      replacedName,
      names: [...data.defs.keys()],
      step_id,
    }" :exp="exp" @beta-reduce="id => onReduce(step_id, id)" />
  </TransitionGroup>
</template>