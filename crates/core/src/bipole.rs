//! Path-style components: `\draw (0,0) to[R=$R_1$, o-*] (2,0);`.
//!
//! A component key (`R`, `C`, …) records which component the `to` draws and sets its `to path`. When
//! the path is built, `voltik/place` reads the two ends, puts the component halfway, turned along the
//! line, draws its outline with the thick stroke of its class, and leaves the rest of the path in
//! `\voltik@path`: the leads up to the component, the poles and the labels, as nodes on the path so
//! that they are drawn over the wires.

use tikra::extension::{Context, Error, Point, Tokens, Transform};

use crate::components::{self, Look, Op};
use crate::label;

/// The `to path` of every component: place it, then the rest of the path.
const TO_PATH: &str = "to path={\\pgfextra{\\pgfkeys{/tikz/voltik/place}}\\voltik@path}";

/// A circuitikz setting, `\ctikzvalof{key}`, as text.
pub fn setting(cx: &mut Context, key: &str) -> Result<String, Error> {
    cx.expand(Tokens::text(&format!("\\pgfkeysvalueof{{/tikz/circuitikz/{key}}}")))
}

/// A circuitikz setting read as a number; `default` when empty or not a number.
fn number(cx: &mut Context, key: &str, default: f64) -> f64 {
    match setting(cx, key) {
        Ok(s) if !s.is_empty() && s != "none" => cx.number(Tokens::text(&s)).unwrap_or(default),
        _ => default,
    }
}

/// Whether a key's value is missing (`to[R]`): pgfkeys passes `\pgfkeysnovalue` to a style.
fn present(value: &Option<Tokens>) -> Option<Tokens> {
    let v = value.as_ref()?.trim();
    let text = v.to_text();
    if text.is_empty() || text.trim() == "\\pgfkeysnovalue" {
        return None;
    }
    Some(v)
}

/// `voltik/component={name}{value}`: the `to` draws this component; the value, if any, is its label.
pub fn k_component(cx: &mut Context, v: Option<Tokens>) -> Result<(), Error> {
    let a = v.unwrap_or_default().arguments(2);
    let name = cx.expand(a[0].clone())?;
    cx.define("voltik@kind", Tokens::text(&name));
    if let Some(value) = present(&Some(a[1].clone())) {
        label::set(cx, "l", 1, value);
    }
    cx.keys(Tokens::text(TO_PATH))
}

/// `voltik/pole={start}{end}`: the poles at the two ends (`o`, `*` or none).
pub fn k_pole(cx: &mut Context, v: Option<Tokens>) -> Result<(), Error> {
    let a = v.unwrap_or_default().arguments(2);
    cx.define("voltik@ps", a[0].clone());
    cx.define("voltik@pe", a[1].clone());
    Ok(())
}

/// `voltik/flag=name`: `mirror` and `invert`, flipped each time they are given.
pub fn k_flag(cx: &mut Context, v: Option<Tokens>) -> Result<(), Error> {
    let name = cx.expand(v.unwrap_or_default())?;
    let macro_name = format!("voltik@{name}");
    let on = cx.macro_text(&macro_name) == "1";
    cx.define(&macro_name, Tokens::text(if on { "0" } else { "1" }));
    Ok(())
}

/// The component the `to` draws: `@resistor` and `@inductor` follow the current style.
fn component(cx: &mut Context) -> Result<Option<&'static components::Component>, Error> {
    let kind = cx.macro_text("voltik@kind");
    let name = match kind.as_str() {
        "@resistor" => match setting(cx, "resistor")?.as_str() {
            "european" => "generic".to_string(),
            _ => "resistor".to_string(),
        },
        "@inductor" => {
            let style = setting(cx, "inductor")?;
            if style != "cute" {
                cx.warn(format!("voltik: {style} inductors are not drawn yet; drawn as cute inductors"));
            }
            "cute inductor".to_string()
        }
        _ => kind,
    };
    let found = components::find(&name);
    if found.is_none() {
        cx.warn(format!("voltik: unknown component {name}; drawn as a short"));
    }
    Ok(found)
}

/// The inverse of an affine transform (the identity if it is singular).
fn inverse(t: &Transform) -> Transform {
    let det = t.a * t.d - t.b * t.c;
    if det.abs() < 1e-12 {
        return Transform::default();
    }
    let (a, b, c, d) = (t.d / det, -t.b / det, -t.c / det, t.a / det);
    Transform { a, b, c, d, e: -(a * t.e + c * t.f), f: -(b * t.e + d * t.f) }
}

/// A canvas point as a picture coordinate: `(x cm, y cm)`, which the path's transform maps back.
pub struct Frame {
    to_picture: Transform,
}

impl Frame {
    pub fn coordinate(&self, p: Point) -> String {
        let q = self.to_picture.apply(p);
        format!("({}cm,{}cm)", num(q.x), num(q.y))
    }
}

fn num(x: f64) -> String {
    let s = format!("{:.5}", x);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" {
        "0".into()
    } else {
        s.into()
    }
}

/// The geometry of a placed component, on the canvas (cm).
pub struct Placed {
    pub mid: Point,
    /// unit vector from start to end, and its left normal
    pub along: Point,
    pub normal: Point,
    /// the direction of the line, in degrees
    pub angle: f64,
    pub width: f64,
    pub height: f64,
    pub mirror: bool,
    pub invert: bool,
}

