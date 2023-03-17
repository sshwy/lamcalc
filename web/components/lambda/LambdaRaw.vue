<!-- 渲染原文的高亮 tokens -->
<script setup lang="ts">
import { parse_exp } from 'lamcalc';

const props = defineProps<{
  block?: boolean
  exp: string
}>()

const [_, tokens] = parse_exp(props.exp)

function render_tokens(tokens: { kind: string, data: string }[]) {
  let bracketLevel = 0;
  let res = tokens.map(tk => {
    switch (tk.kind) {
      case 'LamSym': return `<span class="lambda-lambda">λ</span>`;
      case 'DotSym': return `<span class="lambda-dot">.</span>`;
      case 'LPar': return `<span class="lambda-bracket-${(bracketLevel++) % 3}">(</span>`;
      case 'RPar': return `<span class="lambda-bracket-${(--bracketLevel) % 3}">)</span>`;
      case 'Ident': return `<span class="lambda-var">${tk.data}</span>`;
      default: return tk.data;
    }
  }).join("")
  return res
}
</script>

<template>
  <template v-if="block">
    <div class="lambda-exp-static-block">
      <div class="lambda-inner">
        <span class="lambda" v-html="render_tokens(tokens)"></span>
      </div>
    </div>
  </template>
  <template v-else>
    <span class="lambda-inline">
      <span class="lambda" v-html="render_tokens(tokens)"></span>
    </span>
  </template>
</template>