//! The path-style components: their size and their drawing, in a frame of their own.
//!
//! A component is drawn horizontally, centered at the origin, in units of `bipoles/length` (1.4 cm by
//! default, the length of a resistor with its leads); the path turns it. Its *width* is where the
//! leads stop, its *height* is the band that labels keep out of. Its drawing is a list of [`Part`]s
//! at the component's default size, measured on what CircuiTikZ draws; a component made wider or
//! taller (`resistors/width`, `capacitors/height`) stretches them. Thick strokes may go half their
//! width beyond the leads' ends, so that the joint is covered.

/// A segment of a component's outline, in the component's frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    Move(f64, f64),
    Line(f64, f64),
    /// A cubic Bézier: two control points, then the end.
    Curve([f64; 2], [f64; 2], [f64; 2]),
    Close,
}

use Op::{Close as Z, Curve as C, Line as L, Move as M};

/// A piece of a component's drawing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Part {
    /// Filled with the `fill` of the `to`, not stroked (the space between a capacitor's plates).
    Fill(&'static [Op]),
    /// Stroked thick and filled with the `fill` of the `to` (a generic bipole's box).
    Body(&'static [Op]),
    /// Stroked thick, never filled.
    Line(&'static [Op]),
    /// Stroked thick times the class's `modifier thickness`: the marks that tell a component's kind
    /// (a sensor's bar).
    Modifier(&'static [Op]),
    /// Stroked with the path's own line width.
    Thin(&'static [Op]),
    /// Filled with the color of the stroke.
    Solid(&'static [Op]),
    /// An arrow from the first point to the second, with its head there: a modifier stroke, or the
    /// path's own line width when `thin`.
    Arrow { from: [f64; 2], to: [f64; 2], thin: bool },
    /// A potentiometer's wiper: an arrow down to the top of the component, at `wiper pos` along it.
    Wiper { thin: bool },
    /// A zig-zag of `resistors/zigs` teeth, `half` wide and `high` high on each side of the line.
    ZigZag { half: f64, high: f64 },
    /// Arches with small loops between them (the cute inductor), `inductors/coils` of them.
    Coil,
    /// Plain arches side by side (the American inductor), `inductors/coils` of them.
    Arches,
    /// Text in a font of `size` pt, its baseline starting at the point, `shift` away from it: the
    /// point stretches with the component, the shift does not.
    Text { at: [f64; 2], shift: [f64; 2], size: f64, text: &'static str },
}

use Part::{Arrow, Body, Fill, Line, Modifier, Solid, Text, Thin};

/// The classes of the manual: a class gathers components that share `scale`, `width` and `thickness`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    Default,
    Resistors,
    Capacitors,
    Inductors,
}

impl Class {
    pub fn name(self) -> &'static str {
        match self {
            Class::Default => "default",
            Class::Resistors => "resistors",
            Class::Capacitors => "capacitors",
            Class::Inductors => "inductors",
        }
    }
}

/// How a component meets its leads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leads {
    /// The leads stop at the component's width.
    Ends,
    /// Nothing between the ends: the leads join (`short`).
    Wire,
    /// No leads at all (`open`).
    Gap,
}

/// A component: its name (as in `to[name]`), class, width and height in `bipoles/length`, and its
/// drawing. `ratio` is its width over its class's `width` setting (a piezoelectric element is twice
/// as wide as the other capacitors). `top` and `bottom` are how far it reaches above and below the
/// line, its `north` and `south` (the band that labels keep out of, which a variable resistor's
/// arrow widens); `anchors` are its named points (`wiper`, `tip`, `label`…), all at the default size.
#[derive(Clone, Copy, Debug)]
pub struct Component {
    pub name: &'static str,
    pub class: Class,
    pub width: f64,
    pub height: f64,
    pub ratio: f64,
    pub top: f64,
    pub bottom: f64,
    pub leads: Leads,
    pub parts: &'static [Part],
    pub anchors: &'static [(&'static str, [f64; 2])],
}

const fn bipole(name: &'static str, class: Class, width: f64, height: f64, parts: &'static [Part]) -> Component {
    Component { name, class, width, height, ratio: 1.0, top: height / 2.0, bottom: height / 2.0, leads: Leads::Ends, parts, anchors: &[] }
}

impl Component {
    /// How far it reaches above and below the line, when that is not its height.
    const fn band(self, top: f64, bottom: f64) -> Component {
        Component { top, bottom, ..self }
    }

    const fn anchors(self, anchors: &'static [(&'static str, [f64; 2])]) -> Component {
        Component { anchors, ..self }
    }

    /// The width over a `width` setting.
    const fn ratio(self, ratio: f64) -> Component {
        Component { ratio, ..self }
    }
}

