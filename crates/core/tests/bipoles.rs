//! Path-style components: each one draws, with its leads, labels and poles, and follows the settings
//! of the manual.

fn convert(body: &str) -> voltik::Output {
    voltik::convert(&format!("\\begin{{circuitikz}}{body}\\end{{circuitikz}}")).unwrap()
}

/// The `d` of every stroked path of the SVG.
fn paths(svg: &str) -> Vec<String> {
    svg.split("<path d=\"").skip(1).map(|p| p[..p.find('"').unwrap()].to_string()).collect()
}

fn quiet(body: &str) -> voltik::Output {
    let r = convert(body);
    assert!(r.warnings.is_empty(), "{body}: {:?}", r.warnings);
    r
}

#[test]
fn every_component_draws_without_warnings() {
    for key in ["R", "resistor", "american resistor", "european resistor", "generic", "C", "capacitor", "L", "inductor", "cute inductor", "short", "open"] {
        quiet(&format!("\\draw (0,0) to[{key}] (2,0);"));
    }
}

#[test]
fn a_component_is_its_outline_and_two_leads() {
    let r = quiet(r"\draw (0,0) to[R] (2,0);");
    let p = paths(&r.svg);
    assert_eq!(p.len(), 2, "{p:?}");
    // the zig-zag: a start, six teeth and an end
    assert!(p.iter().any(|d| d.matches(" L").count() == 7), "{p:?}");
    // the leads: two segments
    assert!(p.iter().any(|d| d.matches('M').count() == 2 && d.matches('L').count() == 2), "{p:?}");
}

#[test]
fn short_and_open_have_no_outline() {
    assert_eq!(paths(&quiet(r"\draw (0,0) to[short] (2,0);").svg).len(), 1);
    // an open draws nothing at all but its poles
    let open = quiet(r"\draw (0,0) to[open, o-o] (2,0);");
    assert_eq!(paths(&open.svg).iter().filter(|d| d.contains(" L")).count(), 0);
}

#[test]
fn the_style_chooses_the_resistor() {
    let american = paths(&quiet(r"\draw (0,0) to[R] (2,0);").svg);
    let european = paths(&quiet(r"\ctikzset{resistor=european} \draw (0,0) to[R] (2,0);").svg);
    let styled = paths(&convert(r"\draw[european] (0,0) to[R] (2,0);").svg);
    assert_ne!(american, european);
    assert_eq!(european, styled);
    assert!(european.iter().any(|d| d.ends_with('Z')), "{european:?}");
}

#[test]
fn settings_change_the_shape() {
    let teeth = |body: &str| paths(&quiet(body).svg).iter().map(|d| d.matches(" L").count()).max().unwrap();
    assert_eq!(teeth(r"\draw (0,0) to[R] (2,0);"), 7);
    assert_eq!(teeth(r"\ctikzset{resistors/zigs=6} \draw (0,0) to[R] (2,0);"), 13);
}

#[test]
fn labels_and_units() {
    let r = quiet(r"\draw (0,0) to[R=$R_1$] (2,0) to[C, l_=$C$, a=1<\micro\farad>] (2,-2);");
    // three labels: R_1, C and the annotation
    assert_eq!(r.svg.matches("<g transform=").count(), 3, "{}", r.svg);
}

#[test]
fn poles_are_circles_over_the_wire() {
    let r = quiet(r"\draw (0,0) to[R, o-*] (2,0);");
    let p = paths(&r.svg);
    assert_eq!(p.iter().filter(|d| d.contains(" C")).count(), 2, "{p:?}");
    assert!(r.svg.contains("fill=\"#ffffff\""));
}

#[test]
fn mirror_and_invert_keep_the_leads_outside() {
    for opts in ["", ", mirror", ", invert", ", mirror, invert"] {
        let r = quiet(&format!(r"\draw (0,0) to[R{opts}] (0,2);"));
        let leads = paths(&r.svg).into_iter().find(|d| d.matches('M').count() == 2).unwrap();
        // two vertical segments that do not overlap: the first ends below where the second starts
        let ys: Vec<f64> = leads.split(['M', 'L']).filter_map(|s| s.split_whitespace().nth(1)?.parse().ok()).collect();
        assert!(ys[1] > ys[2], "{opts}: {leads}");
    }
}

#[test]
fn components_work_in_a_plain_tikzpicture() {
    let r = voltik::convert(r"\begin{tikzpicture}\draw (0,0) to[L=$L$] (2,0);\end{tikzpicture}").unwrap();
    assert!(r.warnings.is_empty(), "{:?}", r.warnings);
}
