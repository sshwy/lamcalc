<script setup lang="ts">
import { parse_exp } from 'lamcalc';
import { ref } from 'vue';
import ExpStatic from './ExpStatic.vue'

const props = defineProps<{
  block?: boolean
  exp: string
}>()

const exp = ref(parse_exp(props.exp)[0])

</script>
<template>
  <template v-if="block">
    <div class="lambda-exp-static-block">
      <div class="lambda-inner">
        <ExpStatic v-bind="exp" :bracket-level="0" />
      </div>
    </div>
  </template>
  <template v-else>
    <span class="lambda-inline">
      <ExpStatic v-bind="exp" :bracket-level="0" />
    </span>
  </template>
</template>