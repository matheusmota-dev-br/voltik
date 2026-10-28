# Corpus

Hand-made circuits, one folder per topic, used by the gallery of the site and by `crates/core/tests/corpus.rs`, which checks that each converts without warnings. Each file is a complete input for `voltik`, and each can be drawn by CircuiTikZ itself with `tools/oracle/tex.sh` to compare.

| Folder | What its figures show |
|---|---|
| `circuits/` | Small whole circuits: filters, dividers, ladders, resonant tanks |
| `labels/` | Labels and annotations: sides, units, straight, rotated and smart alignment |
| `styles/` | American and European styles, `\ctikzset` settings, `mirror` and `invert` |

Add a figure by dropping a `.tex` file in its folder (the name, without `.tex`, becomes its title in the gallery: `voltage-divider.tex` is "Voltage divider"); the test and the gallery pick it up. Names are unique across folders.
