<script setup lang="ts">
import Ident from './Ident.vue';
import AbsHead from './AbsHead.vue';
import AbsWrapper from './AbsWrapper.vue'
import { computed, inject, ref } from 'vue';
import { useDebounceFn } from '@vueuse/shared';
import { replaceNameKey } from '../LambdaInteractive.vue';

const props = defineProps<{
  parentheses: boolean
  bracketLevel: number
  inner: any
  redexTrigger?: () => void
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

const nextLevel = computed(() => props.bracketLevel + (props.parentheses ? 1 : 0))

const self = ref<HTMLElement | null>(null)
const param = ref<HTMLElement | null>(null)

const hightlightBetaRedex = computed(() => props.inner.App?.beta_redex ? (enable: boolean) => {
  (param.value as any).highlight(enable, props.inner.App.beta_redex)
} : undefined)

// console.log(props)

const onDropReduce = useDebounceFn((redex: number) => {
  console.log('drop!', redex)
  emit('beta-reduce', redex)
}, 50)

const classes = ref('')
const highlight = (enable: boolean, redex: number) => {
  // console.log('highlight', enable, 'redex:', redex)

  const onDragOver = (event: DragEvent) => {
    event.preventDefault() // prevent default to allow drop
  }
  const onDrop = (event: DragEvent) => {
    event.preventDefault();
    onDropReduce(redex)
  }
  if (!self.value) throw new Error('no self ref')

  if (enable) {
    classes.value = 'lambda-highlight'

    self.value.addEventListener('dragover', onDragOver)
    self.value.addEventListener('drop', onDrop, { once: true })
  } else {
    classes.value = ''
    self.value.removeEventListener('dragover', onDragOver)
    self.value.removeEventListener('drop', onDrop)
  }
}

defineExpose({
  highlight
})

const trigger = inject(replaceNameKey)

const onVarClick = (name: string) => {
  if (props.decoration.names.includes(name)) {
    if (trigger) trigger(name, props.decoration.step_id)
  }
}
</script>

<template>
  <span ref="self" :class="['lambda', classes]">
    <span v-if="parentheses" :class="`lambda-bracket-${bracketLevel % 3}`">(</span>

    <AbsWrapper v-if="inner.Abs" v-slot="slotProps" :redex-trigger="redexTrigger">
      <AbsHead v-bind="slotProps" :redex="!!redexTrigger">
        <span class="lambda-lambda">λ</span>
        <Ident :ident="inner.Abs.ident" :de="0" />
      </AbsHead>
      <span class="lambda-dot">.</span>
      <Exp :decoration="decoration" @beta-reduce="id => $emit('beta-reduce', id)" v-bind="inner.Abs.body"
        :bracket-level="nextLevel" />
    </AbsWrapper>
    <span v-else-if="inner.App"
      :class="['lambda-app', decoration.lastRedex && inner.App.beta_redex === decoration.lastRedex ? 'lambda-redex' : '']">
      <Exp class="lambda-app-func" :decoration="decoration" @beta-reduce="id => $emit('beta-reduce', id)"
        v-bind="inner.App.func" :bracket-level="nextLevel" :redex-trigger="hightlightBetaRedex" />
      <span class="lambda-blank"> {{ " " }} </span>
      <Exp class="lambda-app-body" :decoration="decoration" @beta-reduce="id => $emit('beta-reduce', id)" ref="param"
        v-bind="inner.App.body" :bracket-level="nextLevel" />
    </span>
    <span v-else-if="inner.Var" :class="[decoration.names.includes(inner.Var.ident) ?
      'lambda-const' : 'lambda-var']" @click="onVarClick(inner.Var.ident)">
      <Ident :ident="inner.Var.ident" :de="inner.Var.code" />
    </span>

    <span v-if="parentheses" :class="`lambda-bracket-${bracketLevel % 3}`">)</span>
  </span>
</template>