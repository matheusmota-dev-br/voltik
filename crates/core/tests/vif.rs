//! Voltages, currents and flows: every variant draws, in every style, where the manual puts it.

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

/// The numbers of a path, in order.
fn numbers(d: &str) -> Vec<f64> {
    d.split(|c: char| c.is_ascii_alphabetic() || c == ' ').filter_map(|t| t.parse().ok()).collect()
}

#[test]
fn every_variant_draws_in_every_style() {
    for kind in ["v", "i", "f"] {
        for variant in ["", "^", "_", ">", "<", "^>", "^<", "_>", "_<", ">^", ">_", "<^", "<_"] {
            for style in ["european voltages", "straight voltages", "american voltages", "raised voltages"] {
                for dir in ["RP", "EF"] {
                    quiet(&format!("\\draw[{style}, voltage dir={dir}] (0,0) to[R, {kind}{variant}=$x$] (2,0) to[open, {kind}{variant}=$y$] (2,2);"));
                }
            }
        }
    }
}

#[test]
fn a_current_is_an_arrow_head_on_a_lead() {
    // the label and the head: a filled triangle after the component, or before it with `>_`
    let tip_x = |opts: &str| {
        let svg = quiet(&format!(r"\draw (0,0) to[R, {opts}=$i$] (4,0);")).svg;
        let heads: Vec<Vec<f64>> = paths(&svg).iter().filter(|d| d.matches('L').count() == 2 && d.ends_with('Z')).map(|d| numbers(d)).collect();
        assert_eq!(heads.len(), 1, "{opts}: {svg}");
        heads[0][2]
    };
    let end = tip_x("i");
    let start = tip_x("i>_");
    // 4 cm is 113.4 bp: the end lead is past the middle, the start lead before it
    assert!(end > 80.0 && start < 40.0, "{end} {start}");
}

#[test]
fn the_voltage_direction_follows_the_convention() {
    // the head of a European voltage is at the start with rising-potential voltages, at the end with
    // electric-field ones (the package's default)
    let head = |dir: &str| {
        let svg = quiet(&format!(r"\draw[voltage dir={dir}] (0,0) to[R, v=$v$] (4,0);")).svg;
        let heads: Vec<Vec<f64>> = paths(&svg).iter().filter(|d| d.matches('L').count() == 2 && d.ends_with('Z')).map(|d| numbers(d)).collect();
        heads[0][2]
    };
    assert!(head("RP") < 56.0, "{}", head("RP"));
    assert!(head("EF") > 56.0, "{}", head("EF"));
    // and the document's options choose it
    let doc = |opts: &str| {
        voltik::convert(&format!(
            "\\documentclass{{standalone}}\\usepackage[{opts}]{{circuitikz}}\\begin{{document}}\\begin{{circuitikz}}\\draw (0,0) to[R, v=$v$] (4,0);\\end{{circuitikz}}\\end{{document}}"
        ))
        .unwrap()
        .svg
    };
    assert_ne!(paths(&doc("RPvoltages")), paths(&doc("EFvoltages")));
}

#[test]
fn american_voltages_are_signs() {
    let r = quiet(r"\draw[american voltages] (0,0) to[R, v=$v$] (2,0);");
    // three texts (+, - and the label), no arrow
    assert_eq!(r.svg.matches("<g transform=").count(), 3, "{}", r.svg);
    assert!(!paths(&r.svg).iter().any(|d| d.contains(" C")), "{}", r.svg);
}

#[test]
fn anchors_label_nodes_and_symbols() {
    // the anchors of a named component's voltage, current and flow, and its labels' nodes
    quiet(
        r"\draw (0,0) to[R, l=$R$, v=$v$, i=$i$, f>_=$f$, name=R] (3,0);
          \draw[red] (R-Vfrom) .. controls (R-Vcont1) and (R-Vcont2) .. (R-Vto) (R-Vlab) -- (R-Ipos) -- (R-Ifrom) -- (R-Ito)
            (R-Fpos) -- (R-Ffrom) -- (R-Fto) (Rlabel.north) -- (Rvoltage.south) -- (Rcurrent.center) -- (Rflow.center);
          \node[anchor=\ctikzgetanchor{R}{Vlab}] at (R-Vlab) {x};",
    );
    // without a value only the anchors; without symbols only the label
    let bare = quiet(r"\draw (0,0) to[R, v, name=R] (3,0); \draw (R-Vfrom) -- (R-Vto);");
    assert_eq!(bare.svg.matches("<g transform=").count(), 0);
    let only = quiet(r"\draw (0,0) to[R, v=$v$, no v symbols] (3,0);");
    assert_eq!(only.svg.matches("<g transform=").count(), 1);
    assert!(!paths(&only.svg).iter().any(|d| d.contains(" C")), "{}", only.svg);
}

