<script setup lang="ts">
import Ident from './Ident.vue';
import AbsHead from './AbsHead.vue';
import AbsWrapper from './AbsWrapper.vue'
import AppWrapper from './AppWrapper.vue';
import { computed, ref } from 'vue';

const props = defineProps<{
  bounded?: boolean
  redexTrigger?: () => void
  inAppFirst?: boolean
  inAppSecond?: boolean
  Abs?: [any, any]
  App?: [any, any]
  Var?: [string, number]
  bracketLevel: number
}>()

const nextLevel = props.bracketLevel + (props.bounded ? 1 : 0)
const displayPar = computed(() => props.bounded && !props.Var && !(props.App && props.inAppFirst))

const self = ref<HTMLElement>(null)
const param = ref<HTMLElement>(null)

const hightlightBetaRedex = computed(() => props.App && props.App[0].Abs ? (enable: boolean) => {
  (param.value as any).highlight(enable)
} : undefined)

// console.log(hightlightBetaRedex, props)

const classes = ref('')
const highlight = (enable: boolean) => {
  console.log('highlight', enable)
  console.log(self.value)

  let debounceTimer: number | null = null;
  const debounce = (callback: any, time: number) => {
    if (debounceTimer === null) {
      debounceTimer = setTimeout(callback(), 0);
    } else {
      clearTimeout(debounceTimer);
      debounceTimer = setTimeout(callback, time);
    }
  };

  const dragOver = () => {
    // console.log('dragOver!')
    debounce(() => {
      try {
        classes.value = "lambda-dragover"
      } catch (e) {
        console.warn(e)
      }
    }, 30)
  }
  const dragLeave = () => {
    debounce(() => {
      if (classes.value === 'lambda-dragover') classes.value = "lambda-highlight"
    }, 100)
  }
  if (enable) {
    classes.value = 'lambda-highlight'
    self.value.addEventListener('dragover', dragOver)
    self.value.addEventListener('dragleave', dragLeave)
  } else {
    self.value.removeEventListener('dragover', dragOver)
    self.value.removeEventListener('dragleave', dragLeave)
    classes.value = ''
  }
}

defineExpose({
  highlight
})

</script>

<template>
  <span ref="self" :class="['lambda', classes]">
    <span v-if="displayPar" :class="`lambda-bracket-${bracketLevel % 3}`">(</span>

    <AbsWrapper v-if="Abs" v-slot="slotProps" :redex-trigger="redexTrigger">
      <AbsHead v-bind="slotProps">
        <span class="lambda-lambda">λ</span>
        <Ident :ident="Abs[0][0]" :de="Abs[0][1]" />
      </AbsHead>
      <span class="lambda-dot">.</span>
      <Exp v-bind="Abs[1]" :bracket-level="nextLevel" />
    </AbsWrapper>
    <AppWrapper v-else-if="App">
      <Exp v-bind="App[0]" bounded in-app-first :bracket-level="bracketLevel" :redex-trigger="hightlightBetaRedex" />
      <span class="lambda-blank"> {{ " " }} </span>

      <!-- <template v-if="inAppFirst && App[1].Abs">
        <span :class="`lambda-bracket-${bracketLevel % 3}`">(</span>
        <Exp ref="param" v-bind="App[1]" in-app-second :bracket-level="bracketLevel + 1" />
        <span :class="`lambda-bracket-${bracketLevel % 3}`">)</span>
      </template> -->
      <Exp ref="param" v-bind="App[1]" bounded in-app-second :bracket-level="bracketLevel" />
    </AppWrapper>
    <span v-else-if="Var" class="lambda-var">
      <Ident :ident="Var[0]" :de="Var[1]" />
    </span>

    <span v-if="displayPar" :class="`lambda-bracket-${bracketLevel % 3}`">)</span>
  </span>
</template>

<style>
.lambda-lambda {
  color: var(--vp-c-lambda-lambda);
}

.lambda-dot {
  color: var(--vp-c-lambda-dot);
}

.lambda-bracket-0 {
  color: var(--vp-c-lambda-bracket-0)
}

.lambda-bracket-1 {
  color: var(--vp-c-lambda-bracket-1)
}

.lambda-bracket-2 {
  color: var(--vp-c-lambda-bracket-2)
}

.lambda-ref {
  color: var(--vp-c-lambda-ref)
}

.lambda {
  transition: .1s ease-in-out;
  transition-property: padding, border;
}

.lambda-highlight {
  border: 2px solid var(--vp-c-lambda-lambda);
  display: inline-block;
  padding: 2px 4px 0px;
  border-radius: 6px;
}

.lambda-dragover {
  background-color: var(--vp-c-lambda-hlbg);
  border: 2px solid var(--vp-c-lambda-lambda);
  display: inline-block;
  padding: 2px 4px 0px;
  border-radius: 6px;
}
</style>