#!/bin/zsh
rm default_*.profraw

RUSTFLAGS="-C instrument-coverage" cargo test --tests --examples
llvm-profdata merge -sparse default_*.profraw -o lamcalc.profdata

OBJECTS=$( \
  for file in \
    $( \
      RUSTFLAGS="-C instrument-coverage" \
        cargo test --tests --no-run --message-format=json \
          | jq -r "select(.profile.test == true) | .filenames[]" \
          | grep -v dSYM - \
    ); \
  do \
    printf "%s %s " -object $file; \
  done \
)

llvm-cov report ${=OBJECTS[@]} \
    --instr-profile=lamcalc.profdata \
    --use-color --ignore-filename-regex='/.cargo/registry'

llvm-cov show \
    --use-color --ignore-filename-regex='/.cargo/registry' \
    --instr-profile=lamcalc.profdata \
    ${=OBJECTS[@]} \
    --show-instantiations --show-line-counts-or-regions \
    --line-coverage-lt=2 \
    --Xdemangler=rustfilt | less -R