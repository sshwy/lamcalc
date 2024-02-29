# λ-Calculus: 道生万物 (An Elegant Road to the Profound λ-Calculus)

一个介绍 λ-Calculus 的网站。

## Development

```
pnpm i
```

`ems_loader.mjs` 的作用是在 `vitepress` 构建的时候使用 esm 模式解析 `lamcalc` 模块（`wasm-pack` 没生成好）。
对此还需要在环境参数里开启实验性的 WASM。

WASM 模块构建：

```bash
wasm-pack build --features wasm
```

更新 `lamcalc` 可以执行 `yarn` 完成。
