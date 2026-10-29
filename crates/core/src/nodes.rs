//! Node-style components: `\draw (0,0) node[ground] (G) {};`.
//!
//! Each is a node shape declared to tikra (`tikra::extension::Context::declare_shape`): a node like
//! any other, with the component's anchors (`G.north`, `B.zero`), placed by them (`anchor=hot`),
//! turned with the node, and filled with its `fill` where the manual makes it fillable. The drawing
//! is measured on what CircuiTikZ draws, in `bipoles/length` (1.4 cm), with the node's point at the
//! origin; the terminals are `nodes width` (0.04) across the half.

use tikra::extension::{Context, Error, NodeShape, PathSegment, Point, ShapeBorder, ShapeInput, ShapePaint, ShapePath, Tokens};

use crate::bipole::setting;

mod transistors;
mod tubes;
use crate::components::Op;
use Op::{Close as Z, Curve as C, Line as L, Move as M};

/// A path of a node component and how it is painted.
#[derive(Clone, Copy, Debug)]
pub enum Part {
    /// Stroked as wide as the node's lines, twice, or three times.
    Thin(&'static [Op]),
    Thick(&'static [Op]),
    Thicker(&'static [Op]),
    /// Filled with the node's `fill`, if any.
    Fill(&'static [Op]),
    /// Filled with the node's line color.
    Solid(&'static [Op]),
    /// Filled with the node's `fill`, or its line color: a depletion transistor's channel.
    Channel(&'static [Op]),
    /// A pole: filled with the node's `fill`, or white.
    Pole(&'static [Op]),
    /// Stroked `k` times as wide as the node's lines (a cute switch's lever).
    Stroke(f64, &'static [Op]),
}

/// Where a node component's text goes: a point, and the side of the text's box put there.
#[derive(Clone, Copy, Debug)]
pub enum Text {
    /// Nowhere special: centered on the node's point.
    Center,
    /// Above the point, centered (its box's bottom there, past the inner sep).
    Above([f64; 2]),
    Below([f64; 2]),
    /// Above, with its right or left edge at the point.
    AboveLeft([f64; 2]),
    AboveRight([f64; 2]),
    /// Right of the point, its box's left edge there, centered on it (a transistor's).
    Right([f64; 2]),
    /// Centered on the point (a magnetron's).
    Middle([f64; 2]),
}

/// A node component: its name (`node[name]`), its drawing, its box (left, right, bottom, top) for
/// the compass anchors, its own anchors and where its text goes. Its drawing counts in the
/// picture's size, its text does not, unless `text_counts`; `extent` are points that count too.
#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub name: &'static str,
    pub parts: &'static [Part],
    pub bounds: [f64; 4],
    pub anchors: &'static [(&'static str, [f64; 2])],
    pub text: Text,
    pub text_counts: bool,
    pub extent: &'static [[f64; 2]],
}

const fn node(name: &'static str, parts: &'static [Part], bounds: [f64; 4]) -> Node {
    Node { name, parts, bounds, anchors: &[], text: Text::Center, text_counts: false, extent: &[] }
}

impl Node {
    /// The class whose `scale` it follows, if any.
    pub fn class(&self) -> Option<&'static str> {
        match self.name {
            "vcc" | "vee" => Some("power supplies"),
            "bnc" | "iecconnshape" | "iecsocketL" | "iecsocketR" | "iecplugL" | "iecplugR" => Some("connectors"),
            n if n.ends_with("ground") || n == "eground2" => Some("grounds"),
            n if n.contains("spdt") || n == "proximeter" => Some("switches"),
            n if transistors::TRANSISTORS.iter().any(|t| t.name == n) => Some("transistors"),
            n if tubes::TUBES.iter().any(|t| t.name == n) => Some("tubes"),
            _ => None,
        }
    }
}

/// A circle as four Béziers, around `(x, y)`, `r` across the radius.
macro_rules! circle {
    ($x:expr, $y:expr, $r:expr) => {
        &[
            M($x + $r, $y),
            C([$x + $r, $y + 0.5523 * $r], [$x + 0.5523 * $r, $y + $r], [$x, $y + $r]),
            C([$x - 0.5523 * $r, $y + $r], [$x - $r, $y + 0.5523 * $r], [$x - $r, $y]),
            C([$x - $r, $y - 0.5523 * $r], [$x - 0.5523 * $r, $y - $r], [$x, $y - $r]),
            C([$x + 0.5523 * $r, $y - $r], [$x + $r, $y - 0.5523 * $r], [$x + $r, $y]),
            Z,
        ]
    };
}

/// The three lines of a ground, from `y` down.
macro_rules! ground_lines {
    ($y:expr) => {
        &[M(-0.15, $y), L(0.15, $y), M(-0.1, $y - 0.05), L(0.1, $y - 0.05), M(-0.0625, $y - 0.1), L(0.0625, $y - 0.1)]
    };
}

const STEM: &[Op] = &[M(0.0, 0.0), L(0.0, -0.25)];
const IEC_PLUG: &[Op] = &[M(0.0, 0.05), L(0.2, 0.05), L(0.2, -0.05), L(0.0, -0.05), Z];

/// Every node component voltik draws (the terminals apart, sized by `nodes width`).
pub const NODES: &[Node] = &[
    node("ground", &[Part::Thin(&[M(0.0, 0.0), L(0.0, -0.3)]), Part::Thick(ground_lines!(-0.3))], [-0.15, 0.15, -0.4, 0.0]),
    node("tlground", &[Part::Thick(ground_lines!(0.0))], [-0.15, 0.15, -0.1, 0.0]),
    node("rground", &[Part::Thin(STEM), Part::Thick(&[M(-0.15, -0.25), L(0.15, -0.25)])], [-0.15, 0.15, -0.25, 0.0]),
    node(
        "sground",
        &[Part::Thin(STEM), Part::Fill(&[M(-0.15, -0.25), L(0.15, -0.25), L(0.0, -0.45), Z]), Part::Thick(&[M(-0.15, -0.25), L(0.15, -0.25), L(0.0, -0.45), Z])],
        [-0.15, 0.15, -0.45, 0.0],
    ),
    node("tground", &[Part::Thicker(&[M(-0.15, 0.0), L(0.15, 0.0)])], [-0.15, 0.15, 0.0, 0.0]),
    node(
        "nground",
        &[
            Part::Thin(&[M(0.0, 0.0), L(0.0, -0.3)]),
            Part::Thick(ground_lines!(-0.3)),
            Part::Thick(&[M(0.225, -0.4), C([0.225, -0.2757], [0.1242, -0.175], [0.0, -0.175]), C([-0.1243, -0.175], [-0.225, -0.2757], [-0.225, -0.4])]),
        ],
        [-0.225, 0.225, -0.4, 0.0],
    ),
    node(
        "pground",
        &[Part::Fill(circle!(0.0, -0.225, 0.225)), Part::Thick(circle!(0.0, -0.225, 0.225)), Part::Thin(STEM), Part::Thick(ground_lines!(-0.25))],
        [-0.225, 0.225, -0.45, 0.0],
    ),
    node(
        "cground",
        &[
            Part::Thin(&[M(0.0, 0.0), L(0.0, -0.375)]),
            Part::Thick(&[M(-0.25, -0.525), L(-0.1875, -0.375), L(0.1875, -0.375), L(0.125, -0.525), M(0.0, -0.375), L(-0.0625, -0.525)]),
        ],
        [-0.25, 0.25, -0.5, 0.0],
    ),
    node(
        "eground",
        &[
            Part::Thin(STEM),
            Part::Thicker(&[M(-0.25, -0.25), L(0.25, -0.25)]),
            Part::Thin(&[M(-0.275, -0.425), L(-0.15, -0.25), M(-0.15, -0.425), L(-0.025, -0.25), M(-0.025, -0.425), L(0.1, -0.25), M(0.1, -0.425), L(0.225, -0.25)]),
        ],
        [-0.275, 0.275, -0.425, 0.0],
    ),
    node(
        "eground2",
        &[
            Part::Thin(STEM),
            Part::Thicker(&[M(-0.25, -0.25), L(0.25, -0.25)]),
            Part::Thin(&[M(-0.275, -0.425), L(-0.1, -0.25), M(-0.1125, -0.425), L(0.0625, -0.25), M(0.05, -0.425), L(0.225, -0.25)]),
        ],
        [-0.275, 0.275, -0.425, 0.0],
    ),
    Node {
        text: Text::Above([0.0, 0.4]),
        anchors: &[("text", [0.0, 0.4])],
        text_counts: true,
        extent: &[[0.0, 0.4]],
        ..node("vcc", &[Part::Thick(&[M(-0.1, 0.1601), L(0.0, 0.3), L(0.1, 0.1601)]), Part::Thin(&[M(0.0, 0.0), L(0.0, 0.3)])], [-0.1, 0.1, 0.0, 0.3])
    },
    Node {
        text: Text::Below([0.0, -0.4]),
        anchors: &[("text", [0.0, -0.4])],
        text_counts: true,
        extent: &[[0.0, -0.4]],
        ..node("vee", &[Part::Thick(&[M(-0.1, -0.1601), L(0.0, -0.3), L(0.1, -0.1601)]), Part::Thin(&[M(0.0, 0.0), L(0.0, -0.3)])], [-0.1, 0.1, -0.3, 0.0])
    },
    // connectors
    Node {
        text: Text::Above([-0.1, 0.1]),
        anchors: &[("text", [-0.1, 0.1]), ("hot", [0.0, 0.0]), ("zero", [-0.1, 0.0]), ("shield", [-0.1, -0.1])],
        // its picture is a square around its circle, twice as wide
        extent: &[[-0.3, -0.2], [0.1, 0.2]],
        ..node(
            "bnc",
            &[Part::Fill(circle!(-0.1, 0.0, 0.1)), Part::Thick(circle!(-0.1, 0.0, 0.1)), Part::Thick(circle!(-0.1, 0.0, 0.04)), Part::Thin(&[M(-0.06, 0.0), L(0.0, 0.0)])],
            [-0.2, 0.0, -0.1, 0.1],
        )
    },
    Node {
        text: Text::Above([0.0, 0.1]),
        anchors: &[("text", [0.0, 0.1]), ("plug center", [0.05, 0.0]), ("socket center", [-0.05, 0.0])],
        ..node(
            "iecconnshape",
            &[
                Part::Thick(&[M(-0.05, 0.1), C([-0.1052, 0.1], [-0.15, 0.0552], [-0.15, 0.0]), C([-0.15, -0.0552], [-0.1052, -0.1], [-0.05, -0.1])]),
                Part::Solid(&[M(-0.05, 0.05), L(0.15, 0.05), L(0.15, -0.05), L(-0.05, -0.05), Z]),
                Part::Thick(&[M(-0.05, 0.05), L(0.15, 0.05), L(0.15, -0.05), L(-0.05, -0.05), Z]),
            ],
            [-0.15, 0.15, -0.1, 0.1],
        )
    },
    Node {
        text: Text::AboveLeft([0.0, 0.1]),
        anchors: &[("text", [0.0, 0.1]), ("socket center", [0.1, 0.0])],
        ..node("iecsocketL", &[Part::Thick(&[M(0.1, 0.1), C([0.0448, 0.1], [0.0, 0.0552], [0.0, 0.0]), C([0.0, -0.0552], [0.0448, -0.1], [0.1, -0.1])])], [0.0, 0.1, -0.1, 0.1])
    },
    Node {
        text: Text::AboveRight([0.2, 0.1]),
        anchors: &[("text", [0.2, 0.1]), ("plug center", [0.1, 0.0])],
        ..node("iecplugR", &[Part::Solid(IEC_PLUG), Part::Thick(IEC_PLUG)], [0.0, 0.2, -0.05, 0.05])
    },
    Node {
        text: Text::AboveLeft([0.0, 0.1]),
        anchors: &[("text", [0.0, 0.1]), ("plug center", [0.1, 0.0])],
        ..node("iecplugL", &[Part::Solid(IEC_PLUG), Part::Thick(IEC_PLUG)], [0.0, 0.2, -0.05, 0.05])
    },
    Node {
        text: Text::AboveRight([0.1, 0.1]),
        anchors: &[("text", [0.1, 0.1]), ("socket center", [0.0, 0.0])],
        ..node("iecsocketR", &[Part::Thick(&[M(0.0, 0.1), C([0.0552, 0.1], [0.1, 0.0552], [0.1, 0.0]), C([0.1, -0.0552], [0.0552, -0.1], [0.0, -0.1])])], [0.0, 0.1, -0.1, 0.1])
    },
    // crossings
    node(
        "jump crossing",
        &[
            Part::Thin(&[M(-0.1, 0.0), L(-0.04, 0.0), C([-0.04, 0.0221], [-0.0221, 0.04], [0.0, 0.04]), C([0.0221, 0.04], [0.04, 0.0221], [0.04, 0.0]), L(0.1, 0.0)]),
            Part::Thin(&[M(0.0, 0.1), L(0.0, 0.05), M(0.0, 0.03), L(0.0, -0.1)]),
        ],
        [-0.1, 0.1, -0.1, 0.1],
    ),
    node("plain crossing", &[Part::Thin(&[M(-0.1, 0.0), L(0.1, 0.0)]), Part::Thin(&[M(0.0, 0.1), L(0.0, 0.0101), M(0.0, -0.0101), L(0.0, -0.1)])], [-0.1, 0.1, -0.1, 0.1]),
    // arrows
    Node { anchors: &[("tip", [0.0625, 0.0])], ..node("currarrow", &[Part::Solid(CURRARROW), Part::Thin(CURRARROW)], [-0.0312, 0.0312, -0.0312, 0.0312]) },
    node("inputarrow", &[Part::Solid(&[M(-0.1062, 0.0), L(-0.1062, -0.05), L(0.0, 0.0), L(-0.1062, 0.05), Z])], [-0.1062, 0.0, -0.0312, 0.0312]),
    Node {
        anchors: &[("tip", [0.08, 0.0]), ("btip", [0.075, 0.0])],
        ..node("trarrow", &[Part::Solid(&[M(-0.0487, 0.0), L(-0.0487, 0.0576), L(0.08, 0.0), L(-0.0487, -0.0577), Z])], [-0.0488, 0.0488, -0.0576, 0.0576])
    },
    Node {
        text: Text::Above([0.0, 0.0]),
        anchors: &[("tip", [0.0625, 0.0])],
        ..node("flowarrow", &[Part::Thin(&[M(-0.25, 0.0), L(0.25, 0.0)]), Part::Solid(FLOWHEAD), Part::Thin(FLOWHEAD)], [-0.25, 0.25, -0.0312, 0.0312])
    },
    // switches
    Node {
        anchors: &[("in", [-0.2525, 0.0]), ("out 1", [0.2525, 0.3]), ("out 2", [0.2525, -0.3]), ("mid", [0.0, -0.12])],
        ..node(
            "cute spdt down",
            &[
                Part::Stroke(7.9653, &[M(-0.2125, 0.0), L(0.2125, -0.24)]),
                Part::Pole(&[
                    M(0.2525, 0.3),
                    C([0.2525, 0.3221], [0.2346, 0.34], [0.2125, 0.34]),
                    C([0.1905, 0.34], [0.1726, 0.3221], [0.1726, 0.3]),
                    C([0.1726, 0.2779], [0.1905, 0.2599], [0.2125, 0.2599]),
                    C([0.2346, 0.2599], [0.2525, 0.2779], [0.2525, 0.3]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2525, 0.3),
                    C([0.2525, 0.3221], [0.2346, 0.34], [0.2125, 0.34]),
                    C([0.1905, 0.34], [0.1726, 0.3221], [0.1726, 0.3]),
                    C([0.1726, 0.2779], [0.1905, 0.2599], [0.2125, 0.2599]),
                    C([0.2346, 0.2599], [0.2525, 0.2779], [0.2525, 0.3]),
                    Z,
                ]),
                Part::Pole(&[
                    M(0.2525, -0.3),
                    C([0.2525, -0.278], [0.2346, -0.2601], [0.2125, -0.2601]),
                    C([0.1905, -0.2601], [0.1726, -0.278], [0.1726, -0.3]),
                    C([0.1726, -0.3221], [0.1905, -0.34], [0.2125, -0.34]),
                    C([0.2346, -0.34], [0.2525, -0.3221], [0.2525, -0.3]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2525, -0.3),
                    C([0.2525, -0.278], [0.2346, -0.2601], [0.2125, -0.2601]),
                    C([0.1905, -0.2601], [0.1726, -0.278], [0.1726, -0.3]),
                    C([0.1726, -0.3221], [0.1905, -0.34], [0.2125, -0.34]),
                    C([0.2346, -0.34], [0.2525, -0.3221], [0.2525, -0.3]),
                    Z,
                ]),
                Part::Pole(&[
                    M(-0.1724, 0.0),
                    C([-0.1724, 0.0221], [-0.1904, 0.0399], [-0.2125, 0.0399]),
                    C([-0.2346, 0.0399], [-0.2525, 0.0221], [-0.2525, 0.0]),
                    C([-0.2525, -0.0221], [-0.2346, -0.0399], [-0.2125, -0.0399]),
                    C([-0.1904, -0.0399], [-0.1724, -0.0221], [-0.1724, 0.0]),
                    Z,
                ]),
                Part::Thin(&[
                    M(-0.1724, 0.0),
                    C([-0.1724, 0.0221], [-0.1904, 0.0399], [-0.2125, 0.0399]),
                    C([-0.2346, 0.0399], [-0.2525, 0.0221], [-0.2525, 0.0]),
                    C([-0.2525, -0.0221], [-0.2346, -0.0399], [-0.2125, -0.0399]),
                    C([-0.1904, -0.0399], [-0.1724, -0.0221], [-0.1724, 0.0]),
                    Z,
                ]),
            ],
            [-0.2125, 0.2125, -0.3, 0.3],
        )
    },
    Node {
        anchors: &[("in", [-0.2525, 0.0]), ("out 1", [0.2525, 0.3]), ("out 2", [0.2525, -0.3]), ("mid", [0.0, -0.12])],
        ..node(
            "cute spdt down arrow",
            &[
                Part::Stroke(7.9653, &[M(-0.2125, 0.0), L(0.2125, -0.24)]),
                Part::Thick(&[M(-0.0076, -0.2442), C([0.1273, -0.131], [0.1449, 0.07], [0.0317, 0.2049]), C([0.0126, 0.2276], [-0.0094, 0.2474], [-0.0339, 0.264])]),
                Part::Solid(&[
                    M(-0.1034, 0.2995),
                    C([-0.0791, 0.2768], [-0.036, 0.2348], [-0.0182, 0.1892]),
                    C([-0.0271, 0.2369], [-0.0133, 0.2591], [0.0333, 0.2724]),
                    C([-0.0154, 0.2681], [-0.0722, 0.2879], [-0.1034, 0.2995]),
                    Z,
                ]),
                Part::Pole(&[
                    M(0.2525, 0.3),
                    C([0.2525, 0.3221], [0.2346, 0.34], [0.2125, 0.34]),
                    C([0.1905, 0.34], [0.1726, 0.3221], [0.1726, 0.3]),
                    C([0.1726, 0.2779], [0.1905, 0.2599], [0.2125, 0.2599]),
                    C([0.2346, 0.2599], [0.2525, 0.2779], [0.2525, 0.3]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2525, 0.3),
                    C([0.2525, 0.3221], [0.2346, 0.34], [0.2125, 0.34]),
                    C([0.1905, 0.34], [0.1726, 0.3221], [0.1726, 0.3]),
                    C([0.1726, 0.2779], [0.1905, 0.2599], [0.2125, 0.2599]),
                    C([0.2346, 0.2599], [0.2525, 0.2779], [0.2525, 0.3]),
                    Z,
                ]),
                Part::Pole(&[
                    M(0.2525, -0.3),
                    C([0.2525, -0.278], [0.2346, -0.2601], [0.2125, -0.2601]),
                    C([0.1905, -0.2601], [0.1726, -0.278], [0.1726, -0.3]),
                    C([0.1726, -0.3221], [0.1905, -0.34], [0.2125, -0.34]),
                    C([0.2346, -0.34], [0.2525, -0.3221], [0.2525, -0.3]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2525, -0.3),
                    C([0.2525, -0.278], [0.2346, -0.2601], [0.2125, -0.2601]),
                    C([0.1905, -0.2601], [0.1726, -0.278], [0.1726, -0.3]),
                    C([0.1726, -0.3221], [0.1905, -0.34], [0.2125, -0.34]),
                    C([0.2346, -0.34], [0.2525, -0.3221], [0.2525, -0.3]),
                    Z,
                ]),
                Part::Pole(&[
                    M(-0.1724, 0.0),
                    C([-0.1724, 0.0221], [-0.1904, 0.0399], [-0.2125, 0.0399]),
                    C([-0.2346, 0.0399], [-0.2525, 0.0221], [-0.2525, 0.0]),
                    C([-0.2525, -0.0221], [-0.2346, -0.0399], [-0.2125, -0.0399]),
                    C([-0.1904, -0.0399], [-0.1724, -0.0221], [-0.1724, 0.0]),
                    Z,
                ]),
                Part::Thin(&[
                    M(-0.1724, 0.0),
                    C([-0.1724, 0.0221], [-0.1904, 0.0399], [-0.2125, 0.0399]),
                    C([-0.2346, 0.0399], [-0.2525, 0.0221], [-0.2525, 0.0]),
                    C([-0.2525, -0.0221], [-0.2346, -0.0399], [-0.2125, -0.0399]),
                    C([-0.1904, -0.0399], [-0.1724, -0.0221], [-0.1724, 0.0]),
                    Z,
                ]),
            ],
            [-0.2125, 0.2125, -0.3, 0.3],
        )
    },
    Node {
        anchors: &[("in", [-0.2525, 0.0]), ("out 1", [0.2525, 0.3]), ("out 2", [0.2525, -0.3]), ("mid", [0.0, 0.0])],
        ..node(
            "cute spdt mid",
            &[
                Part::Stroke(7.9653, &[M(-0.2125, 0.0), L(0.2125, 0.0)]),
                Part::Pole(&[
                    M(0.2525, 0.3),
                    C([0.2525, 0.3221], [0.2346, 0.34], [0.2125, 0.34]),
                    C([0.1905, 0.34], [0.1726, 0.3221], [0.1726, 0.3]),
                    C([0.1726, 0.2779], [0.1905, 0.2599], [0.2125, 0.2599]),
                    C([0.2346, 0.2599], [0.2525, 0.2779], [0.2525, 0.3]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2525, 0.3),
                    C([0.2525, 0.3221], [0.2346, 0.34], [0.2125, 0.34]),
                    C([0.1905, 0.34], [0.1726, 0.3221], [0.1726, 0.3]),
                    C([0.1726, 0.2779], [0.1905, 0.2599], [0.2125, 0.2599]),
                    C([0.2346, 0.2599], [0.2525, 0.2779], [0.2525, 0.3]),
                    Z,
                ]),
                Part::Pole(&[
                    M(0.2525, -0.3),
                    C([0.2525, -0.278], [0.2346, -0.2601], [0.2125, -0.2601]),
                    C([0.1905, -0.2601], [0.1726, -0.278], [0.1726, -0.3]),
                    C([0.1726, -0.3221], [0.1905, -0.34], [0.2125, -0.34]),
                    C([0.2346, -0.34], [0.2525, -0.3221], [0.2525, -0.3]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2525, -0.3),
                    C([0.2525, -0.278], [0.2346, -0.2601], [0.2125, -0.2601]),
                    C([0.1905, -0.2601], [0.1726, -0.278], [0.1726, -0.3]),
                    C([0.1726, -0.3221], [0.1905, -0.34], [0.2125, -0.34]),
                    C([0.2346, -0.34], [0.2525, -0.3221], [0.2525, -0.3]),
                    Z,
                ]),
                Part::Pole(&[
                    M(-0.1724, 0.0),
                    C([-0.1724, 0.0221], [-0.1904, 0.0399], [-0.2125, 0.0399]),
                    C([-0.2346, 0.0399], [-0.2525, 0.0221], [-0.2525, 0.0]),
                    C([-0.2525, -0.0221], [-0.2346, -0.0399], [-0.2125, -0.0399]),
                    C([-0.1904, -0.0399], [-0.1724, -0.0221], [-0.1724, 0.0]),
                    Z,
                ]),
                Part::Thin(&[
                    M(-0.1724, 0.0),
                    C([-0.1724, 0.0221], [-0.1904, 0.0399], [-0.2125, 0.0399]),
                    C([-0.2346, 0.0399], [-0.2525, 0.0221], [-0.2525, 0.0]),
                    C([-0.2525, -0.0221], [-0.2346, -0.0399], [-0.2125, -0.0399]),
                    C([-0.1904, -0.0399], [-0.1724, -0.0221], [-0.1724, 0.0]),
                    Z,
                ]),
            ],
            [-0.2125, 0.2125, -0.3, 0.3],
        )
    },
    Node {
        anchors: &[("in", [-0.2525, 0.0]), ("out 1", [0.2525, 0.3]), ("out 2", [0.2525, -0.3]), ("mid", [0.0, 0.0])],
        ..node(
            "cute spdt mid arrow",
            &[
                Part::Stroke(7.9653, &[M(-0.2125, 0.0), L(0.2125, 0.0)]),
                Part::Thick(&[M(0.0094, -0.2289), C([0.112, -0.1296], [0.1382, 0.0301], [0.0636, 0.1594]), C([0.0487, 0.1851], [0.0304, 0.2085], [0.0093, 0.2289])]),
                Part::Solid(&[
                    M(-0.0531, -0.2761),
                    C([-0.0244, -0.2591], [0.0281, -0.2299], [0.0769, -0.2257]),
                    C([0.0286, -0.2206], [0.0112, -0.2012], [0.0117, -0.1526]),
                    C([0.0021, -0.2006], [-0.0331, -0.2494], [-0.0531, -0.2761]),
                    Z,
                ]),
                Part::Solid(&[
                    M(-0.0531, 0.276),
                    C([-0.0331, 0.2494], [0.0021, 0.2006], [0.0117, 0.1526]),
                    C([0.0112, 0.2011], [0.0286, 0.2206], [0.0769, 0.2257]),
                    C([0.0281, 0.2299], [-0.0244, 0.2591], [-0.0531, 0.276]),
                    Z,
                ]),
                Part::Pole(&[
                    M(0.2525, 0.3),
                    C([0.2525, 0.3221], [0.2346, 0.34], [0.2125, 0.34]),
                    C([0.1905, 0.34], [0.1726, 0.3221], [0.1726, 0.3]),
                    C([0.1726, 0.2779], [0.1905, 0.2599], [0.2125, 0.2599]),
                    C([0.2346, 0.2599], [0.2525, 0.2779], [0.2525, 0.3]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2525, 0.3),
                    C([0.2525, 0.3221], [0.2346, 0.34], [0.2125, 0.34]),
                    C([0.1905, 0.34], [0.1726, 0.3221], [0.1726, 0.3]),
                    C([0.1726, 0.2779], [0.1905, 0.2599], [0.2125, 0.2599]),
                    C([0.2346, 0.2599], [0.2525, 0.2779], [0.2525, 0.3]),
                    Z,
                ]),
                Part::Pole(&[
                    M(0.2525, -0.3),
                    C([0.2525, -0.278], [0.2346, -0.2601], [0.2125, -0.2601]),
                    C([0.1905, -0.2601], [0.1726, -0.278], [0.1726, -0.3]),
                    C([0.1726, -0.3221], [0.1905, -0.34], [0.2125, -0.34]),
                    C([0.2346, -0.34], [0.2525, -0.3221], [0.2525, -0.3]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2525, -0.3),
                    C([0.2525, -0.278], [0.2346, -0.2601], [0.2125, -0.2601]),
                    C([0.1905, -0.2601], [0.1726, -0.278], [0.1726, -0.3]),
                    C([0.1726, -0.3221], [0.1905, -0.34], [0.2125, -0.34]),
                    C([0.2346, -0.34], [0.2525, -0.3221], [0.2525, -0.3]),
                    Z,
                ]),
                Part::Pole(&[
                    M(-0.1724, 0.0),
                    C([-0.1724, 0.0221], [-0.1904, 0.0399], [-0.2125, 0.0399]),
                    C([-0.2346, 0.0399], [-0.2525, 0.0221], [-0.2525, 0.0]),
                    C([-0.2525, -0.0221], [-0.2346, -0.0399], [-0.2125, -0.0399]),
                    C([-0.1904, -0.0399], [-0.1724, -0.0221], [-0.1724, 0.0]),
                    Z,
                ]),
                Part::Thin(&[
                    M(-0.1724, 0.0),
                    C([-0.1724, 0.0221], [-0.1904, 0.0399], [-0.2125, 0.0399]),
                    C([-0.2346, 0.0399], [-0.2525, 0.0221], [-0.2525, 0.0]),
                    C([-0.2525, -0.0221], [-0.2346, -0.0399], [-0.2125, -0.0399]),
                    C([-0.1904, -0.0399], [-0.1724, -0.0221], [-0.1724, 0.0]),
                    Z,
                ]),
            ],
            [-0.2125, 0.2125, -0.3, 0.3],
        )
    },
    Node {
        anchors: &[("in", [-0.2525, 0.0]), ("out 1", [0.2525, 0.3]), ("out 2", [0.2525, -0.3]), ("mid", [0.0, 0.12])],
        ..node(
            "cute spdt up",
            &[
                Part::Stroke(7.9653, &[M(-0.2125, 0.0), L(0.2125, 0.24)]),
                Part::Pole(&[
                    M(0.2525, 0.3),
                    C([0.2525, 0.3221], [0.2346, 0.34], [0.2125, 0.34]),
                    C([0.1905, 0.34], [0.1726, 0.3221], [0.1726, 0.3]),
                    C([0.1726, 0.2779], [0.1905, 0.2599], [0.2125, 0.2599]),
                    C([0.2346, 0.2599], [0.2525, 0.2779], [0.2525, 0.3]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2525, 0.3),
                    C([0.2525, 0.3221], [0.2346, 0.34], [0.2125, 0.34]),
                    C([0.1905, 0.34], [0.1726, 0.3221], [0.1726, 0.3]),
                    C([0.1726, 0.2779], [0.1905, 0.2599], [0.2125, 0.2599]),
                    C([0.2346, 0.2599], [0.2525, 0.2779], [0.2525, 0.3]),
                    Z,
                ]),
                Part::Pole(&[
                    M(0.2525, -0.3),
                    C([0.2525, -0.278], [0.2346, -0.2601], [0.2125, -0.2601]),
                    C([0.1905, -0.2601], [0.1726, -0.278], [0.1726, -0.3]),
                    C([0.1726, -0.3221], [0.1905, -0.34], [0.2125, -0.34]),
                    C([0.2346, -0.34], [0.2525, -0.3221], [0.2525, -0.3]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2525, -0.3),
                    C([0.2525, -0.278], [0.2346, -0.2601], [0.2125, -0.2601]),
                    C([0.1905, -0.2601], [0.1726, -0.278], [0.1726, -0.3]),
                    C([0.1726, -0.3221], [0.1905, -0.34], [0.2125, -0.34]),
                    C([0.2346, -0.34], [0.2525, -0.3221], [0.2525, -0.3]),
                    Z,
                ]),
                Part::Pole(&[
                    M(-0.1724, 0.0),
                    C([-0.1724, 0.0221], [-0.1904, 0.0399], [-0.2125, 0.0399]),
                    C([-0.2346, 0.0399], [-0.2525, 0.0221], [-0.2525, 0.0]),
                    C([-0.2525, -0.0221], [-0.2346, -0.0399], [-0.2125, -0.0399]),
                    C([-0.1904, -0.0399], [-0.1724, -0.0221], [-0.1724, 0.0]),
                    Z,
                ]),
                Part::Thin(&[
                    M(-0.1724, 0.0),
                    C([-0.1724, 0.0221], [-0.1904, 0.0399], [-0.2125, 0.0399]),
                    C([-0.2346, 0.0399], [-0.2525, 0.0221], [-0.2525, 0.0]),
                    C([-0.2525, -0.0221], [-0.2346, -0.0399], [-0.2125, -0.0399]),
                    C([-0.1904, -0.0399], [-0.1724, -0.0221], [-0.1724, 0.0]),
                    Z,
                ]),
            ],
            [-0.2125, 0.2125, -0.3, 0.3],
        )
    },
    Node {
        anchors: &[("in", [-0.2525, 0.0]), ("out 1", [0.2525, 0.3]), ("out 2", [0.2525, -0.3]), ("mid", [0.0, 0.12])],
        ..node(
            "cute spdt up arrow",
            &[
                Part::Stroke(7.9653, &[M(-0.2125, 0.0), L(0.2125, 0.24)]),
                Part::Thick(&[M(-0.1034, 0.2995), C([0.062, 0.2393], [0.1473, 0.0564], [0.087, -0.1091]), C([0.0769, -0.1369], [0.0629, -0.1631], [0.0456, -0.187])]),
                Part::Solid(&[
                    M(-0.0076, -0.2442),
                    C([0.0177, -0.2226], [0.0644, -0.1846], [0.1117, -0.1721]),
                    C([0.0633, -0.1754], [0.0428, -0.1593], [0.0349, -0.1114]),
                    C([0.0336, -0.1604], [0.0075, -0.2145], [-0.0076, -0.2442]),
                    Z,
                ]),
                Part::Pole(&[
                    M(0.2525, 0.3),
                    C([0.2525, 0.3221], [0.2346, 0.34], [0.2125, 0.34]),
                    C([0.1905, 0.34], [0.1726, 0.3221], [0.1726, 0.3]),
                    C([0.1726, 0.2779], [0.1905, 0.2599], [0.2125, 0.2599]),
                    C([0.2346, 0.2599], [0.2525, 0.2779], [0.2525, 0.3]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2525, 0.3),
                    C([0.2525, 0.3221], [0.2346, 0.34], [0.2125, 0.34]),
                    C([0.1905, 0.34], [0.1726, 0.3221], [0.1726, 0.3]),
                    C([0.1726, 0.2779], [0.1905, 0.2599], [0.2125, 0.2599]),
                    C([0.2346, 0.2599], [0.2525, 0.2779], [0.2525, 0.3]),
                    Z,
                ]),
                Part::Pole(&[
                    M(0.2525, -0.3),
                    C([0.2525, -0.278], [0.2346, -0.2601], [0.2125, -0.2601]),
                    C([0.1905, -0.2601], [0.1726, -0.278], [0.1726, -0.3]),
                    C([0.1726, -0.3221], [0.1905, -0.34], [0.2125, -0.34]),
                    C([0.2346, -0.34], [0.2525, -0.3221], [0.2525, -0.3]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2525, -0.3),
                    C([0.2525, -0.278], [0.2346, -0.2601], [0.2125, -0.2601]),
                    C([0.1905, -0.2601], [0.1726, -0.278], [0.1726, -0.3]),
                    C([0.1726, -0.3221], [0.1905, -0.34], [0.2125, -0.34]),
                    C([0.2346, -0.34], [0.2525, -0.3221], [0.2525, -0.3]),
                    Z,
                ]),
                Part::Pole(&[
                    M(-0.1724, 0.0),
                    C([-0.1724, 0.0221], [-0.1904, 0.0399], [-0.2125, 0.0399]),
                    C([-0.2346, 0.0399], [-0.2525, 0.0221], [-0.2525, 0.0]),
                    C([-0.2525, -0.0221], [-0.2346, -0.0399], [-0.2125, -0.0399]),
                    C([-0.1904, -0.0399], [-0.1724, -0.0221], [-0.1724, 0.0]),
                    Z,
                ]),
                Part::Thin(&[
                    M(-0.1724, 0.0),
                    C([-0.1724, 0.0221], [-0.1904, 0.0399], [-0.2125, 0.0399]),
                    C([-0.2346, 0.0399], [-0.2525, 0.0221], [-0.2525, 0.0]),
                    C([-0.2525, -0.0221], [-0.2346, -0.0399], [-0.2125, -0.0399]),
                    C([-0.1904, -0.0399], [-0.1724, -0.0221], [-0.1724, 0.0]),
                    Z,
                ]),
            ],
            [-0.2125, 0.2125, -0.3, 0.3],
        )
    },
    Node {
        text: Text::Above([0.0, 0.3]),
        anchors: &[("text", [0.0, 0.3]), ("hlines nw", [-0.21, 0.09]), ("hlines ne", [0.21, 0.09]), ("hlines sw", [-0.21, -0.09]), ("hlines se", [0.21, -0.09])],
        ..node(
            "proximeter",
            &[
                Part::Pole(&[M(-0.3, 0.0), L(0.0, 0.3), L(0.3, 0.0), L(0.0, -0.3), Z]),
                Part::Thick(&[M(-0.3, 0.0), L(0.0, 0.3), L(0.3, 0.0), L(0.0, -0.3), Z]),
                Part::Thin(&[M(-0.2101, 0.0901), L(0.2099, 0.0901), M(-0.2101, -0.09), L(0.2099, -0.09)]),
            ],
            [-0.3, 0.3, -0.3, 0.3],
        )
    },
    Node {
        anchors: &[("in", [-0.425, 0.0]), ("out 1", [0.425, 0.225]), ("out 2", [0.425, -0.225]), ("mid", [0.0, 0.0832])],
        ..node(
            "spdt",
            &[
                Part::Thin(&[M(0.4249, 0.225), L(0.1913, 0.225), M(0.4249, -0.225), L(0.1913, -0.225), M(-0.425, 0.0), L(-0.1913, 0.0)]),
                Part::Pole(&[
                    M(0.2312, 0.225),
                    C([0.2312, 0.2471], [0.2133, 0.265], [0.1913, 0.265]),
                    C([0.1691, 0.265], [0.1512, 0.2471], [0.1512, 0.225]),
                    C([0.1512, 0.2029], [0.1691, 0.1849], [0.1913, 0.1849]),
                    C([0.2133, 0.1849], [0.2312, 0.2029], [0.2312, 0.225]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2312, 0.225),
                    C([0.2312, 0.2471], [0.2133, 0.265], [0.1913, 0.265]),
                    C([0.1691, 0.265], [0.1512, 0.2471], [0.1512, 0.225]),
                    C([0.1512, 0.2029], [0.1691, 0.1849], [0.1913, 0.1849]),
                    C([0.2133, 0.1849], [0.2312, 0.2029], [0.2312, 0.225]),
                    Z,
                ]),
                Part::Pole(&[
                    M(0.2312, -0.225),
                    C([0.2312, -0.203], [0.2133, -0.1851], [0.1913, -0.1851]),
                    C([0.1691, -0.1851], [0.1512, -0.203], [0.1512, -0.225]),
                    C([0.1512, -0.2471], [0.1691, -0.265], [0.1913, -0.265]),
                    C([0.2133, -0.265], [0.2312, -0.2471], [0.2312, -0.225]),
                    Z,
                ]),
                Part::Thin(&[
                    M(0.2312, -0.225),
                    C([0.2312, -0.203], [0.2133, -0.1851], [0.1913, -0.1851]),
                    C([0.1691, -0.1851], [0.1512, -0.203], [0.1512, -0.225]),
                    C([0.1512, -0.2471], [0.1691, -0.265], [0.1913, -0.265]),
                    C([0.2133, -0.265], [0.2312, -0.2471], [0.2312, -0.225]),
                    Z,
                ]),
                Part::Pole(&[
                    M(-0.1513, 0.0),
                    C([-0.1513, 0.0221], [-0.1692, 0.0399], [-0.1913, 0.0399]),
                    C([-0.2134, 0.0399], [-0.2313, 0.0221], [-0.2313, 0.0]),
                    C([-0.2313, -0.0221], [-0.2134, -0.0399], [-0.1913, -0.0399]),
                    C([-0.1692, -0.0399], [-0.1513, -0.0221], [-0.1513, 0.0]),
                    Z,
                ]),
                Part::Thin(&[
                    M(-0.1513, 0.0),
                    C([-0.1513, 0.0221], [-0.1692, 0.0399], [-0.1913, 0.0399]),
                    C([-0.2134, 0.0399], [-0.2313, 0.0221], [-0.2313, 0.0]),
                    C([-0.2313, -0.0221], [-0.2134, -0.0399], [-0.1913, -0.0399]),
                    C([-0.1692, -0.0399], [-0.1513, -0.0221], [-0.1513, 0.0]),
                    Z,
                ]),
                Part::Thick(&[M(-0.1513, 0.0), L(0.1799, 0.1737)]),
            ],
            [-0.425, 0.425, -0.225, 0.225],
        )
    },
    // symbols
    node("dc symbol", &[Part::Thick(&[M(-0.21, 0.105), L(0.21, 0.105), M(-0.21, -0.105), L(-0.07, -0.105), M(0.07, -0.105), L(0.21, -0.105)])], [-0.21, 0.21, -0.105, 0.105]),
    node(
        "ac symbol",
        &[Part::Thick(&[
            M(-0.21, 0.0),
            C([-0.1758, 0.0537], [-0.143, 0.105], [-0.105, 0.105]),
            C([-0.067, 0.105], [-0.0342, 0.0537], [0.0, 0.0]),
            C([0.0342, -0.0537], [0.067, -0.105], [0.105, -0.105]),
            C([0.143, -0.105], [0.1757, -0.0537], [0.21, 0.0]),
        ])],
        [-0.21, 0.21, -0.105, 0.105],
    ),
];

const CURRARROW: &[Op] = &[M(-0.0437, 0.0), L(-0.0437, -0.05), L(0.0625, 0.0), L(-0.0437, 0.05), Z];
const FLOWHEAD: &[Op] = &[M(0.1437, 0.0), L(0.1437, -0.05), L(0.25, 0.0), L(0.1437, 0.05), Z];

/// The terminals: drawn by their shape and filled (with the line color, or white unless filled),
/// `nodes width` across the half.
pub const TERMINALS: &[&str] = &["circ", "ocirc", "diamondpole", "odiamondpole", "squarepole", "osquarepole", "rectjoinfill"];

pub fn find(name: &str) -> Option<&'static Node> {
    NODES.iter().chain(transistors::TRANSISTORS).chain(tubes::TUBES).find(|n| n.name == name)
}

/// The transistors' and tubes' options that change their drawing, in the order of their variants'
/// names (`npn|bodydiode`), and the ones with a value.
pub const VARIANT_FLAGS: &[&str] = &[
    "filament",
    "nocathode",
    "fullcathode",
    "nixieanode",
    "anodedot",
    "nogrid",
    "schottky base",
    "bodydiode",
    "photo",
    "tr circle",
    "nobase",
    "bulk",
    "emptycircle",
    "nocircle",
    "arrowmos",
    "solderdot",
    "fdsoi",
    "fdsoidot",
    "doublegate",
];
pub const VARIANT_VALUES: &[&str] = &["collectors", "emitters", "bjt pins width", "bjt multi height"];

/// The transistors' and tubes' shapes (`npn`, `triode`…), without their variants.
pub fn variant_shapes() -> impl Iterator<Item = &'static str> {
    transistors::TRANSISTORS.iter().chain(tubes::TUBES).map(|n| n.name).filter(|n| !n.contains('|'))
}

/// The transistors' shapes, without their variants.
pub fn transistor_shapes() -> impl Iterator<Item = &'static str> {
    transistors::TRANSISTORS.iter().map(|n| n.name).filter(|n| !n.contains('|'))
}

/// The variant of transistor or tube `base` its options ask for, or the plain one (and a warning) when
/// voltik has not measured that combination.
fn variant(cx: &mut Context, base: &str) -> Result<Option<&'static Node>, Error> {
    let mut name = base.to_string();
    for flag in VARIANT_FLAGS {
        if setting(cx, &format!("tr/{flag}"))? == "true" {
            name.push('|');
            name.push_str(flag);
        }
    }
    for key in VARIANT_VALUES {
        let value = setting(cx, &format!("tr/{key}"))?;
        if !value.is_empty() {
            name.push_str(&format!("|{key}={value}"));
        }
    }
    match find(&name) {
        Some(n) => Ok(Some(n)),
        None => {
            cx.warn(format!("voltik: {} not drawn yet; drawn as a plain {base}", name.replace('|', ", ")));
            Ok(find(base))
        }
    }
}

