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
pub mod nodes;
pub mod vif;

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
    ("/tikz/voltik/vif", vif::k_vif),
    ("/tikz/voltik/shapes", nodes::k_shapes),
    ("/tikz/voltik/class fill", nodes::k_class_fill),
    ("/tikz/voltik/rotary", nodes::k_rotary),
];

/// The settings each class of components has (`resistors/scale`), with their initial values.
const CLASS_SETTINGS: &[(&str, &str)] = &[("scale", "1"), ("width", ""), ("height", ""), ("thickness", "none"), ("modifier thickness", "1"), ("fill", "")];

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
    // the anchors that labels were placed by, and their rotation
    s.push_str("\\def\\ctikzgetanchor#1#2{\\csname voltik@anchor@#1@#2\\endcsname}\n");
    s.push_str("\\def\\ctikzgetdirection#1{\\csname voltik@direction@#1\\endcsname}\n");
    s.push_str(
        "\\pgfkeys{/tikz/circuitikz/.cd,\n\
         bipoles/length/.initial=1.4cm, bipoles/thickness/.initial=2,\n\
         resistors/zigs/.initial=3, inductors/coils/.initial=, wiper pos/.initial=0.5, quadpoles style/.initial=, nodes width/.initial=0.04,\n\
         bipoles/inductors/core distance/.initial=2pt, bipoles/inductors/dot x distance/.initial=4pt, bipoles/inductors/dot y distance/.initial=1pt,\n\
         resistor/.initial=american, inductor/.initial=cute, diode/.initial=empty, thyristor style/.initial=legacy, led arrows/.initial=anode, pd arrows/.initial=anode, diode whiskers/.initial=sloped, tjumper connections/.initial=00, tubes/anodes/.initial=3, transistor circle/scale circle radius/.initial=1, transistor circle/relative thickness/.initial=1, tr/ferroel gate/.initial=false, transistors/arrow pos/.initial=mid, tr/legacy text/.initial=false, legacy transistors text/.style={tr/legacy text=true}, bipoles/cuteswitch/shape/.initial=ocirc, bipoles/cuteswitch/height/.initial=.6, bipoles/cuteswitch/thickness/.initial=1, bipoles/jumper/shape/.initial=ocirc, multipoles/rotary/channels/.initial=5, multipoles/rotary/angle/.initial=60, multipoles/rotary/wiper/.initial=20,\n\
         multipoles/rotary/arrow/.initial=none, multipoles/rotary/thickness/.initial=1, multipoles/rotary/shape/.initial=ocirc, tripoles/spdt/width/.initial=.85, double bipole L/.initial=, double bipole R/.initial=, db/L invert/.initial=false, db/R invert/.initial=false,\n\
         double bipole L invert/.code={\\pgfkeys{/tikz/circuitikz/db/L invert=true}}, double bipole R invert/.code={\\pgfkeys{/tikz/circuitikz/db/R invert=true}},\n\
         every double bipole L/.style={}, every double bipole R/.style={}, transformer L1/.style={}, transformer L2/.style={}, en amp text/.initial=, component text/.initial=center, left text distance/.initial=.3em, tr/noinv input up/.initial=false, tr/noinv input down/.initial=false, tr/noinv output up/.initial=false,\n\
         tr/noinv output down/.initial=false, tr/filament/.initial=false, tr/nocathode/.initial=false, tr/fullcathode/.initial=false, tr/nixieanode/.initial=false,\n\
         tr/anodedot/.initial=false, tr/nogrid/.initial=false, tr/schottky base/.initial=false, tr/bodydiode/.initial=false, tr/photo/.initial=false, tr/tr circle/.initial=false,\n\
         tr/nobase/.initial=false, tr/bulk/.initial=false, tr/emptycircle/.initial=false, tr/nocircle/.initial=false, tr/arrowmos/.initial=false, tr/solderdot/.initial=false,\n\
         tr/fdsoi/.initial=false, tr/fdsoidot/.initial=false, tr/doublegate/.initial=false, tr/collectors/.initial=, tr/emitters/.initial=, tr/bjt pins width/.initial=, tr/bjt multi height/.initial=,\n\
         led arrows from cathode/.style={led arrows=cathode}, led arrows from anode/.style={led arrows=anode}, pd arrows to cathode/.style={pd arrows=cathode},\n\
         pd arrows to anode/.style={pd arrows=anode}, diode straight whiskers/.style={diode whiskers=straight}, diode sloped whiskers/.style={diode whiskers=sloped}, voltage/.initial=european, current/.initial=european, voltage dir/.initial=noold,\n\
         voltage/distance from node/.initial=, voltage/bump b/.initial=, l2 halign/.initial=l, l2 valign/.initial=c, a2 halign/.initial=l, a2 valign/.initial=c, bipole/is voltage/.initial=true, bipole/is current/.initial=true, bipole/override source vif/.initial=false,\n\
         voltage shift/.initial=0, current/distance/.initial=, flow/distance/.initial=, v symbols/.initial=true, i symbols/.initial=true, f symbols/.initial=true,\n\
         bipole label style/.style={}, bipole annotation style/.style={}, bipole voltage style/.style={}, bipole current style/.style={}, bipole flow style/.style={},\n\
         no v symbols/.style={v symbols=false}, no i symbols/.style={i symbols=false}, no f symbols/.style={f symbols=false},\n\
         voltage/american label distance/.initial=1.4, voltage/american font/.initial=, voltage/american plus/.initial=$+$, voltage/american minus/.initial=$\\vphantom{+}-$,\n\
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
        "empty diodes/.style={/tikz/circuitikz/diode=empty}".into(),
        "full diodes/.style={/tikz/circuitikz/diode=full}".into(),
        "stroke diodes/.style={/tikz/circuitikz/diode=stroke}".into(),
        "led arrows from cathode/.style={/tikz/circuitikz/led arrows=cathode}".into(),
        "led arrows from anode/.style={/tikz/circuitikz/led arrows=anode}".into(),
        "pd arrows to cathode/.style={/tikz/circuitikz/pd arrows=cathode}".into(),
        "pd arrows to anode/.style={/tikz/circuitikz/pd arrows=anode}".into(),
        "diode straight whiskers/.style={/tikz/circuitikz/diode whiskers=straight}".into(),
        "diode sloped whiskers/.style={/tikz/circuitikz/diode whiskers=sloped}".into(),
        "voltage dir/.style={/tikz/circuitikz/voltage dir=#1}".into(),
        "american voltages/.style={/tikz/circuitikz/voltage=american}".into(),
        "american currents/.style={/tikz/circuitikz/current=american}".into(),
        "european currents/.style={/tikz/circuitikz/current=european}".into(),
        "european voltages/.style={/tikz/circuitikz/voltage=european}".into(),
        "straight voltages/.style={/tikz/circuitikz/voltage=straight}".into(),
        "raised voltages/.style={/tikz/circuitikz/voltage=raised}".into(),
        "american/.style={american resistors, american inductors, american voltages, american currents}".into(),
        "european/.style={european resistors, european inductors, european voltages, european currents}".into(),
        "straight labels/.style={/tikz/circuitikz/label/align=straight}".into(),
        "rotate labels/.style={/tikz/circuitikz/label/align=rotate}".into(),
        "smart labels/.style={/tikz/circuitikz/label/align=smart}".into(),
        "mirror/.style={voltik/flag=mirror}".into(),
        "invert/.style={voltik/flag=invert}".into(),
        "label distance/.style={/tikz/circuitikz/label distance={#1}}".into(),
        "annotation distance/.style={/tikz/circuitikz/annotation distance={#1}}".into(),
        "wiper pos/.style={/tikz/circuitikz/wiper pos={#1}}".into(),
        "no v symbols/.style={/tikz/circuitikz/v symbols=false}".into(),
        "bipole label append style/.style={/tikz/circuitikz/bipole label style/.append style={#1}}".into(),
        "bipole annotation append style/.style={/tikz/circuitikz/bipole annotation style/.append style={#1}}".into(),
        "bipole voltage append style/.style={/tikz/circuitikz/bipole voltage style/.append style={#1}}".into(),
        "bipole current append style/.style={/tikz/circuitikz/bipole current style/.append style={#1}}".into(),
        "bipole flow append style/.style={/tikz/circuitikz/bipole flow style/.append style={#1}}".into(),
        "no i symbols/.style={/tikz/circuitikz/i symbols=false}".into(),
        "no f symbols/.style={/tikz/circuitikz/f symbols=false}".into(),
        "v symbols/.style={/tikz/circuitikz/v symbols=true}".into(),
        "i symbols/.style={/tikz/circuitikz/i symbols=true}".into(),
        "f symbols/.style={/tikz/circuitikz/f symbols=true}".into(),
    ];
    // the node-style components, shapes of their own
    for name in nodes::NODES.iter().map(|n| n.name).chain(nodes::TERMINALS.iter().copied()).chain(nodes::variant_shapes()) {
        // and its class's `fill` (`\ctikzset{amplifiers/fill=cyan}`)
        match nodes::find(name).and_then(|n| n.class()) {
            Some(class) if name.starts_with("cute spdt") => keys.push(format!("{name}/.style={{shape={name}, voltik/class fill={class}, append after command={{\\voltik@cs}}}}")),
            Some(class) => keys.push(format!("{name}/.style={{shape={name}, voltik/class fill={class}}}")),
            // a double bipole draws the bipoles of `double bipole L` and `R` after it
            None if name == "double bipole" => keys.push(format!("{name}/.style={{shape={name}, append after command={{\\voltik@db}}}}")),
            // a transformer's coils are nodes of their own (`T1-L1`), and drawn by `transformer L1`
            None if name.starts_with("transformer") => keys.push(format!("{name}/.style={{shape={name}, append after command={{\\voltik@tr}}}}")),
            None => keys.push(format!("{name}/.style={{shape={name}}}")),
        }
    }
    for (key, target) in components::aliases() {
        keys.push(format!("{key}/.style={{voltik/component={{{key}}}{{#1}}{{}}}}"));
        // a source's key takes the variants of its voltage or current (`I<=1A`, `V_>=…`)
        if let components::Target::Fixed(n) | components::Target::Voltage(n, _) | components::Target::Current(n, _) = target {
            if components::find(n).is_some_and(|c| c.source() != components::Source::Not) {
                for variant in ["^", "_", ">", "<", "^>", "^<", "_>", "_<", ">^", ">_", "<^", "<_"] {
                    keys.push(format!("{key}{variant}/.style={{voltik/component={{{key}}}{{#1}}{{{variant}}}}}"));
                }
            }
        }
    }
    for (key, which, side) in [
        ("l", "l", 0),
        ("l^", "l", 1),
        ("l_", "l", -1),
        ("a", "a", 0),
        ("a^", "a", 1),
        ("a_", "a", -1),
        ("l2", "l2", 0),
        ("l2^", "l2", 1),
        ("l2_", "l2", -1),
        ("a2", "a2", 0),
        ("a2^", "a2", 1),
        ("a2_", "a2", -1),
    ] {
        keys.push(format!("{key}/.style={{voltik/label={{{which}}}{{{side}}}{{#1}}}}"));
    }
    // the transistors', tubes' and amplifiers' options (`node[npn, bodydiode]`, `node[op amp, noinv input up]`)
    for flag in nodes::VARIANT_FLAGS {
        keys.push(format!("{flag}/.style={{/tikz/circuitikz/tr/{flag}=#1}}, {flag}/.default=true"));
    }
    for key in nodes::VARIANT_VALUES {
        keys.push(format!("{key}/.style={{/tikz/circuitikz/tr/{key}=#1}}"));
    }
    // `en amp text={…}`: the text of an `en amp` instead of its ▷∞, where that goes
    keys.push(
        "en amp text/.code={\\pgfkeys{/tikz/circuitikz/en amp text={#1}}\\pgfkeys{/tikz/label={[anchor=base,inner sep=0pt,xshift=-0.0235cm,yshift=0.7562cm]center:{#1}}}}".into(),
    );
    // a bipole as a node (`node[resistorshape]`), and inside a double bipole
    keys.push("voltik/fit/.code={\\def\\voltik@fit{1}}".into());
    keys.push("voltik/up/.code={\\def\\voltik@up{#1}}".into());
    keys.push("voltik/bipole node/.code={\\def\\voltik@bn{#1}}".into());
    keys.push("voltik/coil/.code={\\def\\voltik@coil{#1}}".into());
    keys.push("voltik/pole shape/.code={\\def\\voltik@pole{#1}}".into());
    for (shape, key) in components::BIPOLE_SHAPES {
        keys.push(format!("{shape}/.style={{shape=voltik bipole, voltik/bipole node={{{key}}}, append after command={{\\voltik@bp}}}}"));
    }
    // the rotary switch: bare, and by its styles
    keys.push("rotaryswitch/.style={shape=rotaryswitch, voltik/class fill=switches, append after command={\\voltik@rs}}".into());
    for (style, arrow) in [("rotary switch", ""), ("rotary switch -", "none"), ("rotary switch <-", "ccw"), ("rotary switch ->", "cw"), ("rotary switch <->", "both")] {
        keys.push(format!("{style}/.style={{voltik/rotary={{{arrow}}}{{#1}}}}, {style}/.default={{}}"));
    }
    keys.push("legacy transistors text/.style={/tikz/circuitikz/tr/legacy text=true}".into());
    keys.push("no ferroel gate/.style={/tikz/circuitikz/tr/ferroel gate=false}".into());
    // `tripoles/mos style/arrows`: every MOSFET with its arrow (`arrowmos`)
    keys.push("/tikz/circuitikz/tripoles/mos style/arrows/.style={/tikz/circuitikz/tr/arrowmos=true}".into());
    keys.push("/tikz/circuitikz/tripoles/mos style/no arrows/.style={/tikz/circuitikz/tr/arrowmos=false}".into());
    // `n=R1`: a component's name, as `name=R1`
    keys.push("n/.style={name={#1}}".into());
    // `t=…`: the text inside an instrument
    keys.push("t/.code={\\def\\voltik@t{#1}}".into());
    for (a, start) in POLES {
        for (b, end) in POLES {
            if a.is_empty() && b.is_empty() {
                continue;
            }
            keys.push(format!("{a}-{b}/.style={{voltik/pole={{{start}}}{{{end}}}}}"));
        }
    }
    keys.push("bipole nodes/.style 2 args={voltik/pole={#1}{#2}}".into());
    // voltages, currents and flows, with the side (`^`, `_`) and the direction (`>`, `<`)
    for kind in ["v", "i", "f"] {
        for variant in ["", "^", "_", ">", "<", "^>", "^<", "_>", "_<", ">^", ">_", "<^", "<_"] {
            keys.push(format!("{kind}{variant}/.style={{voltik/vif={{{kind}}}{{{variant}}}{{#1}}}}"));
        }
    }
    // the settings every class has, and the settings a `to[…]` can give too
    s.push_str("\\pgfkeys{/tikz/circuitikz/.cd");
    let mut settings = vec![
        "bipoles/length".to_string(),
        "bipoles/thickness".into(),
        "resistors/zigs".into(),
        "inductors/coils".into(),
        "voltage".into(),
        "voltage shift".into(),
        "voltage/bump b".into(),
        "voltage/distance from node".into(),
        "current/distance".into(),
        "flow/distance".into(),
        "l2 halign".into(),
        "l2 valign".into(),
        "a2 halign".into(),
        "a2 valign".into(),
        "bipole/is voltage".into(),
        "bipole/is current".into(),
        "bipole/override source vif".into(),
        "thyristor style".into(),
        "led arrows".into(),
        "pd arrows".into(),
        "diode whiskers".into(),
        "tjumper connections".into(),
        "component text".into(),
        "bipoles/jumper/shape".into(),
        "bipoles/cuteswitch/shape".into(),
        "left text distance".into(),
    ];
    // every class of the manual has them, even before voltik draws its components, so that
    // `\ctikzset{diodes/scale=…}` is a setting and not TikZ's `scale`
    for class in [
        "resistors",
        "capacitors",
        "inductors",
        "grounds",
        "power supplies",
        "connectors",
        "diodes",
        "batteries",
        "sources",
        "csources",
        "instruments",
        "mechanicals",
        "misc",
        "blocks",
        "tubes",
        "amplifiers",
        "switches",
        "logic ports",
        "transistors",
        "flipflops",
        "muxdemuxes",
        "chips",
        "displays",
        "default",
    ] {
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
    // `\scalebox{…}[…]{text}`: the text, unscaled (a mirrored text in a mirrored node reads the same)
    s.push_str("\\def\\scalebox#1{\\@ifnextchar[{\\voltik@sbo}{\\voltik@sb}}\\def\\voltik@sbo[#1]#2{#2}\\def\\voltik@sb#1{#1}\n");
    // the manual's text helpers: a text flipped back in a flipped node (`\\ctikzflipx`), without its
    // depth (`\\killdepth`), turned (`\\rotatebox`): the text itself
    s.push_str("\\def\\raisebox#1{\\@ifnextchar[{\\voltik@rbo}{\\voltik@sb}}\\def\\voltik@rbo[#1]{\\@ifnextchar[{\\voltik@sbo}{\\voltik@sb}}\n");
    s.push_str("\\def\\ctikzflipx#1{#1}\\def\\ctikzflipy#1{#1}\\def\\ctikzflipxy#1{#1}\\def\\killdepth#1{#1}\\def\\rotatebox#1#2{#2}\n");
    // siunitx's unit macros, for a unit written outside `\SI` (which is rewritten before, in `convert`)
    for (name, symbol) in label::UNITS {
        s.push_str(&format!("\\def\\{name}{{{symbol}}}"));
    }
    s.push('\n');
    // and `transform shape` scales the components on paths too, as it does nodes
    s.push_str("\\def\\voltik@ts{0}\\expandafter\\let\\csname pgfk@/tikz/voltik@ts/.@cmd\\expandafter\\endcsname\\csname pgfk@/tikz/transform shape/.@cmd\\endcsname\n");
    s.push_str("\\tikzset{transform shape/.code={\\def\\voltik@ts{1}\\pgfkeys{/tikz/voltik@ts}}}\n");
    s.push_str("\\def\\voltik@cs{}\\def\\voltik@rs{}\\def\\voltik@bp{}\\def\\voltik@db{}\\def\\voltik@tr{}\\def\\voltik@bn{}\\def\\voltik@coil{-0}\n");
    s.push_str("\\pgfkeys{/tikz/voltik/shapes}\n");
    // the node components' sizes (`tripoles/op amp/width`), at their defaults
    // (the defaults of the measured sizes, not of a tube's wide measures)
    let mut sizes: Vec<&nodes::Param> = nodes::params().filter(|p| !p.name.contains('~')).collect();
    sizes.dedup_by_key(|p| p.setting);
    for p in sizes {
        s.push_str(&format!("\\pgfkeys{{/tikz/circuitikz/{}/.initial={}}}\n", p.setting, p.default));
    }
    s.push_str("\\tikzset{\n");
    for k in keys {
        s.push_str(&k);
        s.push_str(",\n");
    }
    s.push_str("}\n");
    // the circuit's own defaults, reset at each `to`
    s.push_str(
        "\\def\\voltik@kind{short}\\def\\voltik@fit{0}\\def\\voltik@up{}\\def\\voltik@ps{}\\def\\voltik@pe{}\\def\\voltik@mirror{0}\\def\\voltik@invert{0}\\def\\voltik@l{}\\def\\voltik@t{}\\def\\voltik@a{}\\def\\voltik@fill{}\\def\\voltik@name{}\\def\\voltik@v{}\\def\\voltik@i{}\\def\\voltik@f{}\\def\\voltik@llines{1}\\def\\voltik@alines{1}\\def\\voltik@von{0}\\def\\voltik@ion{0}\\def\\voltik@fon{0}\n",
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

/// `circuitikz` environments as the `tikzpicture`s they are, after the options the document gave
/// the package (`\usepackage[RPvoltages, american]{circuitikz}`) as styles.
fn environments(code: &str) -> String {
    let code = code.replace("\\begin{circuitikz}", "\\begin{tikzpicture}").replace("\\end{circuitikz}", "\\end{tikzpicture}");
    // siunitx anywhere (`\\SI{12}{\\volt}` in a node's text), in math whether the text is or not
    let code = label::siunitx(&code);
    let styles: Vec<&str> = package_options(&code).iter().filter_map(|o| option_style(o)).collect();
    match (styles.is_empty(), code.find("\\begin{document}")) {
        (false, Some(at)) => {
            let at = at + "\\begin{document}".len();
            format!("{}\\tikzset{{{}}}{}", &code[..at], styles.join(","), &code[at..])
        }
        _ => code,
    }
}

/// The options of `\usepackage[…]{circuitikz}`, if the code loads it.
fn package_options(code: &str) -> Vec<String> {
    let Some(end) = code.find("]{circuitikz}") else { return Vec::new() };
    let Some(start) = code[..end].rfind("\\usepackage[") else { return Vec::new() };
    code[start + "\\usepackage[".len()..end].split(',').map(|o| o.trim().to_string()).collect()
}

/// The style a package option stands for.
fn option_style(option: &str) -> Option<&'static str> {
    Some(match option {
        "RPvoltages" => "voltage dir=RP",
        "EFvoltages" => "voltage dir=EF",
        "oldvoltagedirection" => "voltage dir=old",
        "nooldvoltagedirection" => "voltage dir=noold",
        "american" => "american",
        "european" => "european",
        "americanvoltages" | "americanvoltage" => "american voltages",
        "europeanvoltages" | "europeanvoltage" => "european voltages",
        "americancurrents" | "americancurrent" => "american currents",
        "europeancurrents" | "europeancurrent" => "european currents",
        "straightvoltages" => "straight voltages",
        "americanresistors" | "americanresistor" => "american resistors",
        "europeanresistors" | "europeanresistor" => "european resistors",
        "americaninductors" | "americaninductor" => "american inductors",
        "europeaninductors" | "europeaninductor" => "european inductors",
        "cuteinductors" | "cuteinductor" => "cute inductors",
        "emptydiode" | "emptydiodes" => "empty diodes",
        "fulldiode" | "fulldiodes" => "full diodes",
        "strokediode" | "strokediodes" => "stroke diodes",
        "legacytransistorstext" => "legacy transistors text",
        _ => return None,
    })
}
