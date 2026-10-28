//! The path-style components: their size and their drawing, in a frame of their own.
//!
//! A component is drawn horizontally, centered at the origin, in units of `bipoles/length` (1.4 cm by
//! default, the length of a resistor with its leads); the path turns it. Its *width* is where the
//! leads stop, its *height* is the band that labels keep out of. Thick strokes may go half their
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

/// How a component is drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Look {
    /// Nothing: the leads join (`short`).
    Wire,
    /// Nothing, and no leads (`open`).
    Gap,
    /// A zig-zag of `resistors/zigs` teeth (the American resistor).
    ZigZag,
    /// A rectangle (`generic`, the European resistor).
    Box,
    /// Two plates (`capacitor`).
    Plates,
    /// Arches with small loops between them (the "cute" inductor).
    Coil,
}

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

/// A component: its name (as in `to[name]`), look, class, and width and height in `bipoles/length`.
#[derive(Clone, Copy, Debug)]
pub struct Component {
    pub name: &'static str,
    pub look: Look,
    pub class: Class,
    pub width: f64,
    pub height: f64,
}

/// Every component voltik draws, by its main name.
pub const COMPONENTS: &[Component] = &[
    Component { name: "short", look: Look::Wire, class: Class::Default, width: 0.0, height: 0.1 },
    Component { name: "open", look: Look::Gap, class: Class::Default, width: 0.3, height: 0.3 },
    Component { name: "generic", look: Look::Box, class: Class::Resistors, width: 0.8, height: 0.3 },
    Component { name: "resistor", look: Look::ZigZag, class: Class::Resistors, width: 0.8, height: 0.3 },
    Component { name: "capacitor", look: Look::Plates, class: Class::Capacitors, width: 0.2, height: 0.6 },
    Component { name: "cute inductor", look: Look::Coil, class: Class::Inductors, width: 0.6, height: 0.3 },
];

/// The keys that name a component, and what they name: `american` and `european` follow the
/// `resistor` choice (`\ctikzset{resistor=european}`, the `european` style).
pub const ALIASES: &[(&str, &str)] = &[
    ("short", "short"),
    ("open", "open"),
    ("generic", "generic"),
    ("R", "@resistor"),
    ("resistor", "@resistor"),
    ("american resistor", "resistor"),
    ("european resistor", "generic"),
    ("C", "capacitor"),
    ("capacitor", "capacitor"),
    ("L", "@inductor"),
    ("inductor", "@inductor"),
    ("cute inductor", "cute inductor"),
];

pub fn find(name: &str) -> Option<&'static Component> {
    COMPONENTS.iter().find(|c| c.name == name)
}

/// The outline of a component `width` × `height` (in cm, already scaled), whose thick strokes are
/// `thick` cm wide; `count` teeth for a zig-zag, arches for a coil.
pub fn outline(look: Look, width: f64, height: f64, thick: f64, count: usize) -> Vec<Op> {
    let (w, h) = (width / 2.0, height / 2.0);
    match look {
        Look::Wire | Look::Gap => Vec::new(),
        Look::Box => vec![Op::Move(-w, h), Op::Line(w, h), Op::Line(w, -h), Op::Line(-w, -h), Op::Close],
        Look::Plates => vec![Op::Move(-w, h), Op::Line(-w, -h), Op::Move(w, h), Op::Line(w, -h)],
        Look::ZigZag => {
            // 2·zigs half teeth, the first one up, from half a stroke before the lead's end
            let n = 2 * count.max(1);
            let step = width / n as f64;
            let mut ops = vec![Op::Move(-w - thick / 2.0, 0.0)];
            for i in 0..n {
                let y = if i % 2 == 0 { h } else { -h };
                ops.push(Op::Line(-w + (i as f64 + 0.5) * step, y));
            }
            ops.push(Op::Line(w + thick / 2.0, 0.0));
            ops
        }
        Look::Coil => coil(w + thick / 2.0, h, count.max(1)),
    }
}

/// `arches` arches joined by small loops below the line, from `-half` to `half`: each arch is half an
/// ellipse `h` high, each loop half an ellipse half as high and as wide as half the arch's height.
fn coil(half: f64, h: f64, arches: usize) -> Vec<Op> {
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

/// Half an ellipse from `(x, 0)` to `(x + 2rx, 0)` through `(x + rx, ry)`, as two quarter Béziers.
fn half_ellipse(ops: &mut Vec<Op>, x: f64, rx: f64, ry: f64) {
    const K: f64 = 0.552_284_75;
    let c = x + rx;
    ops.push(Op::Curve([x, K * ry], [c - K * rx, ry], [c, ry]));
    ops.push(Op::Curve([c + K * rx, ry], [c + rx, K * ry], [c + rx, 0.0]));
}