impl Placed {
    /// A point of the component's frame on the canvas.
    pub fn at(&self, x: f64, y: f64) -> Point {
        let x = if self.invert { -x } else { x };
        let y = if self.mirror { -y } else { y };
        self.mid.plus(self.along.scale(x)).plus(self.normal.scale(y))
    }
}

/// `voltik/place`, run inside the `to path`: draws the component and defines `\voltik@path`.
pub fn k_place(cx: &mut Context, _v: Option<Tokens>) -> Result<(), Error> {
    let start_name = cx.expand(Tokens::text("\\tikztostart"))?;
    let end_name = cx.expand(Tokens::text("\\tikztotarget"))?;
    let start = cx.point(&start_name)?;
    let end = cx.point(&end_name)?;
    let frame = Frame { to_picture: inverse(&cx.transform()) };

    let length_setting = setting(cx, "bipoles/length")?;
    let unit = cx.length(Tokens::text(&length_setting)).unwrap_or(1.4);
    let comp = component(cx)?;
    let (look, class) = comp.map_or((Look::Wire, components::Class::Default), |c| (c.look, c.class));
    let scale = number(cx, &format!("{}/scale", class.name()), 1.0);
    let base_width = comp.map_or(0.0, |c| c.width);
    let width = number(cx, &format!("{}/width", class.name()), base_width) * unit * scale;
    let height = comp.map_or(0.0, |c| c.height) * unit * scale;
    let thickness = match setting(cx, &format!("{}/thickness", class.name())).unwrap_or_default().as_str() {
        "" | "none" => number(cx, "bipoles/thickness", 2.0),
        _ => number(cx, &format!("{}/thickness", class.name()), 2.0),
    };
    let thick = thickness * cx.line_width();
    let count = match look {
        Look::Coil => number(cx, "inductors/coils", 5.0),
        _ => number(cx, "resistors/zigs", 3.0),
    }
    .round()
    .max(1.0) as usize;

    let d = end.minus(start);
    let length = d.norm().max(1e-9);
    let along = d.scale(1.0 / length);
    let placed = Placed {
        mid: start.lerp(end, 0.5),
        along,
        normal: Point::new(-along.y, along.x),
        angle: d.y.atan2(d.x).to_degrees(),
        width,
        height,
        mirror: cx.macro_text("voltik@mirror") == "1",
        invert: cx.macro_text("voltik@invert") == "1",
    };

    // the outline, a path of its own with the class's thick stroke
    let ops = components::outline(look, width, height, thick, count);
    if !ops.is_empty() {
        let join = match look {
            Look::ZigZag | Look::Coil => "line join=bevel,",
            _ => "",
        };
        let fill = match look {
            Look::Box | Look::Plates => "",
            _ => "fill=none,",
        };
        let mut s = format!("\\draw[-,solid,{join}{fill}miter limit=10,line width={}cm]", num(thick));
        for op in &ops {
            match *op {
                Op::Move(x, y) => s.push_str(&format!(" {}", frame.coordinate(placed.at(x, y)))),
                Op::Line(x, y) => s.push_str(&format!(" -- {}", frame.coordinate(placed.at(x, y)))),
                Op::Curve(c1, c2, p) => s.push_str(&format!(
                    " .. controls {} and {} .. {}",
                    frame.coordinate(placed.at(c1[0], c1[1])),
                    frame.coordinate(placed.at(c2[0], c2[1])),
                    frame.coordinate(placed.at(p[0], p[1]))
                )),
                Op::Close => s.push_str(" -- cycle"),
            }
        }
        s.push(';');
        cx.run(Tokens::text(&s))?;
    }

    // the leads, then what goes over them
    let mut path = match look {
        Look::Gap => "(\\tikztotarget)".to_string(),
        Look::Wire => "-- (\\tikztotarget)".to_string(),
        // the leads stop at the component's ends, whichever way it is turned
        _ => format!(
            "-- {} {} -- (\\tikztotarget)",
            frame.coordinate(placed.mid.minus(placed.along.scale(width / 2.0))),
            frame.coordinate(placed.mid.plus(placed.along.scale(width / 2.0)))
        ),
    };
    path.push_str(&label::nodes(cx, &placed, &frame, unit)?);
    path.push_str(&poles(cx, &frame, start, end, unit));
    path.push_str(" \\tikztonodes");
    cx.define("voltik@path", Tokens::text(&path));
    Ok(())
}

/// The poles at the ends: `o` a circle filled in white, `*` a filled one, both `0.04` units across
/// the radius, drawn with the wire's stroke.
fn poles(cx: &mut Context, frame: &Frame, start: Point, end: Point, unit: f64) -> String {
    let size = 2.0 * 0.04 * unit;
    let mut s = String::new();
    for (which, at) in [("voltik@ps", start), ("voltik@pe", end)] {
        let style = match cx.macro_text(which).as_str() {
            "o" => "fill=white",
            "*" => "fill",
            _ => continue,
        };
        s.push_str(&format!(" node[circle,draw,solid,{style},inner sep=0pt,outer sep=0pt,minimum size={}cm] at {} {{}}", num(size), frame.coordinate(at)));
    }
    s
}
