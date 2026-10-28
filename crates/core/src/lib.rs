//! voltik: CircuiTikZ circuits to SVG and PNG, with no TeX install.
//!
//! voltik is a library of [tikra](https://docs.rs/tikra) (`tikra::extension`): the CircuiTikZ keys
//! and components on top of tikra's TikZ. What it draws follows the CircuiTikZ manual.
//!
//! ```
//! let r = voltik::convert(r"\begin{circuitikz} \draw (0,0) to[R=$R_1$, o-*] (2,0); \end{circuitikz}").unwrap();
//! assert!(r.svg.starts_with("<svg"));
//! ```

pub mod bipole;
pub mod components;
pub mod label;

use tikra::extension::{Context, KeyFn, Library, Tokens};
pub use tikra::{png, Error, Locale, Options, Output, MAX_PIXELS};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The CircuiTikZ library, to give tikra next to other libraries.
pub static LIBRARY: Library = Library { name: "voltik", definitions, keys: KEYS };

const KEYS: &[(&str, KeyFn)] = &[
    ("/tikz/voltik/component", bipole::k_component),
    ("/tikz/voltik/place", bipole::k_place),
    ("/tikz/voltik/pole", bipole::k_pole),
    ("/tikz/voltik/flag", bipole::k_flag),
    ("/tikz/voltik/label", label::k_label),
    ("/tikz/voltik/ctikzset", k_ctikzset),
];

/// The settings each class of components has (`resistors/scale`), with their initial values.
const CLASS_SETTINGS: &[(&str, &str)] = &[("scale", "1"), ("width", ""), ("height", ""), ("thickness", "none"), ("modifier thickness", "1")];

/// `\ctikzset{…}`: the keys under `/tikz/circuitikz`; a style defined there (`tallR/.style={…}`) is
/// also a style of TikZ, to be used in a `to[…]` as the manual does.
fn k_ctikzset(cx: &mut Context, v: Option<Tokens>) -> Result<(), Error> {
    let list = v.unwrap_or_default();
    cx.keys_at("/tikz/circuitikz", list.clone())?;
    let keys = list.key_list();
    if keys.iter().any(|(k, _)| k.ends_with(".cd")) {
        return Ok(());
    }
    for (key, value) in keys {
        let handler = [".style", ".append style", ".prefix style", ".code"].iter().any(|h| key.ends_with(&format!("/{h}")));
        if handler && key.matches('/').count() == 1 {
            cx.keys(Tokens::text(&format!("{key}=")).and(value.unwrap_or_default().braced()))?;
        }
    }
    Ok(())
}

/// The poles a `to` can end in, by their shorthand: none, `o` (open circle), `*` (filled circle) and
/// `d` (filled diamond).
const POLES: &[(&str, &str)] = &[("", "none"), ("o", "ocirc"), ("*", "circ"), ("d", "diamondpole")];