/// `voltik/shapes`: declares every node component to tikra.
pub fn k_shapes(cx: &mut Context, _v: Option<Tokens>) -> Result<(), Error> {
    for n in NODES {
        cx.declare_shape(n.name, shape);
    }
    for name in variant_shapes() {
        cx.declare_shape(name, shape);
    }
    for t in TERMINALS {
        cx.declare_shape(t, shape);
    }
    Ok(())
}

/// The shape of a node component, for one node.
fn shape(cx: &mut Context, input: &ShapeInput) -> Result<NodeShape, Error> {
    let length = setting(cx, "bipoles/length")?;
    let unit = cx.length(Tokens::text(&length)).unwrap_or(1.4);
    if TERMINALS.contains(&input.name.as_str()) {
        let half = crate::bipole::number(cx, "nodes width", 0.04) * unit;
        return Ok(terminal(&input.name, half, input.line_width));
    }
    let n = if variant_shapes().any(|t| t == input.name) { variant(cx, &input.name)? } else { find(&input.name) };
    let Some(n) = n else { return Ok(NodeShape::default()) };
    let unit = match n.class() {
        Some(class) => unit * crate::bipole::number(cx, &format!("{class}/scale"), 1.0),
        None => unit,
    };
    let at = |p: [f64; 2]| Point::new(p[0] * unit, p[1] * unit);
    let w = input.line_width;
    let filled = !cx.macro_text("voltik@fill").is_empty();
    let paths = n
        .parts
        .iter()
        .map(|part| {
            let (ops, paint) = match *part {
                Part::Thin(ops) => (ops, ShapePaint::Stroke(w)),
                Part::Thick(ops) => (ops, ShapePaint::Stroke(2.0 * w)),
                Part::Thicker(ops) => (ops, ShapePaint::Stroke(3.0 * w)),
                Part::Fill(ops) => (ops, ShapePaint::Fill),
                Part::Solid(ops) => (ops, ShapePaint::Solid),
                Part::Pole(ops) => (ops, ShapePaint::Background),
                Part::Channel(ops) => (ops, if filled { ShapePaint::Fill } else { ShapePaint::Solid }),
                Part::Stroke(k, ops) => (ops, ShapePaint::Stroke(k * w)),
            };
            ShapePath { segments: segments(ops, unit), paint }
        })
        .collect();
    let [left, right, bottom, top] = n.bounds;
    let mut anchors = compass(left * unit, right * unit, bottom * unit, top * unit);
    anchors.extend(n.anchors.iter().map(|(name, p)| (name.to_string(), at(*p))));
    // the text: its box's origin (the left end of its baseline), from where its side goes
    let sy = input.inner_sep.y;
    let text = match n.text {
        Text::Center => Point::new(-input.width / 2.0, (input.depth - input.height) / 2.0),
        Text::Above(p) => Point::new(p[0] * unit - input.width / 2.0, p[1] * unit + sy + input.depth),
        Text::Below(p) => Point::new(p[0] * unit - input.width / 2.0, p[1] * unit - sy - input.height),
        Text::AboveLeft(p) => Point::new(p[0] * unit - input.width, p[1] * unit + sy + input.depth),
        Text::AboveRight(p) => Point::new(p[0] * unit, p[1] * unit + sy + input.depth),
        Text::Right(p) => Point::new(p[0] * unit, p[1] * unit + (input.depth - input.height) / 2.0),
        Text::Middle(p) => Point::new(p[0] * unit - input.width / 2.0, p[1] * unit + (input.depth - input.height) / 2.0),
    };
    let mut extent: Vec<Point> = n.extent.iter().map(|p| at(*p)).collect();
    if n.text_counts && (input.width > 0.0 || input.height + input.depth > 0.0) {
        let (sx, sy) = (input.inner_sep.x, input.inner_sep.y);
        extent.push(Point::new(text.x - sx, text.y - input.depth - sy));
        extent.push(Point::new(text.x + input.width + sx, text.y + input.height + sy));
    }
    Ok(NodeShape { center: Point::new(0.0, 0.0), anchors, border: ShapeBorder::Point, text, paths, extent })
}

