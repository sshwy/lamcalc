<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  redexTrigger?: (enable: boolean) => void
}>()

const self = ref<HTMLElement>(null)

const startDrag = () => {
  const trigger = props.redexTrigger
  if (trigger) trigger(true)
  document.addEventListener('dragend', () => {
    if (trigger) trigger(false)
    console.log('remove listener')
    self.value.removeEventListener('dragstart', startDrag);
  })
  document.addEventListener('mouseup', () => {
    if (trigger) trigger(false)
    console.log('remove listener')
    self.value.removeEventListener('dragstart', startDrag);
  })
}

const enableDrag = () => {
  console.log('enable')
  self.value.addEventListener('dragstart', startDrag);
  self.value.draggable = true
}
const disableDrag = () => {
  console.log('disable')
  self.value.draggable = false
}

</script>

<template>
  <span class="lambda-abs" ref="self">
    <slot :enableDrag="enableDrag" :disableDrag="disableDrag"></slot>
  </span>
</template>