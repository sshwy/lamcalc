<script setup lang="ts">
import { LambdaExp } from './lambda';
import { ref } from 'vue';

// 不能直接 import 函数，不然 wasm 初始化的部分会被 tree-shake 删掉
import * as lamcalc from 'lamcalc'
const { parse_exp } = lamcalc;

const str = ref(`(\\x.\\y.x y x) (\\x.\\y.x)`)
// const str = ref('\\f. (\\x. f (x x)) \\x. f (x x)');
const error = ref('');
const data = ref(parse_exp(str.value));

// const inputArea = ref<HTMLElement>(null)
// const current = () => inputArea.value.textContent

const onInput = (event: Event) => {
  try {
    data.value = parse_exp(str.value)
    error.value = ''
  } catch (e) {
    error.value = e
    data.value = { exp: {}, tokens: [] }
  }
}

/*
const render_highlight = (tokens: { kind: string, data: string }[], raw: string) => {
  let rendered = ''
  let res = tokens.map(tk => {
    switch (tk.kind) {
      case 'LamSym': return rendered += "\\", `<span class="lambda-lambda">\\</span>`;
      case 'DotSym': return rendered += ".", `<span class="lambda-dot">.</span>`;
      case 'LPar': return rendered += "(", `<span class="lambda-dot">(</span>`;
      case 'RPar': return rendered += ")", `<span class="lambda-dot">)</span>`;
      case 'Ident': return rendered += tk.data, `<span class="lambda-var">${tk.data}</span>`;
      default: return rendered += tk.data, tk.data;
    }
  }).join("")
  if (rendered.length > raw.length) throw new Error("invaid rendered: " + rendered + ", " + raw)
  return res + raw.slice(rendered.length)
}
onMounted(() => {
  let last = ''
  const update = (s: string) => {
    const select = window.getSelection()
    let offset = null
    if (!select.isCollapsed) {
      console.warn(select)
    } else {
      offset = select.focusOffset;
      let u = select.focusNode
      while (!u.isSameNode(inputArea.value)) {
        for (let v = u.previousSibling; v; v = v.previousSibling) {
          offset += v.textContent.length
        }
        u = u.parentNode
      }
    }
    console.log('offset:', offset)
    if (last != s) {
      last = s;
      inputArea.value.innerHTML = render_highlight(parse_exp(s).tokens, s)
    }
    if (offset !== null) {
      let u: Node = inputArea.value
      while (offset > 0) {
        if (u.nodeType === Node.TEXT_NODE && offset <= u.textContent.length) {
          select.collapse(u, offset)
          break;
        }
        let ok = false;
        for (let v = u.firstChild; v; v = v.nextSibling) {
          if (v.textContent.length <= offset) {
            offset -= v.textContent.length
            if (offset == 0) {
              select.collapse(v, v.nodeType === Node.TEXT_NODE ? v.textContent.length : v.childNodes.length)
              ok = true;
              break;
            }
          } else {
            u = v;
            ok = true;
            break;
          }
        }
        if (!ok) throw new Error("not ok")
        console.log("gg")
      }
    }
  }

  try {
    update(current())
  } catch (e) {
    console.warn(e)
  }
  // listen mutation
  const observer = new MutationObserver((mutationList, observer) => {
    try {
      update(current())
    } catch (e) {
      console.warn(e)
    }
    console.log(inputArea.value.textContent, mutationList)
  });
  observer.observe(inputArea.value, { attributes: true, childList: true, subtree: true, characterData: true })
  // listem paste
  inputArea.value.addEventListener('paste', event => {
    event.preventDefault();
    update(event.clipboardData.getData('text'))
  })
})
*/
</script>

<template>
  <div class="input-wrapper">
    <input type="text" v-model="str" placeholder="enter your lambda" @input="onInput" />
    <!-- <div class="input-area" contenteditable="true" ref="inputArea"> </div> -->
  </div>
  <pre v-if="error" class="error">{{ error }}</pre>
  <LambdaExp :exp="data?.exp" />
</template>

<style>
.input-wrapper {
  border: 1px solid var(--vp-c-bg-soft-mute);
  background-color: var(--vp-c-bg-soft);
  padding: 8px 16px;
  margin: 16px 0;
  border-radius: 4px;
  transition: all .2s ease-in-out;
}
.input-wrapper:hover {
  border: 1px solid var(--vp-c-brand);
}

.input-wrapper input {
  width: 100%;
  font-size: 16px;
  font-family: 'Courier New', Courier, monospace;
}

.input-wrapper .input-area {
  font-size: 16px;
  font-family: 'Courier New', Courier, monospace;
  min-height: 16px;
}
</style>