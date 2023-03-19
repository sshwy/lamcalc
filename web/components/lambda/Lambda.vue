<script setup lang="ts">
import Exp from './Exp.vue';

defineProps<{
  exp: any
  decoration: {
    lastRedex?: number
    replacedName?: string
    names: string[]
    step_id: number,
  }
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
        <Exp :decoration="decoration" v-bind="exp" :bracket-level="0" @beta-reduce="onReduce" />
      </div>
    </div>
</template>