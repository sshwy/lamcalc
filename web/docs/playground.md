---
outline: deep
prev: false
next: false
---

# 在线演绎 λ-Calculus

不妨试试自己写一些 Lambda 表达式！下面是预定义的常用的表达式：

<script setup>
import LambdaPlayground from '../components/LambdaPlayground.vue'
import { data } from '../components/playground.data'
</script>

<LambdaDefs :file="data" />

尝试在下方输入框中写一些 Lambda 表达式。点击下划线部分可以化简表达式，可以使用预定义的表达式，点击预定义表达式的别名可以将其展开。
可以猜猜看这个表达式在计算什么～

<ClientOnly>
  <LambdaPlayground />
</ClientOnly>