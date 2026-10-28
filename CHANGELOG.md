# Changelog

What changed in each release of voltik: the crates `voltik` and `voltik-cli`, `@mathlabs/voltik` on npm and `voltik` on PyPI, which are released together. The local site (`site/build.sh`, then open `site/changelog.html`) shows the same, with the circuits drawn live.

Each release has a headline (the line under its title), what changed, and the figures of [`corpus/`](corpus) that show it (`Figures:`), drawn by the site with today's voltik. `scripts/release.sh` will not release a version without its section here, and makes the GitHub release from it.

## 0.1.0 - 2026-10-28

The first components of CircuiTikZ, drawn as a library of tikra.

- Path-style components: `short`, `open`, `generic`, resistors (American and European), capacitors and cute inductors, placed halfway along `to[…]` and turned with the line.
- Labels and annotations (`l`, `l_`, `a`, `R=label`), straight, rotated or smart, and the siunitx shorthand `20<\ohm>`.
- Poles `o` and `*`, `mirror` and `invert`, `\ctikzset` and the class keys (`resistors/zigs`, `inductors/coils`, `scale`, `width`, `thickness`), the `american` and `european` styles.
- The same core in Rust, the terminal, JavaScript (WebAssembly) and Python, and a site with a playground, a gallery and this page.

Figures: `ladder-network` `labels-around` `european-styles` `mirror-invert`
