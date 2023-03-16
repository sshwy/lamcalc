<script setup lang="ts">
import { ref, watch } from 'vue';
import Exp from './Exp.vue';
import './style.css'

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