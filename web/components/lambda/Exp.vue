<script setup lang="ts">
import Ident from './Ident.vue';
import AbsHead from './AbsHead.vue';
import AbsWrapper from './AbsWrapper.vue'
import { computed, ref } from 'vue';
import { useDebounceFn } from '@vueuse/shared';

const props = defineProps<{
  marked: boolean
  parentheses: boolean
  bracketLevel: number
  inner: any
  redexTrigger?: () => void
  lastRedex?: number
}>()

const emit = defineEmits<{
  (e: 'beta-reduce', id: number): void
}>()

const nextLevel = computed(() => props.bracketLevel + (props.parentheses ? 1 : 0))

const self = ref<HTMLElement>(null)
const param = ref<HTMLElement>(null)

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
      <Exp :last-redex="lastRedex" @beta-reduce="id => $emit('beta-reduce', id)" v-bind="inner.Abs.body"
        :bracket-level="nextLevel" />
    </AbsWrapper>
    <span v-else-if="inner.App" :class="['lambda-app', lastRedex && inner.App.beta_redex === lastRedex ? 'lambda-redex' : '']">
      <Exp :last-redex="lastRedex" @beta-reduce="id => $emit('beta-reduce', id)" v-bind="inner.App.func"
        :bracket-level="nextLevel" :redex-trigger="hightlightBetaRedex" />
      <span class="lambda-blank"> {{ " " }} </span>
      <Exp :last-redex="lastRedex" @beta-reduce="id => $emit('beta-reduce', id)" ref="param" v-bind="inner.App.body"
        :bracket-level="nextLevel" />
    </span>
    <span v-else-if="inner.Var" class="lambda-var">
      <Ident :ident="inner.Var.ident" :de="inner.Var.code" />
    </span>

    <span v-if="parentheses" :class="`lambda-bracket-${bracketLevel % 3}`">)</span>
  </span>
</template>