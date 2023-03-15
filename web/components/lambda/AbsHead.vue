<script setup lang="ts">
/// AbsHead 可以控制 abstraction 能否拖动
import { onBeforeUnmount, onMounted, ref } from 'vue';
const self = ref<HTMLElement>(null)

const props = defineProps<{
  enableDrag: () => void
  disableDrag: () => void
}>()

const enableDragTrigger = () => {
  props.enableDrag()
  document.addEventListener('mouseup', disableDragTrigger)
  document.addEventListener('dragend', disableDragTrigger)
}
const disableDragTrigger = () => {
  props.disableDrag()
  document.removeEventListener('mouseup', disableDragTrigger)
  document.removeEventListener('dragend', disableDragTrigger)
}

onMounted(() => {
  self.value.addEventListener('mousedown', enableDragTrigger)
})
onBeforeUnmount(() => {
  self.value.removeEventListener('mousedown', enableDragTrigger)
})
</script>

<template>
  <span class="lambda-abs-head" ref="self"><slot></slot></span>
</template>

<style>
.lambda-abs-head:hover {
  cursor: move;
  text-decoration: underline;
}
</style>