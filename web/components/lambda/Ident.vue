<script setup lang="ts">
defineProps<{
  ident: string,
  de: number
}>()
</script>
<template>
  <span class="lambda-ident" :title="de.toString()">{{ ident }}</span>
</template>
<style>
.lambda-ident {
  color: var(--vp-c-lambda-ident);
}
</style>