const fn resistor(name: &'static str, parts: &'static [Part]) -> Component {
    bipole(name, Class::Resistors, 0.8, 0.3, parts)
}

const fn capacitor(name: &'static str, parts: &'static [Part]) -> Component {
    bipole(name, Class::Capacitors, 0.2, 0.6, parts)
}

const fn inductor(name: &'static str, width: f64, parts: &'static [Part]) -> Component {
    bipole(name, Class::Inductors, width, 0.3, parts)
}

/// A box `w` wide and `h` high, centered.
macro_rules! rect {
    ($w:expr, $h:expr) => {
        &[M(-$w, $h), L($w, $h), L($w, -$h), L(-$w, -$h), Z]
    };
}

const BOX: &[Op] = rect!(0.4, 0.15);
/// The box of the tunable generic bipole, a little lower than the others.
const TBOX: &[Op] = rect!(0.4, 0.14);
const PLATES: &[Op] = &[M(-0.1, 0.3), L(-0.1, -0.3), M(0.1, 0.3), L(0.1, -0.3)];
const BETWEEN_PLATES: &[Op] = rect!(0.1, 0.3);
/// The dark end of the asymmetric bipoles.
const DARK_END: &[Op] = &[M(0.28, 0.15), L(0.4, 0.15), L(0.4, -0.15), L(0.28, -0.15), Z];
const TUNABLE: Part = Arrow { from: [-0.2, -0.35], to: [0.2, 0.35], thin: false };
/// The bar of a thermistor (and of the European resistive sensor).
const THERMAL_BAR: &[Op] = &[M(0.4, 0.214), L(-0.16, -0.2568), L(-0.4, -0.2568)];
/// The bar of a varistor and of the thermistors with a coefficient.
const VARISTOR_BAR: &[Op] = &[M(0.4, 0.214), L(-0.16, -0.214), L(-0.4, -0.214)];
/// A circle 0.4 across the radius.
const CIRCLE: &[Op] = &[
    M(0.0, 0.4),
    C([-0.2209, 0.4], [-0.4, 0.2209], [-0.4, 0.0]),
    C([-0.4, -0.2209], [-0.2209, -0.4], [0.0, -0.4]),
    C([0.2209, -0.4], [0.4, -0.2209], [0.4, 0.0]),
    C([0.4, 0.2209], [0.2209, 0.4], [0.0, 0.4]),
    Z,
];
/// The light falling on a light-dependent resistor.
const LIGHT: [Part; 2] = [Arrow { from: [0.56, 0.56], to: [0.32, 0.32], thin: false }, Arrow { from: [0.64, 0.48], to: [0.4, 0.24], thin: false }];
/// The wire inside a light-dependent resistor, from its circle to the resistor inside.
const INNER_LEADS: &[Op] = &[M(-0.408, 0.0), L(-0.328, 0.0), M(0.408, 0.0), L(0.328, 0.0)];
const THETA: Part = Text { at: [-0.3937, -0.3684], shift: [0.0, 0.0], size: 5.0, text: "$\\vartheta$" };
const EURO_INDUCTOR: [Part; 2] = [Solid(BOX), Line(BOX)];
const EURO_VARIABLE_INDUCTOR: [Part; 2] = [Solid(TBOX), Line(TBOX)];
/// The plates of the electrolytic capacitor: the positive one open, the negative one solid.
const ELKO_PLUS: &[Op] = &[M(-0.1, 0.25), L(-0.04, 0.25), L(-0.04, -0.25), L(-0.1, -0.25), Z];
const ELKO_MINUS: &[Op] = &[M(0.1, 0.25), L(0.04, 0.25), L(0.04, -0.25), L(0.1, -0.25), Z];
const ZIGZAG: Part = Part::ZigZag { half: 0.4, high: 0.15 };

