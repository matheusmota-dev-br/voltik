//! Path-style components: `\draw (0,0) to[R=$R_1$, o-*] (2,0);`.
//!
//! A component key (`R`, `C`, …) records which component the `to` draws and sets its `to path`. When
//! the path is built, `voltik/place` reads the two ends, puts the component halfway, turned along the
//! line, draws its outline with the thick stroke of its class, and leaves the rest of the path in
//! `\voltik@path`: the leads up to the component, the poles and the labels, as nodes on the path so
//! that they are drawn over the wires.

use tikra::extension::{Context, Error, Point, Tokens, Transform, PT};

use crate::components::{self, Leads, Op, Part, Source, Target};
use crate::{label, vif};

/// The `to path` of every component: place it, then the rest of the path.
const TO_PATH: &str = "to path={\\pgfextra{\\pgfkeys{/tikz/voltik/place}}\\voltik@path}";

/// A circuitikz setting, `\ctikzvalof{key}`, as text.
pub fn setting(cx: &mut Context, key: &str) -> Result<String, Error> {
    cx.expand(Tokens::text(&format!("\\pgfkeysvalueof{{/tikz/circuitikz/{key}}}")))
}

/// A circuitikz setting read as a number; `default` when empty or not a number.
pub fn number(cx: &mut Context, key: &str, default: f64) -> f64 {
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
    let a = v.unwrap_or_default().arguments(3);
    let name = cx.expand(a[0].clone())?;
    let variant = cx.expand(a[2].clone())?;
    // a spark gap's circle keeps the dot given before it
    if !(name == "sparkgap/circle" && cx.macro_text("voltik@kind") == "sparkgap/dot") {
        cx.define("voltik@kind", Tokens::text(&name));
    }
    if let Some(value) = present(&Some(a[1].clone())) {
        // a source's value is its voltage or its current, with the key's variant (`I<=…`)
        let kind = match components::target(&name) {
            Some(Target::Fixed(n) | Target::Voltage(n, _) | Target::Current(n, _)) => components::find(n).map(|c| c.source()),
            _ => None,
        };
        match kind {
            Some(Source::Voltage) => vif::set(cx, "v", &variant, value),
            Some(Source::Current) => vif::set(cx, "i", &variant, value),
            _ => label::set(cx, "l", 1, value),
        }
    }
    cx.keys(Tokens::text(TO_PATH))
}

/// `voltik/pole={start}{end}`: the poles at the two ends, as `bipole nodes` gives them: a shape
/// (`circ`, `ocirc`, `diamondpole`…) and options for it (`ocirc, fill=red`), or `none`.
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

/// The component the `to` draws: the short names (`R`, `vR`, `L`) follow the current style.
fn component(cx: &mut Context) -> Result<Option<&'static components::Component>, Error> {
    let kind = cx.macro_text("voltik@kind");
    let name = match components::target(&kind) {
        Some(Target::Fixed(name)) => name,
        Some(Target::Resistor(american, european)) => match setting(cx, "resistor")?.as_str() {
            "european" => european,
            _ => american,
        },
        Some(Target::Inductor(cute, american, european)) => match setting(cx, "inductor")?.as_str() {
            "american" => american,
            "european" => european,
            _ => cute,
        },
        Some(Target::Voltage(european, american)) => match setting(cx, "voltage")?.as_str() {
            "american" | "raised" => american,
            _ => european,
        },
        Some(Target::Current(european, american)) => match setting(cx, "current")?.as_str() {
            "american" => american,
            _ => european,
        },
        Some(Target::Diode(empty, full, stroke)) => match setting(cx, "diode")?.as_str() {
            "full" => full,
            "stroke" => stroke,
            _ => empty,
        },
        None => {
            cx.warn(format!("voltik: unknown component {kind}; drawn as a short"));
            return Ok(None);
        }
    };
    // the other shapes of thyristors, LEDs, photodiodes and whiskered diodes, by their settings
    if setting(cx, "thyristor style")? == "compact" {
        if let Some(compact) = components::find(&format!("compact {name}")) {
            return Ok(Some(compact));
        }
    }
    for (key, value, suffix) in [("led arrows", "cathode", "@cathode"), ("pd arrows", "cathode", "@cathode"), ("diode whiskers", "straight", "@straight")] {
        let fits = match key {
            "led arrows" => name.ends_with(" led"),
            "pd arrows" => name.ends_with(" photodiode"),
            _ => name.contains("ZZener") || name.contains("TVS"),
        };
        if fits && setting(cx, key)? == value {
            return Ok(components::find(&format!("{name}{suffix}")));
        }
    }
    // a three-pins jumper's connections
    if name == "three-pins jumper" {
        let connections = setting(cx, "tjumper connections")?;
        if let Some(jumper) = components::find(&format!("{name}@{connections}")) {
            return Ok(Some(jumper));
        }
    }
    Ok(components::find(name))
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
    /// how far the component reaches above and below the line (its north and south), unmirrored
    pub top: f64,
    pub bottom: f64,
    pub mirror: bool,
    pub invert: bool,
}

