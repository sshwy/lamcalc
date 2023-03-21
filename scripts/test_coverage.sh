#!/bin/zsh

rm -rf *.profdata

OBJECTS=()
cover_test() {
  LLVM_PROFILE_FILE="$1.profraw" RUSTFLAGS="-C instrument-coverage" ${@:2} # execute command
  OUTPUT=$( for file in $( \
    RUSTFLAGS="-C instrument-coverage" \
      ${@:2} --no-run --message-format=json \
        | jq -r "select(.profile.test == true) | .filenames[]" \
        | grep -v dSYM - \
  ); do \
    printf "%s %s " -object $file; \
  done )
  OBJECTS+=${OUTPUT[@]}
}
cover_example() {
  # RUSTFLAGS="-C instrument-coverage" $@ # execute command
  OUTPUT=$( for file in $( \
    LLVM_PROFILE_FILE="$1.profraw" RUSTFLAGS="-C instrument-coverage" \
      ${@:2} --message-format=json \
        | jq -r "select(.executable != null) | .filenames[]" \
        | grep -v dSYM - \
  ); do \
    printf "%s %s " -object $file; \
  done )
  OBJECTS+=${OUTPUT[@]}
}
cover_example cover_2.profraw cargo run --example y_combinator
cover_example cover_3.profraw cargo run --example church_encoding
cover_example cover_4.profraw cargo run --example parser
cover_test    cover_1.profraw cargo test --tests
echo $OBJECTS

llvm-profdata merge -sparse *.profraw -o lamcalc.profdata
# llvm-profdata merge *.profraw -o lamcalc.profdata
rm -rf *.profraw

llvm-cov report ${=OBJECTS[@]} \
    --instr-profile=lamcalc.profdata \
    --use-color --ignore-filename-regex='/.cargo/registry'

# llvm-cov show \
#     --use-color --ignore-filename-regex='/.cargo/registry' \
#     --instr-profile=lamcalc.profdata \
#     ${=OBJECTS[@]} \
#     --show-instantiations --show-line-counts-or-regions \
#     --line-coverage-lt=2 \
#     --Xdemangler=rustfilt \
#     | less -R