/// Every component voltik draws, by its main name.
pub const COMPONENTS: &[Component] = &[
    Component { leads: Leads::Wire, ..bipole("short", Class::Default, 0.0, 0.1, &[]) },
    Component { leads: Leads::Gap, ..bipole("open", Class::Default, 0.3, 0.3, &[]) },
    // resistive bipoles
    resistor("generic", &[Body(BOX)]),
    resistor("xgeneric", &[Body(BOX), Line(&[M(-0.4, 0.15), L(0.4, -0.15), M(0.4, 0.15), L(-0.4, -0.15)])]),
    resistor("sgeneric", &[Body(BOX), Line(&[M(0.4, 0.15), L(-0.4, -0.15)])]),
    resistor(
        "singeneric",
        &[
            Body(BOX),
            Line(&[
                M(-0.32, 0.0),
                C([-0.2678, 0.0384], [-0.2179, 0.075], [-0.16, 0.075]),
                C([-0.102, 0.075], [-0.0521, 0.0384], [0.0, 0.0]),
                C([0.0522, -0.0384], [0.1021, -0.075], [0.16, -0.075]),
                C([0.218, -0.075], [0.2679, -0.0384], [0.32, 0.0]),
            ]),
        ],
    ),
    bipole("tgeneric", Class::Resistors, 0.8, 0.28, &[Body(TBOX), TUNABLE]).band(0.35, 0.35).anchors(&[("wiper", [-0.2, -0.35]), ("tip", [0.2, 0.35])]),
    resistor("ageneric", &[Body(BOX), Solid(DARK_END), Thin(DARK_END)]),
    resistor(
        "memristor",
        &[
            Body(BOX),
            Thin(&[
                M(-0.4, 0.0),
                L(-0.288, 0.0),
                L(-0.288, 0.075),
                L(-0.14, 0.075),
                L(-0.14, -0.075),
                L(0.02, -0.075),
                L(0.02, 0.075),
                L(0.168, 0.075),
                L(0.168, 0.0),
                L(0.32, 0.0),
            ]),
            Solid(DARK_END),
            Thin(DARK_END),
        ],
    ),
    resistor("american resistor", &[ZIGZAG]),
    resistor("variable american resistor", &[ZIGZAG, Arrow { from: [-0.16, -0.3], to: [0.16, 0.3], thin: false }])
        .band(0.3, 0.3)
        .anchors(&[("wiper", [-0.16, -0.3]), ("tip", [0.16, 0.3])]),
    resistor("american potentiometer", &[ZIGZAG, Part::Wiper { thin: false }]).band(0.4, 0.15),
    resistor("american resistive sensor", &[ZIGZAG, Modifier(&[M(0.16, 0.3), L(-0.16, -0.3), L(-0.36, -0.3)])]).band(0.3, 0.3).anchors(&[
        ("wiper", [-0.36, -0.3]),
        ("tip", [0.16, 0.3]),
        ("label", [-0.16, -0.3]),
    ]),
    bipole("american light dependent resistor", Class::Resistors, 0.8, 0.8, &[Body(CIRCLE), LIGHT[0], LIGHT[1], Line(INNER_LEADS), Part::ZigZag { half: 0.32, high: 0.128 }]),
    bipole("variable european resistor", Class::Resistors, 0.8, 0.28, &[Body(TBOX), TUNABLE]).band(0.35, 0.35).anchors(&[("wiper", [-0.2, -0.35]), ("tip", [0.2, 0.35])]),
    resistor("european potentiometer", &[Body(BOX), Part::Wiper { thin: true }]).band(0.4, 0.15),
    bipole("european light dependent resistor", Class::Resistors, 0.8, 0.8, &[Body(CIRCLE), LIGHT[0], LIGHT[1], Line(INNER_LEADS), Line(rect!(0.32, 0.12))]),
    resistor("varistor", &[Body(BOX), Modifier(VARISTOR_BAR), Text { at: [-0.3058, -0.344], shift: [0.0, 0.0], size: 5.0, text: "U" }]).band(0.214, 0.214),
    resistor("mov", &[Body(BOX), Modifier(&[M(0.4, 0.2353), L(0.28, 0.2353), L(-0.28, -0.2354), L(-0.4, -0.2354)])]).band(0.214, 0.214),
    resistor("photoresistor", &[Body(BOX), Arrow { from: [0.28, 0.3], to: [0.12, 0.18], thin: true }, Arrow { from: [0.18, 0.3], to: [0.02, 0.18], thin: true }]).band(0.3, 0.15),
    resistor("thermistor", &[Body(BOX), Modifier(THERMAL_BAR)]).band(0.214, 0.214).anchors(&[("wiper", [-0.4, -0.2568]), ("tip", [0.4, 0.214]), ("label", [-0.16, -0.2568])]),
    resistor(
        "thermistor ptc",
        &[
            Body(BOX),
            Modifier(VARISTOR_BAR),
            Arrow { from: [-0.248, -0.375], to: [-0.248, -0.2625], thin: true },
            Arrow { from: [-0.18, -0.375], to: [-0.18, -0.2625], thin: true },
            THETA,
        ],
    )
    .band(0.214, 0.375),
    resistor(
        "thermistor ntc",
        &[
            Body(BOX),
            Modifier(VARISTOR_BAR),
            Arrow { from: [-0.248, -0.375], to: [-0.248, -0.2625], thin: true },
            Arrow { from: [-0.18, -0.2625], to: [-0.18, -0.375], thin: true },
            THETA,
        ],
    )
    .band(0.214, 0.375),
    // capacitors
    capacitor("capacitor", &[Fill(BETWEEN_PLATES), Line(PLATES)]),
    capacitor(
        "curved capacitor",
        &[
            Fill(&[M(0.1, 0.3), C([0.0436, 0.2022], [-0.01, 0.1086], [-0.01, 0.0]), C([-0.01, -0.1086], [0.0436, -0.2022], [0.1, -0.3]), L(-0.1, -0.3), L(-0.1, 0.3), Z]),
            Line(&[M(0.1, 0.3), C([0.0436, 0.2022], [-0.01, 0.1086], [-0.01, 0.0]), C([-0.01, -0.1086], [0.0436, -0.2022], [0.1, -0.3])]),
            Line(&[M(-0.1, 0.3), L(-0.1, -0.3)]),
            Thin(&[M(-0.01, 0.0), L(0.1, 0.0)]),
        ],
    ),
    bipole(
        "ecapacitor",
        Class::Capacitors,
        0.2,
        0.5,
        &[Body(ELKO_PLUS), Solid(ELKO_MINUS), Line(ELKO_MINUS), Text { at: [-0.12, 0.15], shift: [-0.1609, -0.0376], size: 6.0, text: "+" }],
    ),
    capacitor("variable capacitor", &[Fill(BETWEEN_PLATES), Line(PLATES), Arrow { from: [-0.3, -0.3], to: [0.3, 0.3], thin: false }])
        .anchors(&[("wiper", [-0.3, -0.3]), ("tip", [0.3, 0.3])]),
    capacitor("capacitive sensor", &[Fill(BETWEEN_PLATES), Line(PLATES), Modifier(&[M(0.26, 0.3), L(-0.26, -0.36), L(-0.44, -0.36)])]).anchors(&[
        ("wiper", [-0.44, -0.36]),
        ("tip", [0.26, 0.3]),
        ("label", [-0.26, -0.36]),
    ]),
    bipole("piezoelectric", Class::Capacitors, 0.4, 0.7, &[Line(&[M(-0.2, 0.35), L(-0.2, -0.35), M(0.2, 0.35), L(0.2, -0.35)]), Body(rect!(0.12, 0.33))]).ratio(2.0),
    bipole(
        "cpe",
        Class::Capacitors,
        0.4,
        0.6,
        &[
            Fill(&[M(-0.2, 0.3), L(0.0, 0.0), L(-0.2, -0.3), L(0.0, -0.3), L(0.2, 0.0), L(0.0, 0.3), Z]),
            Thin(&[M(-0.2, 0.0), L(0.0, 0.0)]),
            Line(&[M(-0.2, 0.3), L(0.0, 0.0), L(-0.2, -0.3), M(0.0, 0.3), L(0.2, 0.0), L(0.0, -0.3)]),
        ],
    )
    .ratio(2.0),
    capacitor(
        "feC",
        &[
            Fill(BETWEEN_PLATES),
            Line(PLATES),
            Modifier(&[
                M(-0.18, 0.3),
                L(-0.18, 0.17),
                C([-0.18, 0.159], [-0.1593, 0.1328], [-0.1339, 0.1116]),
                L(0.1646, -0.1372),
                C([0.1731, -0.1443], [0.18, -0.1769], [0.18, -0.21]),
                L(0.18, -0.3),
            ]),
        ],
    )
    .anchors(&[("kink left", [-0.18, 0.15]), ("kink right", [0.18, -0.15]), ("curve left", [-0.18, 0.3]), ("curve right", [0.18, -0.3])]),
    // inductors
    inductor("cute inductor", 0.6, &[Part::Coil]).band(0.15, 0.075),
    inductor("variable cute inductor", 0.6, &[Arrow { from: [-0.12, -0.15], to: [0.12, 0.3], thin: false }, Part::Coil])
        .band(0.3, 0.15)
        .anchors(&[("wiper", [-0.12, -0.15]), ("tip", [0.12, 0.3])]),
    inductor("cute inductive sensor", 0.6, &[Part::Coil, Modifier(&[M(0.24, 0.3), L(-0.24, -0.195), L(-0.48, -0.195)])]).band(0.15, 0.075).anchors(&[
        ("wiper", [-0.48, -0.195]),
        ("tip", [0.24, 0.3]),
        ("label", [-0.24, -0.195]),
    ]),
    inductor("cute choke", 0.6, &[Part::Coil, Line(&[M(-0.3, 0.191), L(0.3, 0.191)])]).band(0.15, 0.075),
    inductor("american inductor", 0.8, &[Part::Arches]).band(0.15, 0.05),
    inductor("variable american inductor", 0.8, &[Part::Arches, Arrow { from: [-0.16, -0.1], to: [0.16, 0.3], thin: false }])
        .band(0.3, 0.1)
        .anchors(&[("wiper", [-0.16, -0.1]), ("tip", [0.16, 0.3])]),
    inductor("american inductive sensor", 0.8, &[Part::Arches, Modifier(&[M(0.32, 0.3), L(-0.32, -0.13), L(-0.64, -0.13)])]).band(0.15, 0.05).anchors(&[
        ("wiper", [-0.64, -0.13]),
        ("tip", [0.32, 0.3]),
        ("label", [-0.32, -0.13]),
    ]),
    inductor("european inductor", 0.8, &EURO_INDUCTOR),
    bipole("variable european inductor", Class::Inductors, 0.8, 0.28, &[EURO_VARIABLE_INDUCTOR[0], EURO_VARIABLE_INDUCTOR[1], TUNABLE])
        .band(0.35, 0.35)
        .anchors(&[("wiper", [-0.2, -0.35]), ("tip", [0.2, 0.35])]),
    inductor("european inductive sensor", 0.8, &[EURO_INDUCTOR[0], EURO_INDUCTOR[1], Modifier(&[M(0.4, 0.3), L(-0.16, -0.3), L(-0.4, -0.3)])]).anchors(&[
        ("wiper", [-0.4, -0.3]),
        ("tip", [0.4, 0.3]),
        ("label", [-0.16, -0.3]),
    ]),
];

