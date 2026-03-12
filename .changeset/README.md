# Changesets

This repository uses [Changesets](https://github.com/changesets/changesets) to stage npm version
updates for `@docxly/core-rs`.

Common contributor flow:

```bash
npx @changesets/cli add
```

After the changeset lands on `main`, the `Version Packages` workflow opens or updates a release PR
that:

- bumps `packages/npm-core-rs/package.json`
- syncs `packages/core-rs/Cargo.toml`
- refreshes `packages/npm-core-rs/package-lock.json`

The existing tag-based `Release` workflow still handles the actual npm publish for `v*.*.*` tags.
