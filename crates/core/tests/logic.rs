//! Logic gates: every port draws with its anchors in its three styles, with more inputs and
//! without leads; the short keys follow `logic ports`; the gates on a path too.

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
fn every_port_draws_with_its_inputs() {
    for style in ["american", "european", "ieeestd"] {
        for gate in ["and", "or", "nand", "nor", "xor", "xnor"] {
            for n in 2..=8 {
                quiet(&format!(r"\draw (0,0) node[{style} {gate} port, number inputs={n}] (G) {{}} (G.in 1) -- (G.in {n}) -- (G.out);"));
            }
            quiet(&format!(r"\draw (0,0) node[{style} {gate} port, no input leads] (G) {{}} (G.in 1) -- (G.out);"));
        }
        quiet(&format!(r"\draw (0,0) node[{style} not port] (G) {{}} (G.in) -- (G.out);"));
    }
    for node in ["schmitt", "invschmitt", "ieee tgate", "ieee double tgate", "notcirc", "schmitt symbol", "european blank port"] {
        quiet(&format!(r"\draw (0,0) node[{node}] {{}};"));
    }
}

#[test]
fn the_short_keys_follow_the_style() {
    let draw = |style: &str| paths(&quiet(&format!(r"\ctikzset{{logic ports={style}}} \draw (0,0) node[nand port] {{}};")).svg);
    assert_eq!(draw("american"), paths(&quiet(r"\draw (0,0) node[american nand port] {};").svg));
    assert_eq!(draw("ieee"), paths(&quiet(r"\draw (0,0) node[ieeestd nand port] {};").svg));
    assert_ne!(draw("european"), draw("american"));
}

#[test]
fn gates_on_a_path_follow_the_style() {
    for key in ["inline not", "inline buffer", "inline schmitt", "inline invschmitt", "inline tgate", "inline double tgate"] {
        quiet(&format!(r"\draw (0,0) to[{key}, n=G] (2,0) (G.in) -- (G.out);"));
    }
    let american = paths(&quiet(r"\draw (0,0) to[inline not] (2,0);").svg);
    let ieee = paths(&quiet(r"\ctikzset{logic ports=ieee} \draw (0,0) to[inline not] (2,0);").svg);
    assert_ne!(american, ieee);
}
