//! Amplifiers: each node draws with its anchors and its `+` and `-`, and `noinv input up` swaps them.

fn convert(body: &str) -> voltik::Output {
    voltik::convert(&format!("\\begin{{circuitikz}}{body}\\end{{circuitikz}}")).unwrap()
}

fn quiet(body: &str) -> voltik::Output {
    let r = convert(body);
    assert!(r.warnings.is_empty(), "{body}: {:?}", r.warnings);
    r
}

/// The `d` of every path of the SVG.
fn paths(svg: &str) -> Vec<String> {
    svg.split("<path d=\"").skip(1).map(|p| p[..p.find('"').unwrap()].to_string()).collect()
}

#[test]
fn every_amplifier_draws_with_its_anchors() {
    for amp in ["op amp", "fd op amp", "en amp", "gm amp", "inst amp", "inst amp ra", "fd inst amp", "plain amp"] {
        quiet(&format!(r"\draw (0,0) node[{amp}] (A) {{$A$}} (A.+) -- (A.-) -- (A.out) -- (A.up) -- (A.down);"));
    }
    for amp in ["plain mono amp", "plain gm mono amp", "buffer"] {
        quiet(&format!(r"\draw (0,0) node[{amp}] (A) {{}} (A.in) -- (A.out) -- (A.bin) -- (A.bout);"));
    }
    quiet(r"\draw (0,0) node[fd op amp] (A) {} (A.out +) -- (A.out -);");
    quiet(r"\draw (0,0) node[inst amp ra] (A) {} (A.ra+) -- (A.ra-) -- (A.refv up);");
}

#[test]
fn the_signs_are_drawn_and_swap() {
    let plain = paths(&quiet(r"\draw (0,0) node[op amp] {};").svg);
    let swapped = paths(&quiet(r"\draw (0,0) node[op amp, noinv input up] {};").svg);
    // the triangle and its leads, the minus and the plus
    assert!(plain.len() >= 4, "{plain:?}");
    assert_ne!(plain, swapped);
    // the anchors swap with them
    let at = |opts: &str| paths(&quiet(&format!(r"\draw (0,0) node[{opts}] (A) {{}}; \draw[red] (A.+) -- (0,0);")).svg).last().cloned();
    assert_ne!(at("op amp"), at("op amp, noinv input up"));
}