/// The settings of `\ctikzset`, the styles and the component keys, as TeX.
fn definitions() -> String {
    let mut s = String::new();
    s.push_str("\\def\\ctikzset#1{\\pgfkeys{/tikz/voltik/ctikzset={#1}}}\n");
    s.push_str("\\def\\ctikzvalof#1{\\pgfkeysvalueof{/tikz/circuitikz/#1}}\n");
    s.push_str(
        "\\pgfkeys{/tikz/circuitikz/.cd,\n\
         bipoles/length/.initial=1.4cm, bipoles/thickness/.initial=2,\n\
         resistors/zigs/.initial=3, inductors/coils/.initial=, wiper pos/.initial=0.5, quadpoles style/.initial=, nodes width/.initial=0.04,\n\
         bipoles/inductors/core distance/.initial=2pt, bipoles/inductors/dot x distance/.initial=4pt, bipoles/inductors/dot y distance/.initial=1pt,\n\
         resistor/.initial=american, inductor/.initial=cute,\n\
         label/align/.initial=smart, label distance/.initial=0pt, annotation distance/.initial=0pt,\n\
         american/.style={/tikz/american}, european/.style={/tikz/european},\n\
         american resistors/.style={/tikz/american resistors}, european resistors/.style={/tikz/european resistors},\n\
         american inductors/.style={/tikz/american inductors}, european inductors/.style={/tikz/european inductors},\n\
         cute inductors/.style={/tikz/cute inductors}, .search also={/tikz},\n\
         straightlabels/.style={label/align=straight}, rotatelabels/.style={label/align=rotate}, smartlabels/.style={label/align=smart},\n\
         }\n",
    );
    let mut keys: Vec<String> = vec![
        "american resistors/.style={/tikz/circuitikz/resistor=american}".into(),
        "european resistors/.style={/tikz/circuitikz/resistor=european}".into(),
        "american inductors/.style={/tikz/circuitikz/inductor=american}".into(),
        "european inductors/.style={/tikz/circuitikz/inductor=european}".into(),
        "cute inductors/.style={/tikz/circuitikz/inductor=cute}".into(),
        "american/.style={american resistors, american inductors}".into(),
        "european/.style={european resistors, european inductors}".into(),
        "straight labels/.style={/tikz/circuitikz/label/align=straight}".into(),
        "rotate labels/.style={/tikz/circuitikz/label/align=rotate}".into(),
        "smart labels/.style={/tikz/circuitikz/label/align=smart}".into(),
        "mirror/.style={voltik/flag=mirror}".into(),
        "invert/.style={voltik/flag=invert}".into(),
        "label distance/.style={/tikz/circuitikz/label distance={#1}}".into(),
        "annotation distance/.style={/tikz/circuitikz/annotation distance={#1}}".into(),
        "wiper pos/.style={/tikz/circuitikz/wiper pos={#1}}".into(),
    ];
    for (key, _) in components::ALIASES {
        keys.push(format!("{key}/.style={{voltik/component={{{key}}}{{#1}}}}"));
    }
    for (key, which, side) in [("l", "l", 0), ("l^", "l", 1), ("l_", "l", -1), ("a", "a", 0), ("a^", "a", 1), ("a_", "a", -1)] {
        keys.push(format!("{key}/.style={{voltik/label={{{which}}}{{{side}}}{{#1}}}}"));
    }
    for (a, start) in POLES {
        for (b, end) in POLES {
            if a.is_empty() && b.is_empty() {
                continue;
            }
            keys.push(format!("{a}-{b}/.style={{voltik/pole={{{start}}}{{{end}}}}}"));
        }
    }
    keys.push("bipole nodes/.style 2 args={voltik/pole={#1}{#2}}".into());
    // the settings every class has, and the settings a `to[…]` can give too
    s.push_str("\\pgfkeys{/tikz/circuitikz/.cd");
    let mut settings = vec!["bipoles/length".to_string(), "bipoles/thickness".into(), "resistors/zigs".into(), "inductors/coils".into()];
    for class in ["resistors", "capacitors", "inductors"] {
        for (key, initial) in CLASS_SETTINGS {
            s.push_str(&format!(",\n{class}/{key}/.initial={initial}"));
            settings.push(format!("{class}/{key}"));
        }
    }
    s.push_str("}\n");
    for key in settings {
        keys.push(format!("{key}/.style={{/tikz/circuitikz/{key}={{#1}}}}"));
    }
    // the `fill` of a `to` (or of its path) fills its component too: TikZ's own `fill`, kept under
    // another name, after noting its value
    s.push_str("\\expandafter\\let\\csname pgfk@/tikz/voltik@fill/.@cmd\\expandafter\\endcsname\\csname pgfk@/tikz/fill/.@cmd\\endcsname\n");
    s.push_str("\\tikzset{fill/.code={\\def\\voltik@fill{#1}\\pgfkeys{/tikz/voltik@fill={#1}}}}\n");
    // and its `name` names the component, whose anchors are then coordinates (`R1.north`)
    s.push_str("\\expandafter\\let\\csname pgfk@/tikz/voltik@name/.@cmd\\expandafter\\endcsname\\csname pgfk@/tikz/name/.@cmd\\endcsname\n");
    s.push_str("\\tikzset{name/.code={\\def\\voltik@name{#1}\\pgfkeys{/tikz/voltik@name={#1}}}}\n");
    s.push_str("\\tikzset{\n");
    for k in keys {
        s.push_str(&k);
        s.push_str(",\n");
    }
    s.push_str("}\n");
    // the circuit's own defaults, reset at each `to`
    s.push_str(
        "\\def\\voltik@kind{short}\\def\\voltik@ps{}\\def\\voltik@pe{}\\def\\voltik@mirror{0}\\def\\voltik@invert{0}\\def\\voltik@l{}\\def\\voltik@a{}\\def\\voltik@fill{}\\def\\voltik@name{}\n",
    );
    s
}

/// Converts a CircuiTikZ figure (a `circuitikz` environment, or any TikZ figure using its
/// components) to SVG.
pub fn convert(code: &str) -> Result<Output, Error> {
    convert_with(code, &Options::default())
}

/// [`convert`] with tikra's options (the language of the figure in words, the animation).
pub fn convert_with(code: &str, options: &Options) -> Result<Output, Error> {
    tikra::convert_with_libraries(&environments(code), options, &[&LIBRARY])
}

/// `circuitikz` environments as the `tikzpicture`s they are.
fn environments(code: &str) -> String {
    code.replace("\\begin{circuitikz}", "\\begin{tikzpicture}").replace("\\end{circuitikz}", "\\end{tikzpicture}")
}
