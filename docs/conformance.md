# Conformance with CircuiTikZ

How far voltik is from CircuiTikZ, measured by its manual (CircuiTikZ 1.8.7): every example the manual
shows and every entry of its list of components, drawn by pdflatex (TeX Live, CircuiTikZ 1.8.7) and by
voltik, and compared. How it works and how to run it: [tools/conformance](../tools/conformance/README.md).

**540 of 541 components (99.8%) and 470 of 505 examples (93.1%) match TeX.** Of all 1046 items, 35 are close, 1 differ and 0 fail in voltik; 32 of the ones that do not match came out with no warning.

| | Items | Match | Close | Different | Fails | Mean F1 |
|---|---:|---:|---:|---:|---:|---:|
| All | 1046 | 1010 (96.6%) | 35 (3.3%) | 1 (0.1%) | 0 (0.0%) | 1.00 |
| Examples of the manual | 505 | 470 (93.1%) | 35 (6.9%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| Path-style components | 306 | 306 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Node-style components | 235 | 234 (99.6%) | 0 (0.0%) | 1 (0.4%) | 0 (0.0%) | 1.00 |

*Match*: same shape (F1 ≥ 0.9 on the edges, with 0.5 mm of tolerance), no missing or extra piece, the same colors and size. *Close*: F1 ≥ 0.75.
*Fails*: voltik returned an error, panicked or took more than 10 s. Mean F1 counts a failure as 0.

A component is one entry of the manual's list, drawn as the manual draws it: `\draw (0,0) to[<name>] (2,0);` for a path-style one, `\draw (0,0) node[<name>]{};` for a node-style one, filled when the manual fills it. A component voltik says it does not know counts as different, even when the wire it draws instead scores close.

Not counted: 9 examples of the manual where tex could not compile it, 1 path-style components where tex draws nothing visible.

## By section

Sorted by how many items do not match, the work left.

| Section | Items | Match | Close | Different | Fails | Mean F1 |
|---|---:|---:|---:|---:|---:|---:|
| Labels, voltages and currents › Voltages <sub>example</sub> | 44 | 41 (93.2%) | 3 (6.8%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Logic gates <sub>example</sub> | 30 | 27 (90.0%) | 3 (10.0%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| Colors <sub>example</sub> | 2 | 0 (0.0%) | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Labels, voltages and currents › Labels and Annotations <sub>example</sub> | 11 | 9 (81.8%) | 2 (18.2%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| The components: list › Multiplexer and de-multiplexer <sub>example</sub> | 13 | 11 (84.6%) | 2 (15.4%) | 0 (0.0%) | 0 (0.0%) | 0.98 |
| The components: list › Sources and generators <sub>example</sub> | 21 | 19 (90.5%) | 2 (9.5%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| The components: list › Transistors <sub>example</sub> | 44 | 42 (95.5%) | 2 (4.5%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| Using bipoles in circuits › Mirroring and Inverting <sub>example</sub> | 7 | 5 (71.4%) | 2 (28.6%) | 0 (0.0%) | 0 (0.0%) | 0.98 |
| Colors › Fill colors <sub>example</sub> | 8 | 7 (87.5%) | 1 (12.5%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Colors › Shape colors <sub>example</sub> | 7 | 6 (85.7%) | 1 (14.3%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Defining new components › Path-style component <sub>example</sub> | 3 | 2 (66.7%) | 1 (33.3%) | 0 (0.0%) | 0 (0.0%) | 0.98 |
| Examples › 3-phases block schematic <sub>example</sub> | 1 | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.86 |
| Examples › Using components in pics (since v1.8.0) <sub>example</sub> | 1 | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0.96 |
| FAQ: Frequently asked questions › Using dashed (or colored) wires in circuits <sub>example</sub> | 7 | 6 (85.7%) | 1 (14.3%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Labels, voltages and currents › Accessing labels text nodes <sub>example</sub> | 2 | 1 (50.0%) | 1 (50.0%) | 0 (0.0%) | 0 (0.0%) | 0.95 |
| Labels, voltages and currents › Advanced voltages, currents and flows <sub>example</sub> | 6 | 5 (83.3%) | 1 (16.7%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| Labels, voltages and currents › Currents and voltages <sub>example</sub> | 12 | 11 (91.7%) | 1 (8.3%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| The components: list › Amplifiers <sub>example</sub> | 20 | 19 (95.0%) | 1 (5.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Arrows (fake and real) <sub>example</sub> | 4 | 3 (75.0%) | 1 (25.0%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| The components: list › Electronic Tubes <sub>example</sub> | 7 | 6 (85.7%) | 1 (14.3%) | 0 (0.0%) | 0 (0.0%) | 0.98 |
| The components: list › Miscellaneous bipoles and symbols <sub>example</sub> | 7 | 6 (85.7%) | 1 (14.3%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| The components: list › Resistive bipoles <sub>example</sub> | 6 | 5 (83.3%) | 1 (16.7%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Seven segment displays <sub>node</sub> | 1 | 0 (0.0%) | 0 (0.0%) | 1 (100.0%) | 0 (0.0%) | 0.67 |
| The components: usage › Path-style components <sub>example</sub> | 11 | 10 (90.9%) | 1 (9.1%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| The components: usage › Subcircuits <sub>example</sub> | 7 | 6 (85.7%) | 1 (14.3%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| Tutorials › Getting started with : a current shunt <sub>example</sub> | 7 | 6 (85.7%) | 1 (14.3%) | 0 (0.0%) | 0 (0.0%) | 0.98 |
| Defining new components › Path-style component <sub>path</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Defining new components › Suggested setup <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Defining new components › Suggested setup <sub>path</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Examples › A Sallen-Key cell <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Examples › A red diode <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| Examples › A styled low noise input stage <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| Examples › Automatic user-defined voltages, currents and flows (since v1.8.5) <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Examples › Mixing circuits and graphs <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Examples › Photodiodes <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Examples › RF circuit <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Examples › Using the (experimental) siunitx syntax <sub>example</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| FAQ: Frequently asked questions › Labels, voltages and currents woes <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| FAQ: Frequently asked questions › Tunable components <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| FAQ: Frequently asked questions › Using named nodes in circuits <sub>example</sub> | 4 | 4 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Introduction › Known bugs and limitation <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Introduction › Loading the package <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Introduction › Package options <sub>example</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Introduction › Scale factor inaccuracies <sub>example</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Labels, voltages and currents <sub>example</sub> | 7 | 7 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Labels, voltages and currents › Changing the style of labels, voltages, and other text ornaments <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0.98 |
| Labels, voltages and currents › Currents <sub>example</sub> | 15 | 15 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Labels, voltages and currents › Flows <sub>example</sub> | 7 | 7 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Labels, voltages and currents › Integration with siunitx <sub>example</sub> | 6 | 6 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Amplifiers <sub>node</sub> | 11 | 11 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Arrows (fake and real) <sub>node</sub> | 4 | 4 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Block diagram components <sub>example</sub> | 26 | 26 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| The components: list › Block diagram components <sub>node</sub> | 13 | 13 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Block diagram components <sub>path</sub> | 64 | 64 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| The components: list › Capacitors and inductors: dynamical bipoles <sub>example</sub> | 8 | 8 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Capacitors and inductors: dynamical bipoles <sub>path</sub> | 18 | 18 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Chips (integrated circuits) <sub>example</sub> | 6 | 6 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Chips (integrated circuits) <sub>node</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Connectors <sub>example</sub> | 8 | 8 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Connectors <sub>node</sub> | 6 | 6 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Connectors <sub>path</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Crossings <sub>example</sub> | 4 | 4 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Crossings <sub>node</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Crossings <sub>path</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Diodes and such <sub>example</sub> | 6 | 6 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Diodes and such <sub>path</sub> | 70 | 70 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Double bipoles (transformers) <sub>example</sub> | 12 | 12 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Double bipoles (transformers) <sub>node</sub> | 9 | 9 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Electro-Mechanical Devices <sub>example</sub> | 3 | 3 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Electro-Mechanical Devices <sub>node</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Electronic Tubes <sub>node</sub> | 12 | 12 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Flip-flops <sub>example</sub> | 4 | 4 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Flip-flops <sub>node</sub> | 12 | 12 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Grounds and supply voltages <sub>example</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| The components: list › Grounds and supply voltages <sub>node</sub> | 12 | 12 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Instruments <sub>example</sub> | 18 | 18 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| The components: list › Instruments <sub>path</sub> | 13 | 13 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Logic gates <sub>node</sub> | 34 | 34 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Logic gates <sub>path</sub> | 6 | 6 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Mechanical Analogy <sub>path</sub> | 5 | 5 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Miscellaneous bipoles and symbols <sub>node</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Miscellaneous bipoles and symbols <sub>path</sub> | 23 | 23 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Multiple wires (buses) <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Multiple wires (buses) <sub>path</sub> | 3 | 3 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Multiplexer and de-multiplexer <sub>node</sub> | 6 | 6 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › RF components <sub>example</sub> | 3 | 3 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| The components: list › RF components <sub>node</sub> | 14 | 14 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › RF components <sub>path</sub> | 3 | 3 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Resistive bipoles <sub>path</sub> | 24 | 24 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Seven segment displays <sub>example</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Sources and generators <sub>path</sub> | 38 | 38 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Switches, buttons and jumpers <sub>example</sub> | 19 | 19 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Switches, buttons and jumpers <sub>node</sub> | 9 | 9 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Switches, buttons and jumpers <sub>path</sub> | 32 | 32 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Terminal shapes <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: list › Terminal shapes <sub>node</sub> | 7 | 7 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0.99 |
| The components: list › Transistors <sub>node</sub> | 72 | 72 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: usage › Node-style components <sub>example</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: usage › Node-style components <sub>node</sub> | 4 | 4 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: usage › Path-style components <sub>path</sub> | 3 | 3 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| The components: usage › Styling circuits and components <sub>example</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Tutorials › A logic circuit <sub>example</sub> | 3 | 3 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Tutorials › A non-inverting op-amp amplifier <sub>example</sub> | 5 | 5 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Tutorials › A non-inverting op-amp amplifier <sub>node</sub> | 1 | 1 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Tutorials › A transistor-based amplifier <sub>example</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Using bipoles in circuits › Line joins between Path Components <sub>example</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Using bipoles in circuits › Nodes (also called poles) <sub>example</sub> | 19 | 19 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |
| Using bipoles in circuits › Putting them together <sub>example</sub> | 2 | 2 (100.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1.00 |

## Components

The manual's list of components, group by group, with the ones voltik draws as TeX does (and, in italics, the close ones).

| Group | Components | Match | Close | Drawn as TeX does |
|---|---:|---:|---:|---|
| Tutorials › A non-inverting op-amp amplifier | 1 | 1 | 0 | `op amp` |
| The components: usage › Path-style components | 3 | 3 | 0 | `resistor` `pR` `ammeter` |
| The components: usage › Node-style components | 4 | 4 | 0 | `cute spdt down arrow` `npn` `plain amp` `rotaryswitch` |
| The components: list › Grounds and supply voltages | 12 | 12 | 0 | `ground` `tlground` `rground` `sground` `tground` `nground` `pground` `cground` `eground` `eground2` `vcc` `vee` |
| The components: list › Resistive bipoles | 24 | 24 | 0 | `short` `generic` `xgeneric` `sgeneric` `singeneric` `tgeneric` `ageneric` `memristor` `R` `vR` `pR` `sR` `ldR` `R` `vR` `pR` `sR` `ldR` `varistor` `mov` `phR` `thR` `thRp` `thRn` |
| The components: list › Capacitors and inductors: dynamical bipoles | 18 | 18 | 0 | `capacitor` `curved capacitor` `ecapacitor` `variable capacitor` `capacitive sensor` `piezoelectric` `cpe` `feC` `L` `vL` `sL` `L` `vL` `sL` `L` `vL` `sL` `cute choke` |
| The components: list › Diodes and such | 70 | 70 | 0 | `empty diode` `empty Schottky diode` `empty Zener diode` `empty ZZener diode` `empty tunnel diode` `empty photodiode` `empty led` `empty laser diode` `empty varcap` `empty TVS diode` `empty Shockley diode` `empty bidirectionaldiode` `full diode` `full Schottky diode` `full Zener diode` `full ZZener diode` `full tunnel diode` `full photodiode` `full led` `full laser diode` `full varcap` `full TVS diode` `full Shockley diode` `full bidirectionaldiode` `stroke diode` `stroke Schottky diode` `stroke Zener diode` `stroke ZZener diode` `stroke tunnel diode` `stroke photodiode` `stroke led` `stroke laser diode` `stroke varcap` `stroke TVS diode` `full diode` `stroke diode` `triac` `empty triac` `full triac` `thyristor` `empty thyristor` `full thyristor` `stroke thyristor` `put` `empty put` `full put` `stroke put` `gto` `empty gto` `full gto` `stroke gto` `gtobar` `empty gtobar` `full gtobar` `stroke gtobar` `agtobar` `empty agtobar` `full agtobar` `stroke agtobar` `igct` `empty igct` `full igct` `stroke igct` `triac` `thyristor` `put` `gto` `gtobar` `agtobar` `igct` |
| The components: list › Sources and generators | 38 | 38 | 0 | `battery` `battery1` `battery2` `solar` `baertty` `european voltage source` `cute european voltage source` `american voltage source` `european current source` `cute european current source` `american current source` `sinusoidal voltage source` `sinusoidal current source` `european controlled voltage source` `cute european controlled voltage source` `american controlled voltage source` `european controlled current source` `cute european controlled current source` `american controlled current source` `empty controlled source` `controlled sinusoidal voltage source` `controlled sinusoidal current source` `noise voltage source` `noise current source` `square voltage source` `vsourcetri` `esource` `pvsource` `pvmodule` `ioosource` `voosource` `oosourceatrans` `oosourcetrans` `ooosource` `nullator` `norator` `dcvsource` `dcisource` |
| The components: list › Instruments | 13 | 13 | 0 | `rmeter` `rmeterwa` `ammeter` `voltmeter` `ohmmeter` `smeter` `qiprobe` `qvprobe` `qpprobe` `oscope` `iloop` `iloop2` `currtap` |
| The components: list › Mechanical Analogy | 5 | 5 | 0 | `damper` `inerter` `spring` `viscoe` `mass` |
| The components: list › Miscellaneous bipoles and symbols | 25 | 25 | 0 | `thermocouple` `fuse` `afuse` `wfuse` `relais` `squid` `barrier` `openbarrier` `european gas filled surge arrester` `american gas filled surge arrester` `lamp` `bulb` `wbulb` `neonlampcc` `neonlampac` `sparkgap` `sparkgap, sparkgap/circle` `sparkgap, sparkgap/dot, sparkgap/circle` `loudspeaker` `mic` `tlmic` `buzzer` `rbuzzer` `dc symbol` `ac symbol` |
| The components: list › Multiple wires (buses) | 3 | 3 | 0 | `multiwire` `bmultiwire` `tmultiwire` |
| The components: list › Crossings | 3 | 3 | 0 | `crossing` `jump crossing` `plain crossing` |
| The components: list › Arrows (fake and real) | 4 | 4 | 0 | `currarrow` `inputarrow` `trarrow` `flowarrow` |
| The components: list › Terminal shapes | 7 | 7 | 0 | `circ` `ocirc` `diamondpole` `odiamondpole` `squarepole` `osquarepole` `rectjoinfill` |
| The components: list › Connectors | 7 | 7 | 0 | `bnc` `iec connector` `iecconnshape` `iecsocketL` `iecplugR` `iecplugL` `iecsocketR` |
| The components: list › Block diagram components | 77 | 77 | 0 | `mixer` `mixer, boxed` `adder` `oscillator` `circulator` `wilkinson` `splitter` `genericsplitter` `mzm` `twoport` `twoportsplit` `vco` `vco,box` `bandpass` `bandstop` `highpass` `lowpass` `highpass2` `lowpass2` `allpass` `vallpass` `bgenerator` `qgenerator` `cgenerator` `ngenerator` `adc` `dac` `dsp` `fft` `amp` `amp, boxed` `iamp` `iamp, boxed` `vamp` `vamp, boxed` `piattenuator` `vpiattenuator` `tattenuator` `vtattenuator` `phaseshifter` `vphaseshifter` `biast` `sinetable` `register` `detector` `saturation` `sigmoid` `allornothing` `fiber` `sdcdc` `sacdc` `sdcac` `sacac` `tacdc` `tdcac` `tacac` `gridnode` `lowpassp` `highpassp` `bandpassp` `bandstopp` `resonantp` `notchp` `lowpassp, ideal filter` `highpassp, ideal filter` `bandpassp, ideal filter` `bandstopp, ideal filter` `resonantp, ideal filter` `notchp, ideal filter` `camera` `tvset` `trx` `swr` `power` `fourport` `coupler` `coupler2` |
| The components: list › Transistors | 72 | 72 | 0 | `npn` `pnp` `npn, schottky base` `pnp, schottky base` `npn, bodydiode` `npn,photo` `pnp,photo` `nigbt` `pigbt` `Lnigbt` `Lpigbt` `Lpigbt, bodydiode` `bjtnpn, collectors=1, emitters=2` `bjtpnp, collectors=3, emitters=2` `nmos` `pmos` `nmosd` `pmosd` `hemt` `hemt, nobase` `GaN hemt` `nfet` `nfetd` `nigfete` `nigfete,solderdot` `nigfetebulk` `nigfetd` `pfet` `pfetd` `pigfete` `pigfetebulk` `pigfetd` `nigfete, doublegate` `nigfete,solderdot, doublegate` `nigfetebulk, doublegate` `nigfetd, doublegate` `pigfete, doublegate` `pigfetebulk, doublegate` `pigfetd, doublegate` `nigfetebulk, fdsoi` `nfet, fdsoi` `pigfetebulk, fdsoi` `pfet, fdsoi` `nigfete, fdsoi` `pigfete, fdsoi, doublegate, solderdot` `njfet` `pjfet` `nujt` `pujt` `nujt, nobase` `isfet` `ngfet` `pgfet` `nmos` `pmos` `nmosd` `pmosd` `pmos,emptycircle` `pmos,nocircle,arrowmos` `nmos, bulk` `pmos, bulk` `nmosd, bulk` `pmosd, bulk` `nmos, bulk, fdsoi` `pmos, bulk, fdsoidot` `nmosd, bulk, fdsoidot` `pmosd, bulk, fdsoi` `nigbt` `pigbt, nobase` `bjtnpn, collectors=2, emitters=2, bjt pins width=0, bjt multi height=0.8` `npn, tr circle` `npn, tr circle, bodydiode` |
| The components: list › Electronic Tubes | 12 | 12 | 0 | `diodetube` `triode` `tetrode` `pentode` `pentode suppressor to cathode` `diodetube,filament` `diodetube,filament,nocathode` `diodetube,fullcathode` `matube` `matube, nixieanode, anodedot, nogrid` `magnetron` `dynode` |
| The components: list › RF components | 17 | 17 | 0 | `bareantenna` `dinantenna` `bareTXantenna` `bareRXantenna` `waves` `harmonics` `mstline` `mslstub` `msport` `msrstub` `antenna` `rxantenna` `txantenna` `tlinestub` `TL` `TL, bipoles/tline/bare=true` `match` |
| The components: list › Electro-Mechanical Devices | 2 | 2 | 0 | `elmech` `elmech` |
| The components: list › Double bipoles (transformers) | 9 | 9 | 0 | `transformer` `transformer` `transformer` `gyrator` `transformer core` `transformer core` `transformer core` `double bipole` `double bipole` |
| The components: list › Amplifiers | 11 | 11 | 0 | `op amp` `en amp` `fd op amp` `gm amp` `inst amp` `fd inst amp` `inst amp ra` `plain amp` `plain mono amp` `buffer` `plain gm mono amp` |
| The components: list › Switches, buttons and jumpers | 41 | 41 | 0 | `switch` `closing switch` `opening switch` `normal open switch` `normal closed switch` `opening normal closed switch` `closing normal closed switch` `opening normal open switch` `closing normal open switch` `push button` `normally closed push button` `normally open push button closed` `normally closed push button open` `toggle switch` `reed` `spdt` `cute closed switch` `cute open switch` `cute closing switch` `cute opening switch` `cute spdt up` `cute spdt mid` `cute spdt down` `cute spdt up arrow` `cute spdt mid arrow` `cute spdt down arrow` `proximeter` `inline proximeter` `rotaryswitch` `bare jumper` `open jumper` `closed jumper` `three-pins jumper` `three-pins jumper` `three-pins jumper` `open solder jumper` `closed solder jumper` `open double solder jumper` `left double solder jumper` `right double solder jumper` `closed double solder jumper` |
| The components: list › Logic gates | 40 | 40 | 0 | `american and port` `american or port` `american nand port` `american nor port` `american xor port` `american xnor port` `american buffer port` `american not port` `schmitt` `invschmitt` `ieeestd and port` `ieeestd nand port` `ieeestd or port` `ieeestd nor port` `ieeestd xor port` `ieeestd xnor port` `ieeestd buffer port` `ieeestd not port` `ieeestd schmitt port` `ieeestd invschmitt port` `ieee tgate` `ieee double tgate` `notcirc` `schmitt symbol` `european and port` `european or port` `european nand port` `european nor port` `european xor port` `european xnor port` `european buffer port` `european not port` `european blank port` `european blank not port` `inline not` `inline buffer` `inline schmitt` `inline invschmitt` `inline tgate` `inline double tgate` |
| The components: list › Flip-flops | 12 | 12 | 0 | `flipflop` `flipflop AB` `latch` `flipflop SR` `flipflop D` `flipflop T` `flipflop JK` `clockwedge` `flipflop JK, dot on notQ` `flipflop JK, add async SR` `flipflop myJK` `flipflop JK, add async SR, external pins width=0` |
| The components: list › Multiplexer and de-multiplexer | 6 | 6 | 0 | `muxdemux` `wedgeinv, scale=2` `circleinv, scale=2` `demux` `one bit adder` `ALU` |
| The components: list › Chips (integrated circuits) | 2 | 2 | 0 | `dipchip` `qfpchip` |
| The components: list › Seven segment displays | 1 | 0 | 0 |  |
| Defining new components › Suggested setup | 1 | 1 | 0 | `damper` |
| Defining new components › Path-style component | 1 | 1 | 0 | `viscoe` |

## What voltik reported

Warnings, by how many items they appear in (the varying part replaced by …):

| Items | Warning |
|---:|---|
| 56 | `option … not supported: ignored` |
| 1 | `voltik: oscilloscope waveform mywave not drawn yet; drawn as ramps` |
| 1 | `E-REC-N column N: unsupported text command: \color; written as is` |
| 1 | `\draw: figure: unknown anchor: QN.circle bottom; statement ignored` |
| 1 | `\draw: figure: unknown anchor: QN.circle top; statement ignored` |
| 1 | `\draw: figure: unknown anchor: T-LN.midtap; statement ignored` |
| 1 | `voltik: european ports font on a european nand port not drawn yet; its symbol is in the pl` |
| 1 | `image not drawn: file … not found` |
| 1 | `voltik: pole \p is not drawn yet` |

Components voltik does not know yet, by how many items use them:

| Items | Component |
|---:|---|

Other options voltik does not know, likewise:

| Items | Option |
|---:|---|
| 8 | `color` |
| 7 | `dash` |
| 5 | `relative thickness` |
| 3 | `opto end arrow` |
| 3 | `thickness` |
| 3 | `partial borders` |
| 2 | `arrow` |
| 2 | `angle` |
| 2 | `nodes width` |
| 2 | `inner blocks dashed` |
| 2 | `dashed blocks pattern` |
| 2 | `nocircle` |
| 2 | `mos style` |
| 2 | `minus` |
| 2 | `plus` |
| 2 | `en amp text A` |
| 2 | `switch end arrow` |
| 2 | `switch start arrow` |
| 2 | `open poles fill` |
| 1 | `use fpu reciprocal` |
| 1 | `pic anchor` |
| 1 | `tunable end arrow` |
| 1 | `wiper end arrow` |
| 1 | `tunable start arrow` |
| 1 | `onelinechoke` |
| 1 | `cthick` |
| 1 | `twolineschoke` |
| 1 | `gto gate end arrow` |
| 1 | `opto start arrow` |
| 1 | `inner minus` |
| 1 | `margin` |
| 1 | `inner plus` |
| 1 | `sign rotation` |
| 1 | `index position` |
| 1 | `current arrow scale` |
| 1 | `^-^f` |
| 1 | `vf-v` |
| 1 | `-v` |
| 1 | `-^` |
| 1 | `blocks dc out segments` |

Errors of the items that fail:

| Items | Error |
|---:|---|