#[test]
fn settings_move_them() {
    let at = |opts: &str| paths(&quiet(&format!(r"\draw (0,0) to[R, v=$v$, i=$i$, {opts}] (3,0);")).svg);
    let plain = at("");
    for opts in ["voltage shift=1", "voltage/bump b=3", "voltage/distance from node=.2", "current/distance=.2"] {
        assert_ne!(at(opts), plain, "{opts}");
    }
}

#[test]
fn stacked_labels_and_siunitx() {
    let r = quiet(r"\draw (0,0) to[R, l2=$R_1$ and \SI{4.7}{\kilo\ohm}, a=\qty{1}{\milli\ampere}, l2 halign=c] (0,3);");
    // \SI and \qty are written out (`quiet`: no "unsupported text command")
    // two labels, the first of two lines
    assert_eq!(r.svg.matches("<g transform=").count(), 2, "{}", r.svg);
}

#[test]
fn label_anchors_and_directions_as_tex_answers() {
    // what `\ctikzgetanchor` and `\ctikzgetdirection` answer for a label and an annotation, by the
    // line's way and `label/align`, as CircuiTikZ 1.8.7 does
    let cases = [
        ("straight", "(2,0)", "base/0/north/0"),
        ("straight", "(0,-2)", "mid west/0/mid east/0"),
        ("straight", "(-2,0)", "north/0/base/0"),
        ("straight", "(2,1.15)", "-60/0/120/0"),
        ("straight", "(2,-2)", "225/0/45/0"),
        ("straight", "(1,2)", "-27/0/153/0"),
        ("smart", "(0,2)", "mid east/0/mid west/0"),
        ("smart", "(2,1.15)", "mid/30/mid/30"),
        ("smart", "(-2,1)", "mid/-26/mid/-26"),
        ("rotate", "(0,-2)", "mid/270/mid/270"),
        ("rotate", "(-2,0)", "mid/0/mid/0"),
        ("rotate", "(2,-2)", "mid/315/mid/315"),
    ];
    for (align, to, expected) in cases {
        let r = quiet(&format!(
            r"\ctikzset{{label/align={align}}}\draw (0,0) to[R, name=R, l=x, a=y] ++{to};
              \node at (0,-3) {{\ctikzgetanchor{{R}}{{label}}/\ctikzgetdirection{{Rlabel}}/\ctikzgetanchor{{R}}{{annotation}}/\ctikzgetdirection{{Rannotation}}}};"
        ));
        assert!(r.description.contains(expected), "{align} {to}: {}", r.description);
    }
}

#[test]
fn named_components_are_nodes() {
    // `(R)` is the component's center, and a name prefix names it as it names nodes
    let r = quiet(
        r"\begin{scope}[name prefix=in-]\draw (0,0) to[R, name=r, i<] ++(2,1);\end{scope}
          \fill (in-r) circle (1pt) (in-r.north) circle (1pt) (in-r-Ipos) circle (1pt);
          \node at (0,-1) {\ctikzgetdirection{in-r} \ctikzgetdirection{in-r-Iarrow}};",
    );
    assert!(r.description.contains("26.56505 -153"), "{}", r.description);
}

#[test]
fn voltages_of_the_users_own() {
    // `vif queue add` runs a macro with the component's name once it is drawn (named for it when it
    // has no name), and `\ctikzactivatevoltagedirections` gives a style its variants
    let r = quiet(
        r"\newcommand\myv[4]{\draw[red] (#1-Vfrom) -- (#1-Vto) node[anchor=\ctikzgetanchor{#1}{Vlab}] {#2};}
          \ctikzset{myv/.style 2 args={v, vif queue add={myv}{#1}{#2}{}}}
          \ctikzactivatevoltagedirections{myv}
          \draw (0,0) to[R, myv={$v_1$}{}] (3,0) to[R, myv^>={$v_2$}{}, name=R2] (3,3);",
    );
    assert_eq!(r.svg.matches("stroke=\"#ff0000\"").count(), 2, "{}", r.svg);
    assert_eq!(r.svg.matches("<g transform=").count(), 2, "{}", r.svg);
}

#[test]
fn style_files() {
    // `\ctikzloadstyle{example}` gives `example circuit style`
    let plain = quiet(r"\draw (0,0) node[op amp] {};").svg;
    let styled = voltik::convert(r"\ctikzloadstyle{example}\begin{circuitikz}[example circuit style]\draw (0,0) node[op amp] {};\end{circuitikz}").unwrap();
    assert!(styled.warnings.is_empty(), "{:?}", styled.warnings);
    assert_ne!(plain, styled.svg);
    let romano = voltik::convert(r"\ctikzloadstyle{romano}\begin{circuitikz}[romano circuit style]\draw (0,0) to[R] (2,0);\end{circuitikz}").unwrap();
    assert!(romano.warnings.is_empty(), "{:?}", romano.warnings);
}