impl Placed {
    /// How far the component reaches on one side of the line: `side` 1 is the side of the normal.
    pub fn reach(&self, side: f64) -> f64 {
        if (side > 0.0) != self.mirror {
            self.top
        } else {
            self.bottom
        }
    }

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
    let known = component(cx)?;
    let comp = known.or_else(|| components::find("short")).expect("short is a component");
    let class = comp.class.name();
    let scale = number(cx, &format!("{class}/scale"), 1.0);
    // the class's width and height settings, when given, are the same for all its components (but
    // for the ones that are wider than the others, by their ratio)
    let width = optional(cx, &format!("{class}/width")).map_or(comp.width, |w| w * comp.ratio) * unit * scale;
    let height = optional(cx, &format!("{class}/height")).unwrap_or(comp.height) * unit * scale;
    let thickness = optional(cx, &format!("{class}/thickness")).unwrap_or_else(|| number(cx, "bipoles/thickness", 2.0));
    let thick = thickness * cx.line_width();

    // cm to a unit of the default size, along and across the line
    let x = if comp.width > 0.0 { width / comp.width } else { 0.0 };
    let y = if comp.height > 0.0 { height / comp.height } else { 0.0 };

    let d = end.minus(start);
    let length = d.norm().max(1e-9);
    let along = d.scale(1.0 / length);
    let placed = Placed {
        mid: start.lerp(end, 0.5),
        along,
        normal: Point::new(-along.y, along.x),
        angle: d.y.atan2(d.x).to_degrees(),
        width,
        top: comp.top * y,
        bottom: comp.bottom * y,
        mirror: cx.macro_text("voltik@mirror") == "1",
        invert: cx.macro_text("voltik@invert") == "1",
    };
    let look = Look {
        placed: &placed,
        frame: &frame,
        x,
        y,
        unit: unit * scale,
        thick,
        modifier: thick * number(cx, &format!("{class}/modifier thickness"), 1.0),
        thin: cx.line_width(),
        fill: fill(cx),
    };

    if !comp.parts.is_empty() {
        let mut code = String::new();
        if comp.extent > 0.0 {
            // room CircuiTikZ makes around it, beyond what it draws
            code.push_str(&format!("\\path {} {};", look.at([0.0, comp.extent]), look.at([0.0, -comp.extent])));
        }
        for part in comp.parts {
            code.push_str(&draw_part(cx, &look, comp, *part));
        }
        cx.run(Tokens::text(&code))?;
    }
    let name = cx.macro_text("voltik@name");
    if !name.is_empty() {
        let code = anchors(cx, &look, comp, &name);
        cx.run(Tokens::text(&code))?;
    }

