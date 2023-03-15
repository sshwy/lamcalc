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