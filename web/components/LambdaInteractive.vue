<!-- 可以自己拖动进行化简的 Lambda 表达式. Custom Only! -->
<script setup lang="ts">
import { Calculator } from 'lamcalc';
import { onMounted, reactive, watch } from 'vue';
import { LambdaExp } from './lambda';

const props = defineProps<{
  exp: string
}>()

const data = reactive({
  last_exp: '',
  error: '',
  steps: [],
})

const calc = new Calculator()
const resetExp = (exp: string) => {
  try {
    data.last_exp = exp
    calc.init(exp)
    data.steps = calc.history()
    data.error = ''
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

</script>
<template>
  <pre v-if="data.error" class="error">{{ data.error }}</pre>
  <TransitionGroup v-else name="lams">
    <LambdaExp v-for="[exp, betaRedex, id], step_id in data.steps" :key="id" :last-redex="betaRedex" :exp="exp"
      @beta-reduce="id => onReduce(step_id, id)" />
  </TransitionGroup>
</template>