/// A terminal `half` (cm) across the half, with lines `w` wide.
fn terminal(name: &str, half: f64, w: f64) -> NodeShape {
    let p = Point::new;
    let k = 0.5523 * half;
    let circle = vec![
        PathSegment::MoveTo(p(half, 0.0)),
        PathSegment::CurveTo(p(half, k), p(k, half), p(0.0, half)),
        PathSegment::CurveTo(p(-k, half), p(-half, k), p(-half, 0.0)),
        PathSegment::CurveTo(p(-half, -k), p(-k, -half), p(0.0, -half)),
        PathSegment::CurveTo(p(k, -half), p(half, -k), p(half, 0.0)),
        PathSegment::Close,
    ];
    let d = half * std::f64::consts::SQRT_2;
    let diamond = vec![PathSegment::MoveTo(p(0.0, d)), PathSegment::LineTo(p(-d, 0.0)), PathSegment::LineTo(p(0.0, -d)), PathSegment::LineTo(p(d, 0.0)), PathSegment::Close];
    let square = vec![
        PathSegment::MoveTo(p(-half, -half)),
        PathSegment::LineTo(p(half, -half)),
        PathSegment::LineTo(p(half, half)),
        PathSegment::LineTo(p(-half, half)),
        PathSegment::Close,
    ];
    let s = w / 2.0;
    let dot = vec![PathSegment::MoveTo(p(-s, -s)), PathSegment::LineTo(p(s, -s)), PathSegment::LineTo(p(s, s)), PathSegment::LineTo(p(-s, s)), PathSegment::Close];
    let (outline, open, r) = match name {
        "circ" => (circle, false, half),
        "ocirc" => (circle, true, half),
        "diamondpole" => (diamond, false, d),
        "odiamondpole" => (diamond, true, d),
        "squarepole" => (square, false, half),
        "osquarepole" => (square, true, half),
        _ => (dot, false, s),
    };
    let mut anchors = compass(-r, r, -r, r);
    anchors.push(("text".into(), p(0.0, 0.0)));
    let border = ShapeBorder::Outline(outline_points(&outline));
    let fill = if open { ShapePaint::Background } else { ShapePaint::Solid };
    let paths = if name == "rectjoinfill" {
        vec![ShapePath { segments: outline, paint: fill }]
    } else {
        vec![ShapePath { segments: outline.clone(), paint: fill }, ShapePath { segments: outline, paint: ShapePaint::Stroke(w) }]
    };
    NodeShape { center: p(0.0, 0.0), anchors, border, text: p(0.0, 0.0), paths, extent: Vec::new() }
}

