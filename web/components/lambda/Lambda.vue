<script setup lang="ts">
import Exp from './Exp.vue';

defineProps<{
  exp: any
  lastRedex?: number
}>()

const emit = defineEmits<{
  (e: 'beta-reduce', id: number): void
}>()

const onReduce = (id: number) => {
  emit('beta-reduce', id)
}
</script>

<template>
  <div class="lambda-exp">
    <div class="lambda-inner">
      <Exp :last-redex="lastRedex" v-bind="exp" :bracket-level="0" @beta-reduce="onReduce" />
    </div>
  </div>
</template>