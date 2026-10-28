# Conformance with CircuiTikZ

How far voltik is from CircuiTikZ, measured by its manual (CircuiTikZ 1.8.7): every example the manual
shows and every entry of its list of components, drawn by pdflatex (TeX Live, CircuiTikZ 1.8.7) and by
voltik, and compared. How it works and how to run it: [tools/conformance](../tools/conformance/README.md).

**44 of 541 components (8.1%) and 56 of 505 examples (11.1%) match TeX.** Of all 1046 items, 108 are close, 838 differ and 0 fail in voltik; 19 of the ones that do not match came out with no warning.

| | Items | Match | Close | Different | Fails | Mean F1 |
|---|---:|---:|---:|---:|---:|---:|
| All | 1046 | 100 (9.6%) | 108 (10.3%) | 838 (80.1%) | 0 (0.0%) | 0.41 |
| Examples of the manual | 505 | 56 (11.1%) | 108 (21.4%) | 341 (67.5%) | 0 (0.0%) | 0.45 |
| Path-style components | 306 | 44 (14.4%) | 0 (0.0%) | 262 (85.6%) | 0 (0.0%) | 0.60 |
| Node-style components | 235 | 0 (0.0%) | 0 (0.0%) | 235 (100.0%) | 0 (0.0%) | 0.08 |

*Match*: same shape (F1 ≥ 0.9 on the edges, with 0.5 mm of tolerance), no missing or extra piece, the same colors and size. *Close*: F1 ≥ 0.75.
*Fails*: voltik returned an error, panicked or took more than 10 s. Mean F1 counts a failure as 0.

A component is one entry of the manual's list, drawn as the manual draws it: `\draw (0,0) to[<name>] (2,0);` for a path-style one, `\draw (0,0) node[<name>]{};` for a node-style one, filled when the manual fills it. A component voltik says it does not know counts as different, even when the wire it draws instead scores close.

Not counted: 9 examples of the manual where tex could not compile it, 1 path-style components where tex draws nothing visible.

## By section

Sorted by how many items do not match, the work left.

