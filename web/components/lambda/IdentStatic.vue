<script setup lang="ts">
defineProps<{
  ident: string,
  de: number
  alpha: number,
}>()

</script>

<!-- :class="deco.lastRedex && inner.Var.alpha_id === deco.lastRedex[1] ? 'lambda-ident-mark' : ''" -->
<template>
  <span class="lambda-ident" :title="de.toString()">{{ ident }}</span>
</template>