    // the leads, then what goes over them
    let mut path = match comp.leads {
        Leads::Gap => "(\\tikztotarget)".to_string(),
        Leads::Wire => "-- (\\tikztotarget)".to_string(),
        // the leads stop at the component's ends, whichever way it is turned
        Leads::Ends => format!(
            "-- {} {} -- (\\tikztotarget)",
            frame.coordinate(placed.mid.minus(placed.along.scale(width / 2.0))),
            frame.coordinate(placed.mid.plus(placed.along.scale(width / 2.0)))
        ),
    };
    path.push_str(&label::nodes(cx, &placed, &frame, unit)?);
    // `t=…`: upright in the middle of the component (below it on a screen's meter)
    let t = cx.macro_body("voltik@t");
    if !t.to_text().trim().is_empty() {
        let low = if comp.name == "smeter" { 0.15 } else { 0.0 };
        let at = placed.mid.minus(placed.normal.scale(low * y));
        path.push_str(&format!(" node[anchor=center] at {} {{{}}}", frame.coordinate(at), label::units(&t.to_text())));
    }
    let span = vif::Span { placed: &placed, frame: &frame, start, length, unit: unit * scale, y, half: width / 2.0, volts_half: comp.volts().half.map_or(width / 2.0, |h| h * x) };
    // a component voltik does not draw yet gets no voltage nor current: its convention is its own
    let drawn = if known.is_some() { vif::vif(cx, comp, &span, &name)? } else { vif::Drawn::default() };
    if !drawn.draw.is_empty() {
        cx.run(Tokens::text(&drawn.draw))?;
    }
    path.push_str(&drawn.nodes);
    if !name.is_empty() {
        let mut code = String::new();
        for (point, at) in &drawn.points {
            code.push_str(&format!("\\coordinate ({name}-{point}) at {};", frame.coordinate(*at)));
        }
        cx.run(Tokens::text(&code))?;
        for (key, anchor) in &drawn.anchors {
            cx.define_text(&format!("voltik@anchor@{name}@{key}"), anchor, true);
        }
        // `\ctikzgetdirection{R1}`: the way the component goes, in degrees
        cx.define_text(&format!("voltik@direction@{name}"), &format!("{}", placed.angle.round()), true);
    }
    path.push_str(&poles(cx, &frame, start, end, unit));
    path.push_str(" \\tikztonodes");
    cx.define("voltik@path", Tokens::text(&path));
    Ok(())
}

/// A circuitikz setting read as a number, or `None` when it is not set (empty or `none`).
fn optional(cx: &mut Context, key: &str) -> Option<f64> {
    match setting(cx, key) {
        Ok(s) if !s.is_empty() && s != "none" => cx.number(Tokens::text(&s)).ok(),
        _ => None,
    }
}

/// A circuitikz setting read as a length, in cm; `default` when it is not one.
fn distance(cx: &mut Context, key: &str, default: f64) -> f64 {
    match setting(cx, key) {
        Ok(s) if !s.is_empty() => cx.length(Tokens::text(&s)).unwrap_or(default),
        _ => default,
    }
}

/// The `fill` given to the `to` (or its path), as the option of a filled part: `None` without one.
fn fill(cx: &Context) -> Option<String> {
    let f = cx.macro_text("voltik@fill");
    match f.as_str() {
        "" | "none" => None,
        "\\pgfkeysnovalue" => Some("fill".into()),
        c => Some(format!("fill={{{c}}}")),
    }
}

/// What a component's parts are drawn with: where it is, how its default size stretches (`x`, `y`)
/// and its strokes, in cm.
struct Look<'a> {
    placed: &'a Placed,
    frame: &'a Frame,
    x: f64,
    y: f64,
    /// cm to `bipoles/length`, at the class's scale
    unit: f64,
    thick: f64,
    modifier: f64,
    thin: f64,
    fill: Option<String>,
}

