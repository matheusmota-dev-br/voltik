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

use tikra::extension::{KeyFn, Library};
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
];

/// The poles a `to` can end in: none, `o` (open) and `*` (filled).
const POLES: &[&str] = &["", "o", "*"];

/// The settings of `\ctikzset`, the styles and the component keys, as TeX.
fn definitions() -> String {
    let mut s = String::new();
    s.push_str("\\def\\ctikzset#1{\\pgfkeys{/tikz/circuitikz/.cd,#1}}\n");
    s.push_str("\\def\\ctikzvalof#1{\\pgfkeysvalueof{/tikz/circuitikz/#1}}\n");
    s.push_str(
        "\\pgfkeys{/tikz/circuitikz/.cd,\n\
         bipoles/length/.initial=1.4cm, bipoles/thickness/.initial=2,\n\
         resistors/scale/.initial=1, resistors/width/.initial=, resistors/zigs/.initial=3, resistors/thickness/.initial=none,\n\
         capacitors/scale/.initial=1, capacitors/width/.initial=, capacitors/thickness/.initial=none,\n\
         inductors/scale/.initial=1, inductors/width/.initial=, inductors/coils/.initial=, inductors/thickness/.initial=none,\n\
         resistor/.initial=american, inductor/.initial=cute,\n\
         label/align/.initial=smart, label distance/.initial=0pt, annotation distance/.initial=0pt,\n\
         american/.style={/tikz/american}, european/.style={/tikz/european},\n\
         american resistors/.style={/tikz/american resistors}, european resistors/.style={/tikz/european resistors},\n\
         cute inductors/.style={/tikz/cute inductors},\n\
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
    ];
    for (key, name) in components::ALIASES {
        keys.push(format!("{key}/.style={{voltik/component={{{name}}}{{#1}}}}"));
    }
    for (key, which, side) in [("l", "l", 0), ("l^", "l", 1), ("l_", "l", -1), ("a", "a", 0), ("a^", "a", 1), ("a_", "a", -1)] {
        keys.push(format!("{key}/.style={{voltik/label={{{which}}}{{{side}}}{{#1}}}}"));
    }
    for a in POLES {
        for b in POLES {
            if a.is_empty() && b.is_empty() {
                continue;
            }
            keys.push(format!("{a}-{b}/.style={{voltik/pole={{{a}}}{{{b}}}}}"));
        }
    }
    s.push_str("\\tikzset{\n");
    for k in keys {
        s.push_str(&k);
        s.push_str(",\n");
    }
    s.push_str("}\n");
    // the circuit's own defaults, reset at each `to`
    s.push_str("\\def\\voltik@kind{short}\\def\\voltik@ps{}\\def\\voltik@pe{}\\def\\voltik@mirror{0}\\def\\voltik@invert{0}\\def\\voltik@l{}\\def\\voltik@a{}\n");
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
