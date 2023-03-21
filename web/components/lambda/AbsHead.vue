<script setup lang="ts">
/// AbsHead 可以控制 abstraction 能否拖动

defineProps<{
  redexId?: number
  etaRedexId?: number
}>()

defineEmits<{
  (e: 'beta-reduce', id: number): void
  (e: 'eta-reduce', id: number): void
}>()
</script>

<template>
  <span v-if="redexId" class="lambda-abs-head-redex" @click="$emit('beta-reduce', redexId)"><slot></slot></span>
  <span v-else-if="etaRedexId" class="lambda-abs-head-redex" @click="$emit('eta-reduce', etaRedexId)"><slot></slot></span>
  <span v-else><slot></slot></span>
</template>