impl Look<'_> {
    /// A point of the component's default frame, as a picture coordinate.
    fn at(&self, p: [f64; 2]) -> String {
        self.frame.coordinate(self.canvas(p))
    }

    fn canvas(&self, p: [f64; 2]) -> Point {
        self.placed.at(p[0] * self.x, p[1] * self.y)
    }

    /// Ops of the default frame as a path.
    fn path(&self, ops: &[Op]) -> String {
        self.path_scaled(ops, [self.x, self.y])
    }

    /// Ops already in cm (a zig-zag, a coil) as a path.
    fn path_cm(&self, ops: &[Op]) -> String {
        self.path_scaled(ops, [1.0, 1.0])
    }

    fn path_scaled(&self, ops: &[Op], k: [f64; 2]) -> String {
        let at = |p: [f64; 2]| self.frame.coordinate(self.placed.at(p[0] * k[0], p[1] * k[1]));
        let mut s = String::new();
        for op in ops {
            match *op {
                Op::Move(x, y) => s.push_str(&format!(" {}", at([x, y]))),
                Op::Line(x, y) => s.push_str(&format!(" -- {}", at([x, y]))),
                Op::Curve(c1, c2, p) => s.push_str(&format!(" .. controls {} and {} .. {}", at(c1), at(c2), at(p))),
                Op::Close => s.push_str(" -- cycle"),
            }
        }
        s
    }
}

/// The TeX that draws one part of a component.
fn draw_part(cx: &mut Context, look: &Look, comp: &components::Component, part: Part) -> String {
    let stroke = |width: f64, extra: &str| format!("\\draw[-,solid,{extra}miter limit=10,line width={}cm]", num(width));
    match part {
        Part::Fill(ops) => match &look.fill {
            Some(f) => format!("\\path[{f}]{};", look.path(ops)),
            None => String::new(),
        },
        Part::Body(ops) => {
            let fill = look.fill.clone().unwrap_or_else(|| "fill=none".into());
            format!("{}{};", stroke(look.thick, &format!("{fill},")), look.path(ops))
        }
        Part::Line(ops) => format!("{}{};", stroke(look.thick, "fill=none,"), look.path(ops)),
        Part::Modifier(ops) => format!("{}{};", stroke(look.modifier, "fill=none,"), look.path(ops)),
        Part::Stroke(k, ops) => format!("{}{};", stroke(k / 2.0 * look.thick, "fill=none,"), look.path(ops)),
        Part::Paint(color, ops) => format!("\\path[fill={{{color}}}]{};", look.path(ops)),
        Part::Pole(ops) => format!("\\path[{}]{};", look.fill.clone().unwrap_or_else(|| "fill=white".into()), look.path(ops)),
        Part::Lever(k, ops) => format!("{}{};", stroke(k * look.thin, "fill=none,line cap=round,"), look.path(ops)),
        Part::Thin(ops) => format!("{}{};", stroke(look.thin, "fill=none,"), look.path(ops)),
        Part::Solid(ops) => format!("\\fill[color=.]{};", look.path(ops)),
        Part::Arrow { from, to, thin } => arrow(look, from, to, if thin { look.thin } else { look.modifier }),
        Part::Wiper { thin } => {
            let pos = number(cx, "wiper pos", 0.5);
            let x = (pos - 0.5) * comp.width;
            let top = comp.height / 2.0;
            arrow(look, [x, top + 0.25], [x, top], if thin { look.thin } else { look.modifier })
        }
        Part::ZigZag { half, high } => {
            let count = number(cx, "resistors/zigs", 3.0).round().max(1.0) as usize;
            let ops = components::zigzag(half * look.x, high * look.y, look.thick, count);
            format!("{}{};", stroke(look.thick, "line join=bevel,fill=none,"), look.path_cm(&ops))
        }
        Part::Coil | Part::Arches => {
            let (default, least) = if part == Part::Coil { (5.0, 2.0) } else { (4.0, 1.0) };
            let count = optional(cx, "inductors/coils").unwrap_or(default).round().max(least) as usize;
            let (half, h) = (comp.width * look.x / 2.0 + look.thick / 2.0, comp.height * look.y / 2.0);
            let ops = if part == Part::Coil { components::coil(half, h, count) } else { components::arches(half, h, count) };
            format!("{}{};", stroke(look.thick, "line join=bevel,fill=none,"), look.path_cm(&ops))
        }
        Part::Text { at, shift, size, text, turn } => {
            let at = [at[0] + shift[0] * look.unit / look.x.max(1e-9), at[1] + shift[1] * look.unit / look.y.max(1e-9)];
            // turned with the line, and with the component's mirror and inversion
            let base = look.placed.angle + turn;
            // upright text keeps its middle where it is on the component, wherever that turns
            let point = if turn == 0.0 {
                let pt = tikra::extension::PT;
                let chars = text.chars().filter(|c| c.is_alphanumeric()).count().max(1) as f64;
                let half = Point { x: 0.3 * size * chars * pt, y: 0.34 * size * pt };
                let middle = look.canvas([at[0] + half.x / look.x.max(1e-9), at[1] + half.y / look.y.max(1e-9)]);
                look.frame.coordinate(middle.minus(half))
            } else {
                look.at(at)
            };
            format!(
                "\\node[anchor=base west,inner sep=0pt,rotate={},font=\\fontsize{{{size}}}{{{}}}\\selectfont] at {} {{{text}}};",
                if turn == 0.0 { 0.0 } else { base },
                size * 1.2,
                point
            )
        }
    }
}

