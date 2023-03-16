<script setup lang="ts">
import { ref, watch } from 'vue';
import Exp from './Exp.vue';

const props = defineProps<{
  exp: any
}>()

const lastBetaReduce = ref(0)
watch([props], () => {
  console.log('props changed! reset last beta reduce')
  lastBetaReduce.value = 0
})

const emit = defineEmits<{
  (e: 'beta-reduce', id: number): void
}>()

const onReduce = (id: number) => {
  if (id != lastBetaReduce.value) {
    lastBetaReduce.value = id
    emit('beta-reduce', id)
  }
}
</script>

<template>
  <div class="lambda-exp">
    <div class="lambda-inner">
      <Exp v-bind="exp" :bracket-level="0" @beta-reduce="onReduce" />
    </div>
  </div>
</template>

<style>
:root {
  --vp-c-lambda-ident: #001080;
  --vp-c-lambda-lambda: #af00db;
  --vp-c-lambda-dot: #000000;
  --vp-c-lambda-bracket-0: #0431fa;
  --vp-c-lambda-bracket-1: #319331;
  --vp-c-lambda-bracket-2: #7b3814;
  --vp-c-lambda-ref: #0070c1;
  --vp-c-lambda-hlbg: #af00db29;
}

:root.dark {
  --vp-c-lambda-ident: #9cdcfe;
  --vp-c-lambda-lambda: #c586c0;
  --vp-c-lambda-dot: #d4d4d4;
  --vp-c-lambda-bracket-0: #ffd700;
  --vp-c-lambda-bracket-1: #da70d6;
  --vp-c-lambda-bracket-2: #179fff;
  --vp-c-lambda-ref: #4fc1ff;
}

.lambda-exp {
  font-family: var(--vp-font-family-mono);
  padding: 8px 16px;
  font-size: 20px;
}

.lambda-inner {
  width: fit-content;
  margin: 0 auto;
}
</style>