/// What a key names: a component, or one that follows the `resistor` or `inductor` style.
#[derive(Clone, Copy, Debug)]
pub enum Target {
    Fixed(&'static str),
    /// American, European
    Resistor(&'static str, &'static str),
    /// cute, American, European
    Inductor(&'static str, &'static str, &'static str),
}

use Target::{Fixed, Inductor, Resistor};

/// The keys that name a component, and what they name: the short names (`R`, `vR`, `L`) follow the
/// `resistor` and `inductor` styles (`\ctikzset{resistor=european}`, the `european` style).
pub const ALIASES: &[(&str, Target)] = &[
    ("short", Fixed("short")),
    ("open", Fixed("open")),
    ("generic", Fixed("generic")),
    ("xgeneric", Fixed("xgeneric")),
    ("sgeneric", Fixed("sgeneric")),
    ("singeneric", Fixed("singeneric")),
    ("tgeneric", Fixed("tgeneric")),
    ("ageneric", Fixed("ageneric")),
    ("memristor", Fixed("memristor")),
    ("Mr", Fixed("memristor")),
    ("R", Resistor("american resistor", "generic")),
    ("resistor", Resistor("american resistor", "generic")),
    ("american resistor", Fixed("american resistor")),
    ("european resistor", Fixed("generic")),
    ("vR", Resistor("variable american resistor", "variable european resistor")),
    ("variable american resistor", Fixed("variable american resistor")),
    ("variable european resistor", Fixed("variable european resistor")),
    ("pR", Resistor("american potentiometer", "european potentiometer")),
    ("potentiometer", Resistor("american potentiometer", "european potentiometer")),
    ("american potentiometer", Fixed("american potentiometer")),
    ("european potentiometer", Fixed("european potentiometer")),
    ("sR", Resistor("american resistive sensor", "thermistor")),
    ("american resistive sensor", Fixed("american resistive sensor")),
    ("european resistive sensor", Fixed("thermistor")),
    ("ldR", Resistor("american light dependent resistor", "european light dependent resistor")),
    ("american light dependent resistor", Fixed("american light dependent resistor")),
    ("european light dependent resistor", Fixed("european light dependent resistor")),
    ("varistor", Fixed("varistor")),
    ("mov", Fixed("mov")),
    ("phR", Fixed("photoresistor")),
    ("photoresistor", Fixed("photoresistor")),
    ("thR", Fixed("thermistor")),
    ("thermistor", Fixed("thermistor")),
    ("thRp", Fixed("thermistor ptc")),
    ("thermistor ptc", Fixed("thermistor ptc")),
    ("thRn", Fixed("thermistor ntc")),
    ("thermistor ntc", Fixed("thermistor ntc")),
    ("C", Fixed("capacitor")),
    ("capacitor", Fixed("capacitor")),
    ("cC", Fixed("curved capacitor")),
    ("curved capacitor", Fixed("curved capacitor")),
    ("eC", Fixed("ecapacitor")),
    ("elko", Fixed("ecapacitor")),
    ("ecapacitor", Fixed("ecapacitor")),
    ("vC", Fixed("variable capacitor")),
    ("variable capacitor", Fixed("variable capacitor")),
    ("sC", Fixed("capacitive sensor")),
    ("capacitive sensor", Fixed("capacitive sensor")),
    ("PZ", Fixed("piezoelectric")),
    ("piezoelectric", Fixed("piezoelectric")),
    ("cpe", Fixed("cpe")),
    ("feC", Fixed("feC")),
    ("ferrocap", Fixed("feC")),
    ("L", Inductor("cute inductor", "american inductor", "european inductor")),
    ("inductor", Inductor("cute inductor", "american inductor", "european inductor")),
    ("cute inductor", Fixed("cute inductor")),
    ("american inductor", Fixed("american inductor")),
    ("european inductor", Fixed("european inductor")),
    ("vL", Inductor("variable cute inductor", "variable american inductor", "variable european inductor")),
    ("variable cute inductor", Fixed("variable cute inductor")),
    ("variable american inductor", Fixed("variable american inductor")),
    ("variable european inductor", Fixed("variable european inductor")),
    ("sL", Inductor("cute inductive sensor", "american inductive sensor", "european inductive sensor")),
    ("cute inductive sensor", Fixed("cute inductive sensor")),
    ("american inductive sensor", Fixed("american inductive sensor")),
    ("european inductive sensor", Fixed("european inductive sensor")),
    ("cute choke", Fixed("cute choke")),
];

pub fn find(name: &str) -> Option<&'static Component> {
    COMPONENTS.iter().find(|c| c.name == name)
}

pub fn target(key: &str) -> Option<Target> {
    ALIASES.iter().find(|(k, _)| *k == key).map(|(_, t)| *t)
}

/// A zig-zag of `count` teeth from `-half` to `half`, `high` on each side, the first tooth up, from
/// half a stroke (`thick`) before `-half` to as much after `half`.
pub fn zigzag(half: f64, high: f64, thick: f64, count: usize) -> Vec<Op> {
    let n = 2 * count.max(1);
    let step = 2.0 * half / n as f64;
    let mut ops = vec![Op::Move(-half - thick / 2.0, 0.0)];
    for i in 0..n {
        let y = if i % 2 == 0 { high } else { -high };
        ops.push(Op::Line(-half + (i as f64 + 0.5) * step, y));
    }
    ops.push(Op::Line(half + thick / 2.0, 0.0));
    ops
}

/// `arches` arches joined by small loops below the line, from `-half` to `half`: each arch is half an
/// ellipse `h` high, each loop half an ellipse half as high and as wide as half the arch's height.
pub fn coil(half: f64, h: f64, arches: usize) -> Vec<Op> {
    let back = h / 2.0;
    let arch = (2.0 * half + (arches - 1) as f64 * back) / arches as f64;
    let mut ops = vec![Op::Move(-half, 0.0)];
    let mut x = -half;
    for i in 0..arches {
        half_ellipse(&mut ops, x, arch / 2.0, h);
        x += arch;
        if i + 1 < arches {
            half_ellipse(&mut ops, x, -back / 2.0, -h / 2.0);
            x -= back;
        }
    }
    ops
}

/// `arches` arches side by side from `-half` to `half`, each half an ellipse `h` high.
pub fn arches(half: f64, h: f64, arches: usize) -> Vec<Op> {
    let arch = 2.0 * half / arches as f64;
    let mut ops = vec![Op::Move(-half, 0.0)];
    for i in 0..arches {
        half_ellipse(&mut ops, -half + i as f64 * arch, arch / 2.0, h);
    }
    ops
}

/// Half an ellipse from `(x, 0)` to `(x + 2rx, 0)` through `(x + rx, ry)`, as two quarter Béziers.
fn half_ellipse(ops: &mut Vec<Op>, x: f64, rx: f64, ry: f64) {
    const K: f64 = 0.552_284_75;
    let c = x + rx;
    ops.push(Op::Curve([x, K * ry], [c - K * rx, ry], [c, ry]));
    ops.push(Op::Curve([c + K * rx, ry], [c + rx, K * ry], [c + rx, 0.0]));
}

/// The head of the arrows of tunable components and potentiometers (CircuiTikZ's "latexslim"), for
/// a stroke `width` cm wide: its outline from the tip, as (back, side) pairs in units of its length,
/// and its length. The line stops `ARROW_INSET` of the length before the tip, inside the head.
pub const ARROW_HEAD: [[f64; 2]; 9] = [[0.25, 0.05], [0.70, 0.15], [1.0, 0.375], [0.75, 0.1], [0.75, -0.1], [1.0, -0.375], [0.70, -0.15], [0.25, -0.05], [0.0, 0.0]];
pub const ARROW_INSET: f64 = 0.6;

/// The length of an arrow head for a stroke `width` cm wide (measured: 2.79 bp and three widths).
pub fn arrow_length(width: f64) -> f64 {
    2.79 / 72.0 * 2.54 + 3.0 * width
}
