<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  redexTrigger?: (enable: boolean) => void
}>()

const self = ref<HTMLElement | null>(null)

const startDrag = () => {
  const trigger = props.redexTrigger
  if (trigger) trigger(true)

  const disable = () => {
    if (trigger) trigger(false)

    document.removeEventListener('dragend', disable)
    document.removeEventListener('mouseup', disable);
  }

  document.addEventListener('dragend', disable)
  document.addEventListener('mouseup', disable);
}

const enableDrag = () => {
  console.debug('enable')
  if (!self.value) throw new Error("no self ref")
  self.value.addEventListener('dragstart', startDrag);
  self.value.draggable = true
}
const disableDrag = () => {
  console.debug('disable')
  if (!self.value) throw new Error("no self ref")
  self.value.removeEventListener('dragstart', startDrag);
  self.value.draggable = false
}

</script>

<template>
  <span class="lambda-abs" ref="self">
    <slot :enableDrag="enableDrag" :disableDrag="disableDrag"></slot>
  </span>
</template>