/// The points of an outline, its curves by their ends (enough for a border).
fn outline_points(segments: &[PathSegment]) -> Vec<Point> {
    let mut points = Vec::new();
    for s in segments {
        match *s {
            PathSegment::MoveTo(p) | PathSegment::LineTo(p) => points.push(p),
            PathSegment::CurveTo(a, b, c) => {
                // the middle of the curve too, so a circle's border is round enough
                let last = *points.last().unwrap_or(&c);
                let m = Point::new(0.125 * last.x + 0.375 * a.x + 0.375 * b.x + 0.125 * c.x, 0.125 * last.y + 0.375 * a.y + 0.375 * b.y + 0.125 * c.y);
                points.extend([m, c]);
            }
            PathSegment::Close => {}
        }
    }
    points
}

/// The compass anchors of a box, and their short names (`n`, `ne`, `left`…).
fn compass(left: f64, right: f64, bottom: f64, top: f64) -> Vec<(String, Point)> {
    let (x, y) = ((left + right) / 2.0, (bottom + top) / 2.0);
    let p = Point::new;
    let mut out = Vec::new();
    for (names, at) in [
        (&["north", "n"][..], p(x, top)),
        (&["south", "s"], p(x, bottom)),
        (&["east", "e", "right"], p(right, y)),
        (&["west", "w", "left"], p(left, y)),
        (&["north east", "ne"], p(right, top)),
        (&["north west", "nw"], p(left, top)),
        (&["south east", "se"], p(right, bottom)),
        (&["south west", "sw"], p(left, bottom)),
    ] {
        for n in names {
            out.push((n.to_string(), at));
        }
    }
    out
}

/// Ops in `bipoles/length` as segments in cm.
fn segments(ops: &[Op], unit: f64) -> Vec<PathSegment> {
    let p = |x: f64, y: f64| Point::new(x * unit, y * unit);
    ops.iter()
        .map(|op| match *op {
            Op::Move(x, y) => PathSegment::MoveTo(p(x, y)),
            Op::Line(x, y) => PathSegment::LineTo(p(x, y)),
            Op::Curve(a, b, c) => PathSegment::CurveTo(p(a[0], a[1]), p(b[0], b[1]), p(c[0], c[1])),
            Op::Close => PathSegment::Close,
        })
        .collect()
}
