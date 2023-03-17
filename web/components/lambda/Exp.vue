<script setup lang="ts">
import Ident from './Ident.vue';
import AbsHead from './AbsHead.vue';
import AbsWrapper from './AbsWrapper.vue'
import { computed, ref } from 'vue';

const props = defineProps<{
  marked: boolean
  parentheses: boolean
  bracketLevel: number
  inner: any
  redexTrigger?: () => void
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

const classes = ref('')
const highlight = (enable: boolean, redex: number) => {
  console.log('highlight', enable, 'redex:', redex)

  self.value.addEventListener('dragover', event => {
    event.preventDefault() // prevent default to allow drop
  })

  self.value.addEventListener('drop', event => {
    event.preventDefault();
    // console.log(event.target)
    emit('beta-reduce', redex)
  }, { once: true })


  if (enable) {
    classes.value = 'lambda-highlight'
  } else {
    classes.value = ''
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
      <Exp @beta-reduce="id => $emit('beta-reduce', id)" v-bind="inner.Abs.body" :bracket-level="nextLevel" />
    </AbsWrapper>
    <span v-else-if="inner.App" class="lambda-app">
      <Exp @beta-reduce="id => $emit('beta-reduce', id)" v-bind="inner.App.func" :bracket-level="nextLevel" :redex-trigger="hightlightBetaRedex" />
      <span class="lambda-blank"> {{ " " }} </span>
      <Exp @beta-reduce="id => $emit('beta-reduce', id)" ref="param" v-bind="inner.App.body" :bracket-level="nextLevel" />
    </span>
    <span v-else-if="inner.Var" class="lambda-var">
      <Ident :ident="inner.Var.ident" :de="inner.Var.code" />
    </span>

    <span v-if="parentheses" :class="`lambda-bracket-${bracketLevel % 3}`">)</span>
  </span>
</template>