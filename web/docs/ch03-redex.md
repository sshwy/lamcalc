# 等价与规约

要理解一段叙述，就得先给出这段叙述，否则上下文的指向性  就不够清晰。因此我不得不在读者尚未理解他们之前，先把他们拿出来说一遍，尽管这样的叙述读了近乎没读。

## 抽象与应用

“抽象” 和 “应用” 规则在上一章已有所叙述，对此我们展开来说。

**抽象（abstraction）**：对于形如 l`\x. e` 的 {l} 表达式，其中 l`e` 是任意 {l}，l`x` 是符号。将 ```{l}``` 和 ```.``` 之间的符号称作**捕获变量**， 将 ```.``` 之后的表达式 l`e` 称作**子表达式**。将子表达式中与捕获变量相同的符号 “捕获”，并全部与该变量绑定到一起，这个过程被称作抽象。

**应用（application）**：将两个 {l} 并排放置，以空格隔开，形如 l`f g`，此时我们称 l`f` 和  l`g` 都是该应用的**子表达式**。此时我们将整个表达式称作 l`f` 应用于 l`g`。

在这里，“抽象” 和 “应用” 是 {l} 表达式的形式，不是动词。

来看看这两个概念将会引入对 {l} 怎样的变化方式吧。

## 第一规约

对于一个 “抽象” 表达式 l`\x. e`，我们可以将之应用于任意一个表达式 l`y`。这会将 l`e` 中所有被 l`x` 捕获的符号 `x` 替换为表达式 l`y`，然后去掉捕获变量，只保留子表达式，得到最终的结果。

举个例子：对于表达式 l`(\x. x \x. x) \y. y`，我们可以将 l`\x. x \x. x` 应用于 l`\y. y`，得到 l`(\y. y) \x. x`。

可以发现这个结果本身也是一个 “应用”，并且这个应用中的第一个子表达式是一个 “抽象”。于是我们可以再次规约，最终得到 l`\x. x`。

请读者自己试试化简这个表达式：鼠标点击下划线处的表达式，观察化简后的结果。

<ClientOnly>
  <LambdaInteractive exp="(\x. x \x. x) \y. y" />
</ClientOnly>

::: tip
上述表达式的嵌套关系可以更清晰地描述为 <LambdaRaw exp="(\x.(x (\x. x))) (\y. y)" />。
其中第一个 l`x` 只捕获了第二个 l`x`，而第四个 l`x` 则被第三个 l`x` 捕获。
因此前两个 l`x` 和后两个 l`x` 不会同步变化。

换句话说，上述表达式等价于 <LambdaRaw exp="(\x.(x (\z. z))) (\y. y)" />。这样不改变表达式等价性的符号替换被称作 α 规约（也称 α-renaming）。不妨试试化简下面这个表达式：

<ClientOnly>
  <LambdaInteractive exp="(\x. x \z. z) \y. y" />
</ClientOnly>

如果想尝试更多 {l} 的化简，请移步[在线演绎 λ-Calculus](/playground)。
:::

这样的变化法则有一个玄奥的名字：**β 规约**。这种规约其实在初中数学中已有所体现。l`(\x. x) y` 好比将 $y$ 带入 $f(x) = x$，最终得到 $y$。

**β 规约**的描述方式有很多，以 l`(\x. e) y` 为例，我们可以称这个规约的过程为

- 将 l`\x. e` **应用**于表达式 l`y`；
- 将 l`y` **带入**函数 l`\x. e`；
- l`\x. e` 的捕获变量 l`x` **接受**一个表达式 l`y`。

在 {L} 中，函数是最本原的概念，也是表达所有事物的载体。因此函数的表达式本身也可以是函数（抽象），或者是一个尚未化简的函数的应用。某种意义上，“应用” 表达式也可以理解为是即将执行规约的 {l} 表达式，只不过我们不知道它第一个子表达式的函数是什么，所以暂时下不了手。

## 抽象：自由亦或束缚

什么是抽象？其实可以从它的字面意思下手，就是把一个东西从另一个东西里抽出来。

“道可道非常道”。如果我们将其中的道抽象出来，记作一个符号 l`x`，写成 {l} 就是 l`\x. x 可 x 非 常 x`。
于是我们有了一个自然的想法（尝试化简下面这个表达式）：

<ClientOnly>
  <LambdaInteractive exp="(\x. x 可 x 非 常 x) 名" />
</ClientOnly>

再比如说：

<ClientOnly>
  <LambdaInteractive exp="(\x. \y. \z. \w. x y z w w y z x) 色 即 是 空" />
</ClientOnly>

这些例子在语文上可以称作 “固定句式”，而在 {L} 中，它们便叫做抽象。

当我们将表达式中相同的部分使用一个变量捕获时，它们就具有变化的可能，于是我们就可以有很多相同 “句式” 的表达式；
但是从另一个角度，这些变量又只能同步变化，永远保持相同，绑定在了一起。
其实对于表达式 l`x x x x x` 来说，5 个 l`x` 虽然符号相同，但因为他们没有被任何捕获变量绑定，因此实际上他们是互不相同的。因此抽象即可以是一种自由，也可能是某种束缚。但需要说明，**在正规的 {l} 表达式中，不会允许自由变量的存在**。我们所举的例子中的自由变量均可以当作某个未知的函数。

## 以函数为第一要义

避免自由变量的另一个原因是：{L} 中，函数为第一要义（first class citizen）。

试想，各位在接触数学的初期，遇到的是各种数、四则运算，进位借位法则等等。而随着学习的深入，我们认识了各种抽象的符号和算子，推理证明相关的逻辑等等。

**{L} 中，函数是一切的起点**。

我们认识的第一个函数：

dL`I = \x. x`

被称作单位函数（identity function）。因为它应用到任何表达式上都会得到那个表达式本身（**尝试点击 `I` 来将其替换为单位函数的表达式**）：

```lambda-interactive
I = \x. x
K = \x. \y. x
---
I z
```

和 l`I` 一样的基本函数还有常值函数（constant function），也称 K 组合子（K conbinator）：

dL`K = \x. \y. x`

这个函数比 l`I` 多了一个 l`y`，它的作用是接受一个表达式，然后把它丢掉，只返回 l`x` 作为结果。
结合我们刚学的 l`I` 函数举个例子：设 

dL`f = K I`

表示始终返回 l`I` 的常函数，那么我们将某个任意的表达式 l`z` 带入 l`f`，将会得到什么呢（尝试化简以下表达式）：

```lambda-interactive
I = \x. x
K = \x. \y. x
f = K I
---
f z
```

::: tip
不要忘了，表达式的 “应用” 是左结合，因此 l`K I z` 等价于 <LambdaRaw exp="(K I) z" />。
:::

最终的表达式中没有 l`z`，只有 l`\x.x`。这说明不论 l`z` 是什么，都会被丢掉，得到的永远是 l`I`。

## 结束也是开始

我想，演算的部分已经告一段落，但我们所见识的 {L} 仍只是冰山一角。请大家放心，梦还很长。函数何为第一要义？道生万物于何处？从下一章开始，从 {l} 与自然数的关系出发，我们再来细说。