#!/usr/bin/env sh
# Path-scoped pre-commit hook (Husky: .husky/pre-commit, or .git/hooks/pre-commit).
# Each check runs only when files that affect it are staged, so the hook stays fast and
# nobody reaches for --no-verify. Include the hook and justfile in every pattern so editing
# the tooling re-runs it. Edit the patterns and commands per project.

staged() { git diff --cached --name-only | grep -Eq "$1"; }

if staged '^(\.husky/pre-commit|justfile|backend/(Cargo\.(toml|lock)|build\.rs|rust-toolchain.*|\.cargo/|\.sqlx/|migrations/|src/|tests/))'; then
    echo "backend: fmt + clippy"
    just fmt-check || { echo "  rustfmt failed. Run: just fmt"; exit 1; }
    just lint      || { echo "  clippy failed. Run: just lint"; exit 1; }
fi

if staged '^(\.husky/pre-commit|justfile|eslint\.config\.js|.*\.(js|cjs|mjs|ts|cts|mts|svelte))$'; then
    echo "frontend: eslint"
    just lint-js || { echo "  eslint failed. Run: just lint-js"; exit 1; }
fi

# Generated files: the generator gets a --check mode that renders to a temp dir and diffs.
if staged '^(\.husky/pre-commit|k8s/scripts/gen-policies\.sh|k8s/.*/policies/)'; then
    echo "k8s: rendered policies in sync"
    bash k8s/scripts/gen-policies.sh --check || { echo "  out of date. Run: just gen-policies"; exit 1; }
fi
