<script setup lang="ts">
import { Calculator } from 'lamcalc';
import { ref } from 'vue';
import ExpStatic from './ExpStatic.vue'
import './style.css'

const props = defineProps<{
  block?: boolean
  exp: string
}>()

const calc = new Calculator()
const exp = ref(null)
const error = ref(null)

try {
  exp.value = calc.init(props.exp)
} catch (e) {
  error.value = e
}

</script>
<template>
  <template v-if="block">
    <div class="lambda-exp-static-block">
      <div class="lambda-inner">
        <ExpStatic v-if="exp" v-bind="exp" />
        <span v-else>{{ error }}</span>
      </div>
    </div>
  </template>
  <template v-else>
    <span class="lambda-inline">
      <ExpStatic v-if="exp" v-bind="exp" />
      <span v-else>{{ error }}</span>
    </span>
  </template>
</template>