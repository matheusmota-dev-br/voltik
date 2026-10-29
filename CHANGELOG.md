# Changelog

What changed in each release of voltik: the crates `voltik` and `voltik-cli`, `@mathlabs/voltik` on npm and `voltik` on PyPI, which are released together. The local site (`site/build.sh`, then open `site/changelog.html`) shows the same, with the circuits drawn live.

Each release has a headline (the line under its title), what changed, and the figures of [`corpus/`](corpus) that show it (`Figures:`), drawn by the site with today's voltik. `scripts/release.sh` will not release a version without its section here, and makes the GitHub release from it.

## 0.11.0 - 2026-10-29

Transformers and motors, the sizes of components, and bipoles as nodes.

- The double bipoles as nodes: `transformer` and `transformer core`, their coils by the `inductor` setting, `gyrator` and `double bipole`, each also `quadpoles style=inline`, with their anchors (`A1`…`B2`, the dots).
- The electro-mechanical device `elmech`, and `Telmech` on a path.
- The sizes of amplifiers and double bipoles (`tripoles/op amp/width`, `port width`, `input height`…, `quadpoles/transformer/width`, `height`, `inner`), drawn in between the measured ones.
- A class's `fill` (`\ctikzset{amplifiers/fill=cyan}`), `component text=left`, `left text distance` and `en amp text`.
- Every bipole as a node by its shape name (`node[resistorshape]`, `fulllediodeshape`…), and inside a double bipole (`double bipole L`, `R`, `R invert`, `every double bipole L`).
- A transformer's coils as nodes (`T1-L1.a`), drawn by `transformer L1` and `L2`.
- siunitx in any text, not only in labels (`node {\SI{12}{\volt}}`), and `\scalebox`; `transform shape` scales components on paths too.

Figures: `transformers` `sizes-and-shapes`

## 0.10.0 - 2026-10-29

Amplifiers.

- The amplifiers of the manual as nodes: `op amp`, `fd op amp`, `en amp`, `gm amp`, the instrumentation amplifiers, the plain amplifiers and `buffer`, with their anchors and their `+` and `-`, in the `amplifiers` class.
- `noinv input up`, `noinv input down`, `noinv output up` and `noinv output down`.

Figures: `amplifiers` `non-inverting-amplifier`

## 0.9.0 - 2026-10-28

Electronic tubes.

- The tubes of the manual as nodes: `diodetube`, `triode`, `tetrode`, `pentode`, `pentode suppressor to cathode`, `matube`, `magnetron` and `dynode`, with their anchors, in the `tubes` class.
- Their options `filament`, `nocathode`, `fullcathode`, `nixieanode`, `anodedot` and `nogrid`.

Figures: `tubes`

## 0.8.0 - 2026-10-28

Transistors.

- Every transistor of the manual as a node: bipolar (`npn`, `pnp`, multi-collector `bjtnpn`…), IGBTs, MOSFETs (`nmos`, `pmos`, depletion), FETs (`nfet`, `nigfete`…), HEMTs, JFETs, UJTs, ISFET and graphene FETs, with all their anchors, in the `transistors` class.
- Their options in the combinations the manual draws: `bodydiode`, `tr circle`, `bulk`, `fdsoi`, `doublegate`, `solderdot`, `schottky base`, `photo`, `nobase`, `arrowmos`, `emptycircle`, `nocircle`, `collectors`, `emitters`.
- Transistors on a path (`to[Tnmos=$M_1$, n=M]`), turned round by `invert`.
- Depletion channels dark unless filled; the text right of the transistor, not counting in the picture's size, as in CircuiTikZ.

Figures: `transistors` `common-emitter`

## 0.7.0 - 2026-10-28

Switches, buttons and jumpers.

- Every path-style switch of the manual: traditional (`spst`, `nos`, `ncs`, `cnos`…), push buttons, the toggle switch, the reed switch, the cute switches, the inline proximeter, the jumpers (`three-pins jumper` with `tjumper connections`) and the solder jumpers, with their anchors; their poles white unless filled.
- The node-style `spdt`, `cute spdt up`, `cute spdt mid`, `cute spdt down` (and their `… arrow`) and `proximeter`, with their anchors, in the `switches` class.
- `n=…`, the short form of a component's `name`.

Figures: `switches`

## 0.6.0 - 2026-10-28

Diodes and the thyristors.

- Every diode of the manual: plain, Schottky, Zener, ZZener, tunnel, photodiode, LED, laser, varcap, TVS, Shockley and bidirectional, empty, full and stroke (`D`, `D*`, `D-`…); the short keys follow the `diode` setting, the styles `empty diodes`, `full diodes`, `stroke diodes` and the package options `emptydiode`, `fulldiode`, `strokediode`.
- The thyristor-like tripoles (`Tr`, `Ty`, `PUT`, `GTO`, `GTOb`, `aGTOb`, `IGCT`), legacy or `thyristor style=compact`, with their gate and anode and cathode anchors.
- `led arrows from cathode`, `pd arrows to cathode` and `diode straight whiskers`, and their way back.
- Their voltages as CircuiTikZ draws them, on each side: a voltage above an LED clears its arrows.

Figures: `diodes` `rectifier-and-zener`

## 0.5.0 - 2026-10-28

Sources, instruments and the other bipoles.

- Every source of the manual: independent and controlled, voltage and current, American, European and cute (`american currents`, `european currents` and their package options), sinusoidal, square, triangle, noise, DC, the `oosource`s, `nullator` and `norator`; the batteries and solar cells.
- Their voltages and currents as CircuiTikZ draws a source's: `V=…` and `I<=…` are the voltage and the current, a voltage source's voltage is a straight arrow across it, a current source's turns the other way; `bipole/is voltage`, `bipole/is current` and `bipole/override source vif`.
- The instruments (`rmeter`, `smeter`, `ammeter`, `voltmeter`, `oscope`, the probes, `iloop`…) with `t=…`, the mechanical analogies (`spring`, `damper`, `mass`, `inerter`, `viscoe`), and the other bipoles: fuses, lamps, bulbs, spark gaps, loudspeaker, microphones, buzzers, `squid`, barriers, `relais`, `thermocouple`, surge arresters; the `dc symbol` and `ac symbol` nodes.
- A component's built-in text stays upright and in its middle when the component turns.

Figures: `sources` `instruments-and-more` `source-and-load`

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