/// The anchors of a component named `name` (`to[R, name=R1]`), as coordinates `R1.north`,
/// `R1.wiper`…: the compass points of its band, and its own named points.
fn anchors(cx: &mut Context, look: &Look, comp: &components::Component, name: &str) -> String {
    let (w, top, bottom) = (comp.width / 2.0, comp.top, -comp.bottom);
    let mut points: Vec<(&str, [f64; 2])> = vec![
        ("center", [0.0, 0.0]),
        ("north", [0.0, top]),
        ("south", [0.0, bottom]),
        ("east", [w, 0.0]),
        ("west", [-w, 0.0]),
        ("right", [w, 0.0]),
        ("left", [-w, 0.0]),
        ("north east", [w, top]),
        ("north west", [-w, top]),
        ("south east", [w, bottom]),
        ("south west", [-w, bottom]),
        ("n", [0.0, top]),
        ("s", [0.0, bottom]),
        ("e", [w, 0.0]),
        ("w", [-w, 0.0]),
        ("ne", [w, top]),
        ("nw", [-w, top]),
        ("se", [w, bottom]),
        ("sw", [-w, bottom]),
    ];
    points.extend(comp.anchors.iter().copied());
    if comp.parts.iter().any(|p| matches!(p, Part::Wiper { .. })) {
        let x = (number(cx, "wiper pos", 0.5) - 0.5) * comp.width;
        points.extend([("wiper", [x, comp.height / 2.0 + 0.25]), ("tip", [x, comp.height / 2.0])]);
    }
    if comp.class == components::Class::Inductors {
        // the core lines go `core distance` above the coil; the dots of mutual inductance a little
        // beyond its corners; the middle tap is on the coil, above or below as its coils fall
        let unit = look.y.max(1e-9);
        let core = distance(cx, "bipoles/inductors/core distance", 2.0 * PT) / unit;
        let dx = distance(cx, "bipoles/inductors/dot x distance", 4.0 * PT) / unit;
        let dy = distance(cx, "bipoles/inductors/dot y distance", PT) / unit;
        let h = comp.height / 2.0;
        points.extend([("core west", [-w, h + core]), ("core east", [w, h + core])]);
        let (x, up, down) = (w * 1.1 + dx, comp.top * 1.1 + dy, -(comp.bottom * 1.1 + dy));
        points.extend([("ul dot", [-x, up]), ("ur dot", [x, up]), ("ll dot", [-x, down]), ("lr dot", [x, down])]);
        let coils = comp.parts.iter().find_map(|p| match p {
            Part::Coil => Some(optional(cx, "inductors/coils").unwrap_or(5.0).round()),
            Part::Arches => Some(optional(cx, "inductors/coils").unwrap_or(4.0).round()),
            _ => None,
        });
        let midtap = match coils {
            Some(n) if n as i64 % 2 == 1 => h,
            Some(_) if comp.parts.contains(&Part::Coil) => -comp.bottom,
            Some(_) => 0.0,
            None => comp.top,
        };
        points.push(("midtap", [0.0, midtap]));
    }
    let mut s = String::new();
    for (anchor, p) in points {
        s.push_str(&format!("\\coordinate ({name}.{anchor}) at {};", look.at(p)));
    }
    s
}

