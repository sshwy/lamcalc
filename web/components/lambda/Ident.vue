<script setup lang="ts">
import { inject } from 'vue';
import { decorKey } from './LambdaExp.vue';

defineProps<{
  ident: string,
  de: number
  alpha: number,
}>()

const deco = inject(decorKey);
</script>

<template>
  <span v-if="alpha && deco.betaRedex?.alpha === alpha" class="lambda-ident-mark" :title="de.toString()">{{ ident
  }}</span>
  <span v-else-if="alpha && deco.etaRedex?.alpha === alpha" class="lambda-ident-mark" :title="de.toString()">{{ ident
  }}</span>
  <span v-else class="lambda-ident" :title="de.toString()">{{ ident }}</span>
</template>