| Section | Items | Match | Close | Different | Fails | Mean F1 |
|---|---:|---:|---:|---:|---:|---:|
| The components: list › Transistors <sub>node</sub> | 72 | 0 (0.0%) | 0 (0.0%) | 72 (100.0%) | 0 (0.0%) | 0.12 |
| The components: list › Diodes and such <sub>path</sub> | 70 | 0 (0.0%) | 0 (0.0%) | 70 (100.0%) | 0 (0.0%) | 0.48 |
| The components: list › Block diagram components <sub>path</sub> | 64 | 0 (0.0%) | 0 (0.0%) | 64 (100.0%) | 0 (0.0%) | 0.40 |
| Labels, voltages and currents › Voltages <sub>example</sub> | 44 | 0 (0.0%) | 21 (47.7%) | 23 (52.3%) | 0 (0.0%) | 0.58 |
| The components: list › Transistors <sub>example</sub> | 44 | 0 (0.0%) | 0 (0.0%) | 44 (100.0%) | 0 (0.0%) | 0.10 |
| The components: list › Sources and generators <sub>path</sub> | 38 | 0 (0.0%) | 0 (0.0%) | 38 (100.0%) | 0 (0.0%) | 0.56 |
| The components: list › Logic gates <sub>node</sub> | 34 | 0 (0.0%) | 0 (0.0%) | 34 (100.0%) | 0 (0.0%) | 0.01 |
| The components: list › Switches, buttons and jumpers <sub>path</sub> | 32 | 0 (0.0%) | 0 (0.0%) | 32 (100.0%) | 0 (0.0%) | 0.80 |
| The components: list › Logic gates <sub>example</sub> | 30 | 0 (0.0%) | 1 (3.3%) | 29 (96.7%) | 0 (0.0%) | 0.11 |
| The components: list › Block diagram components <sub>example</sub> | 26 | 0 (0.0%) | 1 (3.8%) | 25 (96.2%) | 0 (0.0%) | 0.26 |
| The components: list › Miscellaneous bipoles and symbols <sub>path</sub> | 23 | 0 (0.0%) | 0 (0.0%) | 23 (100.0%) | 0 (0.0%) | 0.62 |
| The components: list › Sources and generators <sub>example</sub> | 21 | 0 (0.0%) | 1 (4.8%) | 20 (95.2%) | 0 (0.0%) | 0.42 |
| The components: list › Amplifiers <sub>example</sub> | 20 | 0 (0.0%) | 0 (0.0%) | 20 (100.0%) | 0 (0.0%) | 0.03 |
| The components: list › Switches, buttons and jumpers <sub>example</sub> | 19 | 0 (0.0%) | 1 (5.3%) | 18 (94.7%) | 0 (0.0%) | 0.32 |
| The components: list › Instruments <sub>example</sub> | 18 | 0 (0.0%) | 0 (0.0%) | 18 (100.0%) | 0 (0.0%) | 0.50 |
| Labels, voltages and currents › Currents <sub>example</sub> | 15 | 0 (0.0%) | 9 (60.0%) | 6 (40.0%) | 0 (0.0%) | 0.69 |
| The components: list › RF components <sub>node</sub> | 14 | 0 (0.0%) | 0 (0.0%) | 14 (100.0%) | 0 (0.0%) | 0.18 |
| The components: list › Block diagram components <sub>node</sub> | 13 | 0 (0.0%) | 0 (0.0%) | 13 (100.0%) | 0 (0.0%) | 0.01 |
| The components: list › Instruments <sub>path</sub> | 13 | 0 (0.0%) | 0 (0.0%) | 13 (100.0%) | 0 (0.0%) | 0.56 |
| The components: list › Multiplexer and de-multiplexer <sub>example</sub> | 13 | 0 (0.0%) | 0 (0.0%) | 13 (100.0%) | 0 (0.0%) | 0.06 |
| Labels, voltages and currents › Currents and voltages <sub>example</sub> | 12 | 0 (0.0%) | 1 (8.3%) | 11 (91.7%) | 0 (0.0%) | 0.53 |
| The components: list › Double bipoles (transformers) <sub>example</sub> | 12 | 0 (0.0%) | 0 (0.0%) | 12 (100.0%) | 0 (0.0%) | 0.03 |
| The components: list › Electronic Tubes <sub>node</sub> | 12 | 0 (0.0%) | 0 (0.0%) | 12 (100.0%) | 0 (0.0%) | 0.04 |
| The components: list › Flip-flops <sub>node</sub> | 12 | 0 (0.0%) | 0 (0.0%) | 12 (100.0%) | 0 (0.0%) | 0.05 |
| The components: list › Grounds and supply voltages <sub>node</sub> | 12 | 0 (0.0%) | 0 (0.0%) | 12 (100.0%) | 0 (0.0%) | 0.00 |
| The components: list › Amplifiers <sub>node</sub> | 11 | 0 (0.0%) | 0 (0.0%) | 11 (100.0%) | 0 (0.0%) | 0.01 |
| The components: list › Double bipoles (transformers) <sub>node</sub> | 9 | 0 (0.0%) | 0 (0.0%) | 9 (100.0%) | 0 (0.0%) | 0.04 |
| The components: list › Switches, buttons and jumpers <sub>node</sub> | 9 | 0 (0.0%) | 0 (0.0%) | 9 (100.0%) | 0 (0.0%) | 0.06 |
| The components: usage › Path-style components <sub>example</sub> | 11 | 3 (27.3%) | 5 (45.5%) | 3 (27.3%) | 0 (0.0%) | 0.81 |
| Colors › Shape colors <sub>example</sub> | 7 | 0 (0.0%) | 4 (57.1%) | 3 (42.9%) | 0 (0.0%) | 0.49 |
| Labels, voltages and currents <sub>example</sub> | 7 | 0 (0.0%) | 7 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.85 |
| Labels, voltages and currents › Flows <sub>example</sub> | 7 | 0 (0.0%) | 7 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.87 |
| The components: list › Connectors <sub>example</sub> | 8 | 1 (12.5%) | 2 (25.0%) | 5 (62.5%) | 0 (0.0%) | 0.62 |
| The components: list › Electronic Tubes <sub>example</sub> | 7 | 0 (0.0%) | 1 (14.3%) | 6 (85.7%) | 0 (0.0%) | 0.19 |
| The components: list › Miscellaneous bipoles and symbols <sub>example</sub> | 7 | 0 (0.0%) | 1 (14.3%) | 6 (85.7%) | 0 (0.0%) | 0.48 |
| The components: list › Terminal shapes <sub>node</sub> | 7 | 0 (0.0%) | 0 (0.0%) | 7 (100.0%) | 0 (0.0%) | 0.00 |
| The components: usage › Subcircuits <sub>example</sub> | 7 | 0 (0.0%) | 2 (28.6%) | 5 (71.4%) | 0 (0.0%) | 0.30 |
| Tutorials › Getting started with : a current shunt <sub>example</sub> | 7 | 0 (0.0%) | 7 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.88 |
| Using bipoles in circuits › Mirroring and Inverting <sub>example</sub> | 7 | 0 (0.0%) | 0 (0.0%) | 7 (100.0%) | 0 (0.0%) | 0.53 |
| Labels, voltages and currents › Advanced voltages, currents and flows <sub>example</sub> | 6 | 0 (0.0%) | 6 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.79 |
| Labels, voltages and currents › Integration with siunitx <sub>example</sub> | 6 | 0 (0.0%) | 6 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.87 |
| The components: list › Chips (integrated circuits) <sub>example</sub> | 6 | 0 (0.0%) | 0 (0.0%) | 6 (100.0%) | 0 (0.0%) | 0.12 |
| The components: list › Connectors <sub>node</sub> | 6 | 0 (0.0%) | 0 (0.0%) | 6 (100.0%) | 0 (0.0%) | 0.41 |
| The components: list › Diodes and such <sub>example</sub> | 6 | 0 (0.0%) | 0 (0.0%) | 6 (100.0%) | 0 (0.0%) | 0.46 |
| The components: list › Logic gates <sub>path</sub> | 6 | 0 (0.0%) | 0 (0.0%) | 6 (100.0%) | 0 (0.0%) | 0.42 |
| The components: list › Multiplexer and de-multiplexer <sub>node</sub> | 6 | 0 (0.0%) | 0 (0.0%) | 6 (100.0%) | 0 (0.0%) | 0.11 |
| Colors › Fill colors <sub>example</sub> | 8 | 3 (37.5%) | 1 (12.5%) | 4 (50.0%) | 0 (0.0%) | 0.54 |
| Labels, voltages and currents › Labels and Annotations <sub>example</sub> | 11 | 6 (54.5%) | 4 (36.4%) | 1 (9.1%) | 0 (0.0%) | 0.97 |
| The components: list › Mechanical Analogy <sub>path</sub> | 5 | 0 (0.0%) | 0 (0.0%) | 5 (100.0%) | 0 (0.0%) | 0.63 |
| Tutorials › A non-inverting op-amp amplifier <sub>example</sub> | 5 | 0 (0.0%) | 0 (0.0%) | 5 (100.0%) | 0 (0.0%) | 0.07 |
| The components: list › Arrows (fake and real) <sub>node</sub> | 4 | 0 (0.0%) | 0 (0.0%) | 4 (100.0%) | 0 (0.0%) | 0.12 |
| The components: list › Flip-flops <sub>example</sub> | 4 | 0 (0.0%) | 0 (0.0%) | 4 (100.0%) | 0 (0.0%) | 0.04 |
| The components: usage › Node-style components <sub>node</sub> | 4 | 0 (0.0%) | 0 (0.0%) | 4 (100.0%) | 0 (0.0%) | 0.00 |
| Defining new components › Path-style component <sub>example</sub> | 3 | 0 (0.0%) | 0 (0.0%) | 3 (100.0%) | 0 (0.0%) | 0.39 |
| The components: list › Arrows (fake and real) <sub>example</sub> | 4 | 1 (25.0%) | 1 (25.0%) | 2 (50.0%) | 0 (0.0%) | 0.64 |
| The components: list › Electro-Mechanical Devices <sub>example</sub> | 3 | 0 (0.0%) | 0 (0.0%) | 3 (100.0%) | 0 (0.0%) | 0.35 |
| The components: list › Multiple wires (buses) <sub>path</sub> | 3 | 0 (0.0%) | 0 (0.0%) | 3 (100.0%) | 0 (0.0%) | 0.85 |
| The components: list › RF components <sub>example</sub> | 3 | 0 (0.0%) | 0 (0.0%) | 3 (100.0%) | 0 (0.0%) | 0.28 |
| The components: list › RF components <sub>path</sub> | 3 | 0 (0.0%) | 0 (0.0%) | 3 (100.0%) | 0 (0.0%) | 0.44 |
| Tutorials › A logic circuit <sub>example</sub> | 3 | 0 (0.0%) | 0 (0.0%) | 3 (100.0%) | 0 (0.0%) | 0.00 |
| Colors <sub>example</sub> | 2 | 0 (0.0%) | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.97 |
| Examples › Using the (experimental) siunitx syntax <sub>example</sub> | 2 | 0 (0.0%) | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.89 |
| FAQ: Frequently asked questions › Using dashed (or colored) wires in circuits <sub>example</sub> | 7 | 5 (71.4%) | 2 (28.6%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| Introduction › Package options <sub>example</sub> | 2 | 0 (0.0%) | 1 (50.0%) | 1 (50.0%) | 0 (0.0%) | 0.75 |
| Introduction › Scale factor inaccuracies <sub>example</sub> | 2 | 0 (0.0%) | 0 (0.0%) | 2 (100.0%) | 0 (0.0%) | 0.17 |
| Labels, voltages and currents › Accessing labels text nodes <sub>example</sub> | 2 | 0 (0.0%) | 0 (0.0%) | 2 (100.0%) | 0 (0.0%) | 0.53 |
| The components: list › Chips (integrated circuits) <sub>node</sub> | 2 | 0 (0.0%) | 0 (0.0%) | 2 (100.0%) | 0 (0.0%) | 0.00 |
| The components: list › Crossings <sub>example</sub> | 4 | 2 (50.0%) | 1 (25.0%) | 1 (25.0%) | 0 (0.0%) | 0.74 |
| The components: list › Crossings <sub>node</sub> | 2 | 0 (0.0%) | 0 (0.0%) | 2 (100.0%) | 0 (0.0%) | 0.00 |
| The components: list › Electro-Mechanical Devices <sub>node</sub> | 2 | 0 (0.0%) | 0 (0.0%) | 2 (100.0%) | 0 (0.0%) | 0.26 |
| The components: list › Grounds and supply voltages <sub>example</sub> | 2 | 0 (0.0%) | 0 (0.0%) | 2 (100.0%) | 0 (0.0%) | 0.28 |
| The components: list › Miscellaneous bipoles and symbols <sub>node</sub> | 2 | 0 (0.0%) | 0 (0.0%) | 2 (100.0%) | 0 (0.0%) | 0.72 |
| The components: list › Resistive bipoles <sub>example</sub> | 6 | 4 (66.7%) | 2 (33.3%) | 0 (0.0%) | 0 (0.0%) | 0.97 |
| The components: list › Seven segment displays <sub>example</sub> | 2 | 0 (0.0%) | 0 (0.0%) | 2 (100.0%) | 0 (0.0%) | 0.00 |
| The components: usage › Node-style components <sub>example</sub> | 2 | 0 (0.0%) | 0 (0.0%) | 2 (100.0%) | 0 (0.0%) | 0.35 |
| Tutorials › A transistor-based amplifier <sub>example</sub> | 2 | 0 (0.0%) | 0 (0.0%) | 2 (100.0%) | 0 (0.0%) | 0.15 |
| Using bipoles in circuits › Putting them together <sub>example</sub> | 2 | 0 (0.0%) | 1 (50.0%) | 1 (50.0%) | 0 (0.0%) | 0.54 |
| Defining new components › Path-style component <sub>path</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.65 |
| Defining new components › Suggested setup <sub>example</sub> | 1 | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.85 |
| Defining new components › Suggested setup <sub>path</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.67 |
| Examples › 3-phases block schematic <sub>example</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.10 |
| Examples › A Sallen-Key cell <sub>example</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.12 |
| Examples › A red diode <sub>example</sub> | 1 | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.86 |
| Examples › A styled low noise input stage <sub>example</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.09 |
| Examples › Automatic user-defined voltages, currents and flows (since v1.8.5) <sub>example</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.24 |
| Examples › Mixing circuits and graphs <sub>example</sub> | 1 | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.90 |
| Examples › Photodiodes <sub>example</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.00 |
| Examples › RF circuit <sub>example</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.00 |
| Examples › Using components in pics (since v1.8.0) <sub>example</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.67 |
| FAQ: Frequently asked questions › Labels, voltages and currents woes <sub>example</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.00 |
| FAQ: Frequently asked questions › Tunable components <sub>example</sub> | 1 | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.94 |
| Introduction › Known bugs and limitation <sub>example</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.66 |
| Introduction › Loading the package <sub>example</sub> | 1 | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Labels, voltages and currents › Changing the style of labels, voltages, and other text ornaments <sub>example</sub> | 1 | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.77 |
| The components: list › Capacitors and inductors: dynamical bipoles <sub>example</sub> | 8 | 7 (87.5%) | 1 (12.5%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| The components: list › Connectors <sub>path</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.71 |
| The components: list › Crossings <sub>path</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 1.00 |
| The components: list › Multiple wires (buses) <sub>example</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.18 |
| The components: list › Seven segment displays <sub>node</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.00 |
| The components: list › Terminal shapes <sub>example</sub> | 1 | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.98 |
| The components: usage › Path-style components <sub>path</sub> | 3 | 2 (66.7%) | 0 (0.0%) | 1 (33.3%) | 0 (0.0%) | 0.85 |
| The components: usage › Styling circuits and components <sub>example</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.20 |
| Tutorials › A non-inverting op-amp amplifier <sub>node</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.00 |
| Using bipoles in circuits › Nodes (also called poles) <sub>example</sub> | 19 | 18 (94.7%) | 0 (0.0%) | 1 (5.3%) | 0 (0.0%) | 0.99 |
| FAQ: Frequently asked questions › Using named nodes in circuits <sub>example</sub> | 4 | 4 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Capacitors and inductors: dynamical bipoles <sub>path</sub> | 18 | 18 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Resistive bipoles <sub>path</sub> | 24 | 24 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Using bipoles in circuits › Line joins between Path Components <sub>example</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |

## Components

The manual's list of components, group by group, with the ones voltik draws as TeX does (and, in italics, the close ones).

| Group | Components | Match | Close | Drawn as TeX does |
|---|---:|---:|---:|---|
| Tutorials › A non-inverting op-amp amplifier | 1 | 0 | 0 |  |
| The components: usage › Path-style components | 3 | 2 | 0 | `resistor` `pR` |
| The components: usage › Node-style components | 4 | 0 | 0 |  |
| The components: list › Grounds and supply voltages | 12 | 0 | 0 |  |
| The components: list › Resistive bipoles | 24 | 24 | 0 | `short` `generic` `xgeneric` `sgeneric` `singeneric` `tgeneric` `ageneric` `memristor` `R` `vR` `pR` `sR` `ldR` `R` `vR` `pR` `sR` `ldR` `varistor` `mov` `phR` `thR` `thRp` `thRn` |
| The components: list › Capacitors and inductors: dynamical bipoles | 18 | 18 | 0 | `capacitor` `curved capacitor` `ecapacitor` `variable capacitor` `capacitive sensor` `piezoelectric` `cpe` `feC` `L` `vL` `sL` `L` `vL` `sL` `L` `vL` `sL` `cute choke` |
| The components: list › Diodes and such | 70 | 0 | 0 |  |
| The components: list › Sources and generators | 38 | 0 | 0 |  |
| The components: list › Instruments | 13 | 0 | 0 |  |
| The components: list › Mechanical Analogy | 5 | 0 | 0 |  |
| The components: list › Miscellaneous bipoles and symbols | 25 | 0 | 0 |  |
| The components: list › Multiple wires (buses) | 3 | 0 | 0 |  |
| The components: list › Crossings | 3 | 0 | 0 |  |
| The components: list › Arrows (fake and real) | 4 | 0 | 0 |  |
| The components: list › Terminal shapes | 7 | 0 | 0 |  |
| The components: list › Connectors | 7 | 0 | 0 |  |
| The components: list › Block diagram components | 77 | 0 | 0 |  |
| The components: list › Transistors | 72 | 0 | 0 |  |
| The components: list › Electronic Tubes | 12 | 0 | 0 |  |
| The components: list › RF components | 17 | 0 | 0 |  |
| The components: list › Electro-Mechanical Devices | 2 | 0 | 0 |  |
| The components: list › Double bipoles (transformers) | 9 | 0 | 0 |  |
| The components: list › Amplifiers | 11 | 0 | 0 |  |
| The components: list › Switches, buttons and jumpers | 41 | 0 | 0 |  |
| The components: list › Logic gates | 40 | 0 | 0 |  |
| The components: list › Flip-flops | 12 | 0 | 0 |  |
| The components: list › Multiplexer and de-multiplexer | 6 | 0 | 0 |  |
| The components: list › Chips (integrated circuits) | 2 | 0 | 0 |  |
| The components: list › Seven segment displays | 1 | 0 | 0 |  |
| Defining new components › Suggested setup | 1 | 0 | 0 |  |
| Defining new components › Path-style component | 1 | 0 | 0 |  |

## What voltik reported

Warnings, by how many items they appear in (the varying part replaced by …):

| Items | Warning |
|---:|---|
| 932 | `option … not supported: ignored` |
| 49 | `\draw: figure: unknown coordinate: (…); statement ignored` |
| 29 | `unknown anchor …: center used` |
| 19 | `\node: figure: unknown coordinate: (…); statement ignored` |
| 10 | `\draw: figure: unknown anchor: C.pin N; statement ignored` |
| 8 | `\draw: figure: unknown anchor: opamp.+; statement ignored` |
| 7 | `E-REC-N column N: unsupported text command: \SI; written as is` |
| 7 | `\draw: figure: unknown anchor: E.out; statement ignored` |
| 7 | `\draw: figure: unknown anchor: myandN.out; statement ignored` |
| 6 | `\draw: figure: unknown anchor: A.out; statement ignored` |
| 4 | `\path: figure: unknown coordinate: (…); statement ignored` |
| 4 | `\draw: figure: unknown anchor: T.AN; statement ignored` |
| 4 | `E-REC-N column N: unsupported text command: \rotatebox; written as is` |
| 3 | `\draw: figure: unknown anchor: OA.-; statement ignored` |
| 3 | `\draw: figure: unknown anchor: FF.bup; statement ignored` |
| 3 | `E-REC-N column N: unsupported text command: \scalebox; written as is` |
| 3 | `\draw: figure: unknown anchor: TN.S; statement ignored` |
| 2 | `E-REC-N column N: unsupported text command: \height; written as is` |
| 2 | `E-REC-N column N: unsupported text command: \raisebox; written as is` |
| 2 | `\draw: figure: unknown anchor: FF.pin N; statement ignored` |
| 2 | `E-REC-N column N: unsupported text command: \ctikzflipx; written as is` |
| 2 | `E-REC-N column N: unsupported text command: \ctikzflipy; written as is` |
| 2 | `E-REC-N column N: unsupported text command: \ctikzflipxy; written as is` |
| 2 | `unrecognized path fragment: …` |
| 2 | `E-REC-N column N: unsupported text command: \qty; written as is` |
| 2 | `\draw: figure: unknown anchor: PN.D; statement ignored` |
| 2 | `\draw: figure: unknown anchor: B.nobase; statement ignored` |
| 2 | `\draw: figure: unknown anchor: pnpN.B; statement ignored` |
| 2 | `\draw: figure: unknown anchor: motor.bottom; statement ignored` |
| 2 | `\draw: figure: unknown anchor: B.out N; statement ignored` |
| 2 | `\draw: figure: unknown anchor: C.out; statement ignored` |
| 2 | `\draw: figure: unknown anchor: B.in N; statement ignored` |
| 2 | `\node: figure: unknown anchor: B.in N; statement ignored` |
| 2 | `\node: figure: unknown anchor: A.bin N; statement ignored` |
| 2 | `\draw: figure: unknown anchor: AN.body left; statement ignored` |
| 2 | `\node: figure: unknown anchor: C.bpin N; statement ignored` |
| 2 | `\draw: figure: unknown anchor: C.bpin N; statement ignored` |
| 1 | `\draw: figure: unknown anchor: OAN-OA.-; statement ignored` |
| 1 | `\draw: figure: unknown anchor: P.wiper; statement ignored` |
| 1 | `\ctikzsubcircuitactivate not supported: … ignored` |

Components voltik does not know yet, by how many items use them:

| Items | Component |
|---:|---|
| 29 | `npn` |
| 20 | `op amp` |
| 14 | `nmos` |
| 14 | `muxdemux` |
| 13 | `dipchip` |
| 11 | `pnp` |
| 11 | `transformer` |
| 10 | `pmos` |
| 9 | `ocirc` |
| 9 | `oosourcetrans` |
| 9 | `nigfete` |
| 8 | `en amp` |
| 7 | `amp` |
| 7 | `mixer` |
| 7 | `transformer core` |
| 7 | `flipflop JK` |
| 6 | `flipflop` |
| 6 | `rmeterwa` |
| 6 | `ooosource` |
| 6 | `nmosd` |
| 6 | `pmosd` |
| 6 | `nigfetebulk` |
| 6 | `cute spdt up` |
| 5 | `battery2` |
| 5 | `ammeter` |
| 5 | `circ` |
| 5 | `oscope` |
| 5 | `viscoe` |
| 5 | `nigbt` |
| 5 | `pigbt` |
| 5 | `pigfete` |
| 5 | `pigfetebulk` |
| 5 | `inline not` |
| 4 | `plain amp` |
| 4 | `tlground` |
| 4 | `esource` |
| 4 | `oosourceatrans` |
| 4 | `dcvsource` |
| 4 | `smeter` |
| 4 | `spring` |
| 4 | `sparkgap` |
| 4 | `bnc` |
| 4 | `iec connector` |
| 4 | `lowpass` |
| 4 | `lowpassp` |
| 4 | `coupler` |
| 4 | `nfet` |
| 4 | `pfet` |
| 4 | `diodetube` |
| 4 | `matube` |
| 4 | `elmech` |
| 4 | `double bipole` |
| 4 | `fd op amp` |
| 4 | `three-pins jumper` |
| 4 | `qfpchip` |
| 3 | `put` |
| 3 | `vcc` |
| 3 | `rmeter` |
| 3 | `qiprobe` |
| 3 | `qvprobe` |

Other options voltik does not know, likewise:

| Items | Option |
|---:|---|
| 64 | `v` |
| 37 | `i` |
| 37 | `thyristor style` |
| 28 | `logic ports` |
| 27 | `thickness` |
| 25 | `scale` |
| 21 | `width` |
| 21 | `V` |
| 15 | `arrows` |
| 14 | `fill` |
| 14 | `height` |
| 13 | `tr circle` |
| 13 | `muxdemux def` |
| 13 | `xnor port` |
| 12 | `v_` |
| 12 | `t` |
| 12 | `dash` |
| 12 | `I` |
| 11 | `and port` |
| 10 | `voltage shift` |
| 10 | `color` |
| 10 | `relative thickness` |
| 10 | `i_` |
| 9 | `noinv input up` |
| 9 | `sV` |
| 9 | `doublegate` |
| 9 | `bulk` |
| 9 | `external pins width` |
| 9 | `american voltages` |
| 8 | `arrow pos` |
| 8 | `fdsoi` |
| 8 | `nand port` |
| 8 | `muxdemux label` |
| 8 | `straight voltages` |
| 7 | `component text` |
| 7 | `prim` |
| 7 | `boxed` |
| 7 | `ideal filter` |
| 6 | `isource` |
| 6 | `f` |

Errors of the items that fail:

| Items | Error |
|---:|---|
