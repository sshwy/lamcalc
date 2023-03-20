<script lang="ts">
import { InjectionKey, provide, Ref, ref, watch } from 'vue'
export type Decoration = {
  lastRedex?: [number, number]
  replacedName?: string
  names: string[]
  step_id: number,
}
export const decorKey = Symbol() as InjectionKey<Ref<Decoration>>;
</script>

<script setup lang="ts">
import Exp from './Exp.vue';

const props = defineProps<{
  exp: any
  decoration: Decoration
}>()

const deco = ref(props.decoration)
watch([props], ([cur]) => {
  deco.value = cur.decoration
})
provide(decorKey, deco)
</script>


<template>
  <div class="lambda-exp">
    <div class="lambda-inner">
      <Exp v-bind="exp" :bracket-level="0" />
    </div>
  </div>
</template>