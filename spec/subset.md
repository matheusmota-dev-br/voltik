# CircuiTikZ subset

What voltik reads and how it draws it. The specification is the CircuiTikZ manual, version 1.8.7; what the manual leaves open is measured on the drawings CircuiTikZ itself makes (`tools/oracle/`). Anything outside this list becomes a warning and the rest of the circuit is still drawn.

## Model

voltik is a library of tikra (`tikra::extension`): the figure is TikZ, read and drawn by tikra, and the CircuiTikZ keys are added to it. A `circuitikz` environment is a `tikzpicture`; the components are keys of `to[…]`, so they work in any `tikzpicture` too.

## Path-style components

`\draw (A) to[name=label, options] (B);` places the component halfway between `A` and `B`, turned along the line, and draws the leads from `A` to it and from it to `B`.

- **Size:** every length is a fraction of `bipoles/length` (1.4 cm): a component has a width (where the leads stop), a height, and how far it reaches above and below the line, its north and south (the band that labels keep out of; a variable resistor's arrow widens it). The class keys change them: `scale` scales the whole component; `width` sets the width of all the class (the piezoelectric element and the constant phase element stay twice as wide as the other capacitors); `height` sets the height of all the class. The drawing stretches with the width and the height; arrow heads and text do not.
- **Strokes:** the outline is `bipoles/thickness` (2) times the line width of the path, or its class's `thickness`; the marks that tell the kind of a component (a sensor's bar, a variable component's arrow) are that times the class's `modifier thickness` (1); the leads keep the path's own width. Zig-zags and coils start and end half a thick stroke beyond the leads, so the joint is covered.
- **Fill:** the `fill` of the `to` (or of its path) fills the components the manual marks as fillable: the box of the generic bipoles, the space between a capacitor's plates. The others are not filled; the solid parts (the dark end of `ageneric`, the European inductor) take the color of the stroke.
- **Coordinates and transforms:** the ends are read as TikZ reads them (named nodes give their center); the picture's `scale` moves the ends but does not resize the components, as with nodes.

The shapes are measured on what CircuiTikZ draws (`tools/oracle/`), at their default size.

| Class | Keys | Components |
|---|---|---|
| — | `short`, `open` | wire; open circuit (nothing but its poles and labels) |
| resistors | `generic`, `xgeneric`, `sgeneric`, `singeneric`, `tgeneric`, `ageneric`, `memristor` (`Mr`) | box 0.8 × 0.3, crossed, slashed, with a sine, tunable, asymmetric, memristor |
| resistors | `R` (`resistor`), `vR`, `pR` (`potentiometer`), `sR`, `ldR` | American or European by the `resistor` setting: zig-zag of `resistors/zigs` teeth (3) or box; variable, potentiometer (the wiper at `wiper pos`, 0.5), resistive sensor (the European one is the thermistor), light dependent |
| resistors | `american resistor`, `european resistor`, `variable american resistor`, `variable european resistor`, `american potentiometer`, `european potentiometer`, `american resistive sensor`, `european resistive sensor`, `american light dependent resistor`, `european light dependent resistor` | the same, whatever the setting |
| resistors | `varistor`, `mov`, `phR` (`photoresistor`), `thR` (`thermistor`), `thRp` (`thermistor ptc`), `thRn` (`thermistor ntc`) | varistors, photoresistor, thermistors |
| capacitors | `C` (`capacitor`), `cC` (`curved capacitor`), `eC` (`elko`, `ecapacitor`), `vC` (`variable capacitor`), `sC` (`capacitive sensor`), `PZ` (`piezoelectric`), `cpe`, `feC` (`ferrocap`) | plates 0.2 apart and 0.6 high (0.5 the electrolytic, 0.7 the piezoelectric) |
| inductors | `L` (`inductor`), `vL`, `sL` | cute, American or European by the `inductor` setting: `inductors/coils` arches with loops (5, at least 2), plain arches (4, at least 1), or a solid box; variable; inductive sensor |
| batteries | `battery`, `battery1`, `battery2`, `solar` (`pvmodule`'s cells)… | cells of plates, as the manual draws them |
| sources | `V` (`vsource`), `I` (`isource`), `sV`, `sI`, `nV`, `nI`, `sqV`, `vsourcetri`, `esource`, `pvsource`, `pvmodule`, `dcvsource`, `dcisource`, `ioosource`, `voosource`, `oosourcetrans`, `nullator`, `norator`… | every independent source of the manual, American, European or cute by the `voltage` and `current` settings (`american currents`, `european currents`, and their package options); the noise sources gray |
| csources | `cV`, `cI`, `cvsource`, `cisource`, `csV`, `csI`, `ecsource`… | the controlled sources, diamonds |
| instruments | `rmeter`, `rmeterwa`, `smeter`, `ammeter`, `voltmeter`, `ohmmeter`, `qiprobe`, `qvprobe`, `qpprobe`, `oscope`, `iloop`, `iloop2`, `currtap` | `t=…` writes upright in the middle of the meter (below the screen's) |
| mechanicals | `spring`, `damper`, `mass`, `inerter`, `viscoe` | the mechanical analogies |
| diodes | `D`, `sD`, `zD`, `zzD`, `tD`, `pD`, `leD`, `lasD`, `VC`, `tvsD`, `shD`, `biD`, each `…o` (empty), `…*` (full) and `…-` (stroke), and the long names (`empty diode`, `full Zener diode`, `stroke led`…) | the short keys follow the `diode` setting (`empty`, the default, `full`, `stroke`; the styles `empty diodes`, `full diodes`, `stroke diodes` and the package options `emptydiode`, `fulldiode`, `strokediode`); `led arrows from cathode`, `pd arrows to cathode` (and back, `… anode`) turn the arrows of LEDs and photodiodes; `diode straight whiskers` (`diode sloped whiskers`) the ZZener's and TVS's |
| diodes | `Tr` (`triac`), `Ty` (`thyristor`), `PUT` (`put`), `GTO` (`gto`), `GTOb` (`gtobar`), `aGTOb` (`agtobar`), `IGCT` (`igct`), each `…o`, `…*`, `…-` and long names | the thyristor-like tripoles, legacy or `thyristor style=compact`, with their gate (`G`, `gate`) and `anode`, `cathode` anchors |
| switches | `switch` (`spst`), `closing switch` (`cspst`), `opening switch` (`ospst`), `nos`, `ncs`, `oncs`, `cncs`, `onos`, `cnos`, `push button` (`nopb`), `ncpb`, `nopbc`, `ncpbo`, `toggle switch`, `reed`, `cute closed switch` (`ccsw`), `cute open switch` (`cosw`), `ccgsw`, `cogsw`, `inline proximeter`, `bare jumper`, `open jumper`, `closed jumper`, `three-pins jumper` (`tjumper connections`), the solder jumpers | with their anchors (`in`, `out`, `mid`, `tip`, `tap`, `out 1`…); the poles white unless filled |
| misc | `fuse`, `afuse`, `wfuse`, `lamp`, `bulb`, `wbulb`, `neonlampcc`, `neonlampac`, `sparkgap` (`sparkgap/dot`, `sparkgap/circle`), `loudspeaker`, `mic`, `tlmic`, `buzzer`, `rbuzzer`, `squid`, `barrier`, `openbarrier`, `relais`, `thermocouple`, `european gas filled surge arrester`… | the other bipoles |
| inductors | `cute inductor`, `american inductor`, `european inductor`, `variable cute inductor`, `variable american inductor`, `variable european inductor`, `cute inductive sensor`, `american inductive sensor`, `european inductive sensor`, `cute choke` | the same, whatever the setting; the choke has a core line |

## Node-style components

`\draw (0,0) node[ground] (G) {};` draws a component as a node: a node shape of its own (declared to tikra), with the component's anchors (`G.south`, `B.zero`, `S.socket center`), placed by them (`node[bnc, anchor=zero]`), turned and mirrored with the node (`rotate`, `xscale=-1`), and filled with its `fill` where the manual makes it fillable. Every node component has the compass anchors of its drawing (`north`, `south west`…, and `n`, `sw`, `left`, `right`); its `center` is the point it hangs from. Lines to it reach that point (to a terminal, its border).

| Class (`…/scale`) | Keys |
|---|---|
| grounds | `ground`, `tlground`, `rground`, `sground`, `tground`, `nground`, `pground`, `cground`, `eground`, `eground2` |
| power supplies | `vcc`, `vee` (the text above or below the arrow, at `text`) |
| connectors | `bnc` (`hot`, `zero`, `shield`), `iecconnshape`, `iecsocketL`, `iecsocketR`, `iecplugL`, `iecplugR` (`plug center`, `socket center`) |
| — | the terminals `circ`, `ocirc`, `diamondpole`, `odiamondpole`, `squarepole`, `osquarepole`, `rectjoinfill` (`nodes width`; the open ones white unless filled), the crossings `jump crossing`, `plain crossing`, the arrows `currarrow`, `inputarrow`, `trarrow`, `flowarrow` (`tip`, `btip`), the symbols `dc symbol`, `ac symbol`, the switches `spdt`, `cute spdt up`, `cute spdt mid`, `cute spdt down` (and their `… arrow`), `proximeter` (`in`, `out 1`, `out 2`, `mid`, `hlines nw`…; the `switches` class) |

**Transistors** (the `transistors` class): `npn`, `pnp`, `bjtnpn`, `bjtpnp` (`collectors`, `emitters`), `nigbt`, `pigbt`, `Lnigbt`, `Lpigbt`, `nmos`, `pmos`, `nmosd`, `pmosd`, `hemt`, `GaN hemt`, `nfet`, `nfetd`, `pfet`, `pfetd`, `nigfete`, `nigfetebulk`, `nigfetd`, `pigfete`, `pigfetebulk`, `pigfetd`, `njfet`, `pjfet`, `nujt`, `pujt`, `isfet`, `ngfet`, `pgfet`, with all their anchors (`B`, `C`, `E`, `G`, `D`, `S`, `bulk`, `nobase`, `G2`, `E1`, `circle C`…) and their options `schottky base`, `bodydiode`, `photo`, `tr circle`, `nobase`, `bulk`, `emptycircle`, `nocircle`, `arrowmos`, `solderdot`, `fdsoi`, `fdsoidot`, `doublegate`, in the combinations the manual draws (another one is drawn plain, with a warning). Their text goes right of them and does not count in the picture's size; a depletion channel (and a pmos bubble) is dark unless filled. On a path, `Tnpn`, `Tpnp`, `Tnmos`, `Tpmos`, `Tnmosd`, `Tpmosd`, `Tnfet`, `Tpfet`, `Tnigfete`, `Tnigfetebulk`, `Tnigfetd` (and `Tp…`), `Tnjfet`, `Tpjfet`, `Tnigbt`, `Tpigbt`, `TLnigbt`, `TLpigbt`, `Themt` lay the transistor along it, its value in its middle; `invert` turns them round.

What counts in the picture's size is what CircuiTikZ counts: the drawing, the text of the supplies (not of the connectors), the BNC's square.

Path-style, with the bipoles: `multiwire`, `bmultiwire`, `tmultiwire` (buses), `crossing` (`xing`), `iec connector` (`iecconn`, in the `connectors` class).

## Named components and their anchors

`to[R, name=R1]` names the component, and its anchors become coordinates: `(R1.north)`, `(R1.wiper)`. Every component has `center`, `north`, `south`, `east`, `west` (and `left`, `right`), and the four corners of its band. Variable components have `wiper` and `tip` (the ends of the arrow); potentiometers `wiper` and `tip` (the ends of the wiper); sensors `wiper`, `tip` and `label` (the ends and the corner of the bar); the ferroelectric capacitor `kink left`, `kink right`, `curve left`, `curve right`; inductors `midtap` (on the coil, above or below as the coils fall), `core west` and `core east` (`bipoles/inductors/core distance` above the coil, 2pt) and the dots of mutual inductance `ul dot`, `ur dot`, `ll dot`, `lr dot` (`bipoles/inductors/dot x distance` and `dot y distance`, 4pt and 1pt, beyond the corners). The anchors are mirrored and inverted with the component. `(R1)` alone is not a coordinate.

## Labels and annotations

- **Keys:** `l`, `l^`, `l_` (label), `a`, `a^`, `a_` (annotation), and `R=label` (any component key with a value) for `l`. The label goes above a left-to-right component, the annotation below; `^` and `_` choose the side. `mirror` and `invert` do not move them.
- **Alignment:** `label/align` (`\ctikzset`), or the styles `straight labels`, `rotate labels`, `smart labels`. `straight` never turns the text; `rotate` turns it along the line, read left to right; `smart` (the default) turns it only when the line is between 20° and 70° from an axis.
- **Place:** from the component's band widened by 10%, a straight label keeps its baseline (above and below) or the middle of its x-height (at the sides) at a fixed distance, whatever its letters; a rotated one sits further below the line than above it. `label distance` and `annotation distance` add to it.
- **Two lines:** `l2=first and second` (`l2^`, `l2_`, and `a2…` for the annotation), aligned by `l2 halign` (`l`, `c`, `r`) and placed by `l2 valign` (`b`, `c`, `t`), as the manual's `tabular`.
- **Nodes:** a named component's labels are nodes too, `R1label` and `R1annotation`, styled by `bipole label style` and `bipole annotation style` (and their `append` forms); `\ctikzgetanchor{R1}{label}` and `\ctikzgetdirection{R1label}` give their anchor and turn. Their nodes keep TikZ's inner sep, which counts in the picture's size.
- **Units:** `value<units>`, the siunitx shorthand, and `\SI{1}{\kilo\ohm}`, `\qty{…}{…}`, `\si{…}`, `\unit{…}`, `\num{…}` in any label, voltage or current, as `$value\,\mathrm{units}$` with the unit macros as symbols (`\kilo\ohm` → kΩ, `\micro\farad` → μF, and the abbreviations `\kohm`, `\mA`, `\nF`…).

## Voltages, currents and flows

- **Keys:** `v`, `i`, `f`, and their variants with a side (`^` the left of the path, `_` the other) and a direction (`>` toward the end, `<` toward the start): `v^>`, `i_<`, `f>^`… For currents and flows the symbol goes on the lead after the component, or before it when the direction comes first (`i>_`); `i<` alone goes before it. A key without a value (`to[R, v, name=R]`) draws nothing but gives the anchors.
- **Currents:** a filled arrow head on the lead, `current/distance` (0.5) of the way along it, with the label beside it. **Flows:** a short arrow beside the lead (`flow/distance`).
- **Voltages,** `voltage` = `european` (a curved arrow under the component, bent by the component's own bump; `voltage/bump b`), `straight`, `american` (`+` and `-` at the ends, `voltage/american plus`, `american minus`, `american font`, `american label distance`) or `raised` (the signs at the label's height); the styles `european voltages`, `straight voltages`, `american voltages`, `raised voltages`, and `american`/`european`. A voltage spans from `voltage/distance from node` of the way between each end and the component; an open circuit's lies on the line. `voltage shift` moves it all away from the component.
- **Direction:** `voltage dir` = `noold` (the package's default) and `EF` point the arrow from `+` to `-`, `RP` and `old` from `-` to `+`; with no `>` nor `<`, a passive component's voltage points to the end with the former, to the start with the latter. The document's `\usepackage[RPvoltages, american…]{circuitikz}` sets them (the voltage directions and the style options).
- **Anchors:** a named component's voltage, current and flow have `R1-Vfrom`, `R1-Vto`, `R1-Vcont1`, `R1-Vcont2`, `R1-Vlab`, `R1-Ifrom`, `R1-Ito`, `R1-Ipos`, `R1-Ffrom`, `R1-Fto`, `R1-Fpos`, their labels are the nodes `R1voltage`, `R1current`, `R1flow` (`bipole voltage style`…), and `\ctikzgetanchor{R1}{Vlab}` gives the label's anchor. `no v symbols`, `no i symbols`, `no f symbols` keep only the labels.
- **Sources:** a source's value is its voltage or its current (`V=$u$` is `V, v=$u$`; `I<=$a$` is `I, i<=$a$`); a component's own value stays its label. A voltage source's voltage (a battery's too) is a straight arrow across it, above it, pointing to its end in every convention, or `+` and `-` in the American styles; a current source's is the curved arrow the other way round from a passive component's; a voltage source's current goes out of its end. `bipole/is voltage=false`, `bipole/is current=false` and `bipole/override source vif` draw them as a passive component's.
- The shapes and places are measured on what CircuiTikZ draws, component by component (`components::VOLTS`, `SOURCE_VOLTS`).

## Poles and orientation

- `bipole nodes={start}{end}` puts a pole at each end: `none`, `circ`, `ocirc`, `diamondpole`, `odiamondpole`, `squarepole`, `osquarepole`, each with options of its own (`{odiamondpole, fill=red}`). The shorthands combine none, `o` (`ocirc`), `*` (`circ`) and `d` (`diamondpole`): `o-*`, `*-o`, `-o`, `d-d`, `o-d`…
- A pole is `nodes width` (0.04) of `bipoles/length` across the half, with the path's stroke, drawn over the wires; a full one is filled with the stroke's color, an open one with the component's fill, or white.
- `mirror` flips the component across the line; `invert` turns it end for end.

## Settings and styles

- `\ctikzset{…}` and `\ctikzvalof{…}` (keys under `/tikz/circuitikz/`): `bipoles/length`, `bipoles/thickness`, `resistor` (`american`, `european`), `inductor` (`cute`, `american`, `european`), `label/align`, `label distance`, `annotation distance`, `nodes width`, `wiper pos`, the class keys `resistors/…`, `capacitors/…`, `inductors/…` (`scale`, `width`, `height`, `thickness`, `modifier thickness`; `resistors/zigs`, `inductors/coils`), `bipoles/inductors/…` (`core distance`, `dot x distance`, `dot y distance`), and `quadpoles style`, kept for the quadpoles to come.
- The same settings in a `to[…]` or a TikZ style: `to[R, resistors/scale=1.8]`. A style defined with `\ctikzset{tallR/.style={…}}` is a TikZ style too: `to[R, tallR]`.
- Styles: `american`, `european`, `american resistors`, `european resistors`, `american inductors`, `european inductors`, `cute inductors`, on the environment, a scope or a path.

## Out, for now

- The rotary switches (`rotaryswitch`, `rotary switch`), the electronic tubes, op-amps, the transformers and the other components of chapter 4 of the manual after the miscellaneous bipoles.
- The sources' own keys (`prim`, `sec`, `tert` and the windings of the `oosource`s, `sign rotation`), the oscilloscope's `waveform`, the instruments' `rotated instruments`, the `sparkgap end arrow`, the poles of jumpers and cute switches as nodes of their own (`J1-in.s`), the switches' arrows and thickness customizations (`switch end arrow`, `bipoles/cuteswitch/thickness`), the transistors' customizations (`arrow pos`, `legacy transistors text`, `ferroel gate`, `tr gap fill`, `transistor circle/…`, `bjt pins width` other than the manual's), the opto and GTO gate arrows' customizations (`opto arrows/…`, `opto end arrow`, `gto gate end arrow`), and the experimental automatic voltages (`vif queue`, `\fixedvlen`).
- Bipoles as nodes (`node[resistorshape]`), and a named bipole as a node of its own (`(R1)`); the customizations of the supplies' arrows (`monopoles/vcc/arrow`) and the `Jack Tap` arrow tip.
- The fill and opacity of open poles set apart (`open poles fill`, `open poles opacity`), the arrow tips of tunable components and potentiometers (`tunable end arrow`, `wiper end arrow`), the zig-zag joins (`resistors/zigzag hook`, `zigzag stub`), the choke's lines (`twolineschoke`, `cthick`).
- Straight labels on lines at angles that are not multiples of 90° (15°, 75°…) are a few points from where CircuiTikZ puts them.