/// An arrow from `from` to `to` (default frame) with its head at `to`, for a stroke `width` cm wide.
fn arrow(look: &Look, from: [f64; 2], to: [f64; 2], width: f64) -> String {
    let (p0, tip) = (look.canvas(from), look.canvas(to));
    let d = tip.minus(p0);
    let u = d.scale(1.0 / d.norm().max(1e-9));
    let n = Point::new(-u.y, u.x);
    let size = components::arrow_length(width);
    let head = |b: f64, s: f64| look.frame.coordinate(tip.minus(u.scale(b * size)).plus(n.scale(s * size)));
    let mut s = format!(
        "\\draw[-,solid,line width={}cm] {} -- {};\\fill[color=.] {}",
        num(width),
        look.frame.coordinate(p0),
        look.frame.coordinate(tip.minus(u.scale(components::ARROW_INSET * size))),
        look.frame.coordinate(tip)
    );
    for c in components::ARROW_HEAD.chunks(3) {
        s.push_str(&format!(" .. controls {} and {} .. {}", head(c[0][0], c[0][1]), head(c[1][0], c[1][1]), head(c[2][0], c[2][1])));
    }
    s.push_str(" -- cycle;");
    s
}

/// The poles at the ends: circles, diamonds or squares `nodes width` (of `bipoles/length`) across
/// the half, drawn with the wire's stroke and filled with its color, or the open ones (`ocirc`) with
/// the component's fill (white without one).
fn poles(cx: &mut Context, frame: &Frame, start: Point, end: Point, unit: f64) -> String {
    let half = number(cx, "nodes width", 0.04) * unit;
    // the filled poles take the stroke's color; the open ones the component's fill, or white
    let closed = "color=.,fill";
    let open = fill(cx).unwrap_or_else(|| "fill=white".into());
    let mut s = String::new();
    for (which, at) in [("voltik@ps", start), ("voltik@pe", end)] {
        let spec = cx.macro_text(which);
        let (name, options) = spec.split_once(',').map_or((spec.trim(), ""), |(n, o)| (n.trim(), o.trim()));
        let (shape, size, fill) = match name {
            "circ" => ("circle", 2.0 * half, closed),
            "ocirc" => ("circle", 2.0 * half, open.as_str()),
            "squarepole" => ("rectangle", 2.0 * half, closed),
            "osquarepole" => ("rectangle", 2.0 * half, open.as_str()),
            "diamondpole" => ("rectangle,rotate=45", 2.0 * half, closed),
            "odiamondpole" => ("rectangle,rotate=45", 2.0 * half, open.as_str()),
            "" | "none" => continue,
            other => {
                cx.warn(format!("voltik: pole {other} is not drawn yet"));
                continue;
            }
        };
        let options = if options.is_empty() { String::new() } else { format!(",{options}") };
        s.push_str(&format!(" node[{shape},draw,solid,{fill},inner sep=0pt,outer sep=0pt,minimum size={}cm{options}] at {} {{}}", num(size), frame.coordinate(at)));
    }
    s
}
