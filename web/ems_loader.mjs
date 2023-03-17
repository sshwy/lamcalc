//! ref: https://nodejs.org/api/esm.html#loaders

// for files in the "lamcalc" module, load them in 'module' format.
export async function load(url, context, nextLoad) {
  if (/node_modules\/lamcalc\/.*.js$/.test(url) && context.format != 'module') {

    const ctxt = Object.assign({}, context);
    ctxt.format = 'module'
    console.log(url, ctxt.format)

    return nextLoad(url, ctxt);
  }

  // Defer to the next hook in the chain.
  return nextLoad(url);
}

export async function resolve(specifier, context, nextResolve) {
  if (specifier === 'lamcalc') { // in case nodejs can't find lamcalc module in vitepress's app.js
    return {
      format: 'module',
      shortCircuit: true,
      url: `file://${process.cwd()}/node_modules/lamcalc/lamcalc.js`,
    }
  }

  // Defer to the next hook in the chain, which would be the
  // Node.js default resolve if this is the last user-specified loader.
  return nextResolve(specifier);
}