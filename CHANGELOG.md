# Changelog

What changed in each release of voltik: the crates `voltik` and `voltik-cli`, `@mathlabs/voltik` on npm and `voltik` on PyPI, which are released together. The local site (`site/build.sh`, then open `site/changelog.html`) shows the same, with the circuits drawn live.

Each release has a headline (the line under its title), what changed, and the figures of [`corpus/`](corpus) that show it (`Figures:`), drawn by the site with today's voltik. `scripts/release.sh` will not release a version without its section here, and makes the GitHub release from it.

## 0.4.0 - 2026-10-28

Components as nodes, with anchors of their own.

- Node-style components, each a node shape of its own (tikra's new library shapes): the ten grounds, `vcc` and `vee`, the terminals (`circ`, `ocirc`, `diamondpole`, `squarepole`… and `rectjoinfill`), the BNC and IEC connectors, the jump and plain crossings, and the arrows (`currarrow`, `inputarrow`, `trarrow`, `flowarrow`).
- Their anchors (`G.south`, `B.zero`, `S.socket center`), placing by them (`anchor=zero`), turning with the node, `fill` where fillable, and the picture's size as CircuiTikZ counts it.
- The classes `grounds`, `power supplies` and `connectors` (`…/scale`); every class of the manual has its settings, so `\ctikzset{diodes/scale=…}` no longer scales the picture.
- Path-style buses (`multiwire`, `bmultiwire`, `tmultiwire`), `crossing` and `iec connector`; named bipoles have the short anchors too (`R1.nw`).

Figures: `grounds-and-supplies` `terminals-and-connectors` `powered-divider`

## 0.3.0 - 2026-10-28

Voltages, currents and flows, as the manual draws them for every component so far.

- `v=`, `i=`, `f=` and their variants with a side (`^`, `_`) and a direction (`<`, `>`), on the lead before or after the component.
- Voltages in the four styles: European curved arrows, straight arrows, American signs and raised American signs, with `voltage shift`, `voltage/bump b`, `voltage/distance from node` and the American keys (`american plus`, `american label distance`…); an open circuit's voltage lies on the line.
- The `voltage dir` conventions (`noold`, the default, `EF`, `RP`, `old`), and the document's own `\usepackage[RPvoltages, american]{circuitikz}`.
- The anchors of named components' voltages, currents and flows (`R1-Vfrom`, `R1-Ipos`…), their labels' nodes (`R1voltage`, `R1label`), `\ctikzgetanchor`, `\ctikzgetdirection`, `no v symbols` and the `bipole … style` keys.
- Two-line labels `l2=… and …`, and siunitx in any label: `\SI`, `\qty`, `\si`, `\unit`, `\num`.
- Labels keep TikZ's inner sep, as CircuiTikZ's do, so pictures are their size.

Figures: `voltages-and-currents` `voltage-styles` `stacked-and-units`

## 0.2.0 - 2026-10-28

Every resistive bipole, capacitor and inductor of the manual, drawn as CircuiTikZ draws them.

- The resistive bipoles: the generic ones (crossed, slashed, sine, tunable, asymmetric), memristors, variable resistors, potentiometers with `wiper pos`, resistive sensors, light-dependent resistors, varistors, photoresistors and thermistors, American and European.
- The capacitors (curved, electrolytic, variable, sensing, piezoelectric, constant phase, ferroelectric) and the inductors in the three styles, cute, American and European, variable and sensing, and the cute choke.
- `fill=` on a `to` fills the components the manual makes fillable; `name=` gives a component its anchors as coordinates (`P.wiper`, `L1.midtap`, `L1.core west`).
- Poles `d`, and `bipole nodes={…}{…}` with circles, diamonds and squares; the class keys `height` and `modifier thickness`; the class keys in a `to[…]`, and styles defined with `\ctikzset`.
- Labels keep out of each component's own band, above and below: a potentiometer's wiper pushes the label up, not the annotation down.

Figures: `resistive-bipoles` `american-and-european-resistors` `capacitors` `inductors` `thermistor-bridge` `fills-and-poles` `class-settings`

## 0.1.0 - 2026-10-28

The first components of CircuiTikZ, drawn as a library of tikra.

- Path-style components: `short`, `open`, `generic`, resistors (American and European), capacitors and cute inductors, placed halfway along `to[…]` and turned with the line.
- Labels and annotations (`l`, `l_`, `a`, `R=label`), straight, rotated or smart, and the siunitx shorthand `20<\ohm>`.
- Poles `o` and `*`, `mirror` and `invert`, `\ctikzset` and the class keys (`resistors/zigs`, `inductors/coils`, `scale`, `width`, `thickness`), the `american` and `european` styles.
- The same core in Rust, the terminal, JavaScript (WebAssembly) and Python, and a site with a playground, a gallery and this page.

Figures: `ladder-network` `labels-around` `european-styles` `mirror-invert`
