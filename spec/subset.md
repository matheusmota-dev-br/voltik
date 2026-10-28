# CircuiTikZ subset

What voltik reads and how it draws it. The specification is the CircuiTikZ manual, version 1.8.7; what the manual leaves open is measured on the drawings CircuiTikZ itself makes (`tools/oracle/`). Anything outside this list becomes a warning and the rest of the circuit is still drawn.

## Model

voltik is a library of tikra (`tikra::extension`): the figure is TikZ, read and drawn by tikra, and the CircuiTikZ keys are added to it. A `circuitikz` environment is a `tikzpicture`; the components are keys of `to[…]`, so they work in any `tikzpicture` too.

## Path-style components

`\draw (A) to[name=label, options] (B);` places the component halfway between `A` and `B`, turned along the line, and draws the leads from `A` to it and from it to `B`.

- **Size:** every length is a fraction of `bipoles/length` (1.4 cm): a component has a width (where the leads stop) and a height (the band that labels keep out of). The class keys scale them: `resistors/scale`, `resistors/width`, and the same for `capacitors` and `inductors`.
- **Strokes:** the component's outline is `bipoles/thickness` (2) times the line width of the path, or its class's `thickness`; the leads keep the path's own width. Zig-zags and coils start and end half a thick stroke beyond the leads, so the joint is covered.
- **Coordinates and transforms:** the ends are read as TikZ reads them (named nodes give their center); the picture's `scale` moves the ends but does not resize the components, as with nodes.

| Key | Component | Look |
|---|---|---|
| `short` | wire | the leads join |
| `open` | open circuit | nothing but its poles and labels |
| `generic`, `european resistor` | generic bipole, European resistor | rectangle 0.8 × 0.3 |
| `american resistor` | American resistor | zig-zag 0.8 × 0.3, `resistors/zigs` teeth (3) |
| `R`, `resistor` | resistor | American or European, by the `resistor` setting |
| `C`, `capacitor` | capacitor | two plates 0.6 high, 0.2 apart |
| `cute inductor` | inductor | `inductors/coils` arches (5) over 0.6, with small loops between them |
| `L`, `inductor` | inductor | cute; American and European inductors are not drawn yet (a warning, and a cute one) |

## Labels and annotations

- **Keys:** `l`, `l^`, `l_` (label), `a`, `a^`, `a_` (annotation), and `R=label` (any component key with a value) for `l`. The label goes above a left-to-right component, the annotation below; `^` and `_` choose the side. `mirror` and `invert` do not move them.
- **Alignment:** `label/align` (`\ctikzset`), or the styles `straight labels`, `rotate labels`, `smart labels`. `straight` never turns the text; `rotate` turns it along the line, read left to right; `smart` (the default) turns it only when the line is between 20° and 70° from an axis.
- **Place:** from the component's band widened by 10%, a straight label keeps its baseline (above and below) or the middle of its x-height (at the sides) at a fixed distance, whatever its letters; a rotated one sits further below the line than above it. `label distance` and `annotation distance` add to it.
- **Units:** `value<units>`, the siunitx shorthand, as `$value\,\mathrm{units}$` with the unit macros as symbols (`\kilo\ohm` → kΩ, `\micro\farad` → μF).

## Poles and orientation

- `o-*`, `*-o`, `-o`, `o-`, `*-`, `-*`, `o-o`, `*-*`: an open pole is a circle filled in white, a closed one a filled circle, 0.04 of `bipoles/length` across the radius, with the path's stroke, drawn over the wires.
- `mirror` flips the component across the line; `invert` turns it end for end.

## Settings and styles

- `\ctikzset{…}` and `\ctikzvalof{…}` (keys under `/tikz/circuitikz/`): `bipoles/length`, `bipoles/thickness`, `resistor` (`american`, `european`), `inductor`, `label/align`, `label distance`, `annotation distance`, and the class keys above.
- Styles: `american`, `european`, `american resistors`, `european resistors`, `american inductors`, `european inductors`, `cute inductors`, on the environment, a scope or a path.

## Out, for now

- Sources, batteries, diodes, transistors, op-amps, grounds and every other component of chapter 4 of the manual.
- Voltages, currents and flows (`v=`, `i=`, `f=`).
- Node-style components (`node[resistorshape]`) and named bipoles with their anchors (`name=`).
- Straight labels on lines at angles that are not multiples of 90° (15°, 75°…) are a few points from where CircuiTikZ puts them.
