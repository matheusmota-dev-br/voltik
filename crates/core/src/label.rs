//! Labels (`l`) and annotations (`a`) of path-style components.
//!
//! A label goes on one side of the component (above it when the path goes left to right; `l_` puts it
//! on the other side), an annotation on the other side by default. `label/align` says how the text
//! turns with the component: `straight` never, `rotate` always (along the line), `smart` (the default)
//! only when the line is far from horizontal and vertical.

use tikra::extension::{Context, Error, Point, Tokens};

use crate::bipole::{setting, Frame, Placed};

/// The space between the component's band and its label (`.75ex` of the 10 pt font, in cm).
const LABEL_SPACE: f64 = 0.75 * 4.305_56 * tikra::extension::PT;
/// How far out of the component's outline its labels start (the "border margin").
const MARGIN: f64 = 1.1;
/// Straight labels, from the band: the baseline of a label above, of a label below, and the text's
/// near side at the left and right (measured on TeX's drawings, in cm).
const ABOVE: f64 = 2.83 * tikra::extension::PT;
const BELOW: f64 = 10.03 * tikra::extension::PT;
const SIDE: f64 = 1.35 * tikra::extension::PT;

/// Records a label (`which` = `l`) or an annotation (`a`) on side `side` (1 above, -1 below, 0 the
/// default of its kind).
pub fn set(cx: &mut Context, which: &str, side: i32, text: Tokens) {
    cx.define(&format!("voltik@{which}"), text);
    cx.define(&format!("voltik@{which}side"), Tokens::text(&side.to_string()));
}

/// `voltik/label={which}{side}{text}`.
pub fn k_label(cx: &mut Context, v: Option<Tokens>) -> Result<(), Error> {
    let a = v.unwrap_or_default().arguments(3);
    let which = cx.expand(a[0].clone())?;
    let side = cx.expand(a[1].clone())?.parse().unwrap_or(0);
    set(cx, &which, side, a[2].clone());
    Ok(())
}

/// The nodes of the label and the annotation, if any, as path operations.
pub fn nodes(cx: &mut Context, placed: &Placed, frame: &Frame, unit: f64) -> Result<String, Error> {
    let _ = unit;
    let align = setting(cx, "label/align")?;
    let mut s = String::new();
    for (which, default_side, distance_key) in [("l", 1, "label distance"), ("a", -1, "annotation distance")] {
        let text = cx.macro_body(&format!("voltik@{which}"));
        if text.trim().is_empty() {
            continue;
        }
        let side = match cx.macro_text(&format!("voltik@{which}side")).parse::<i32>().unwrap_or(0) {
            0 => default_side,
            n => n.signum(),
        };
        // the label stays on its side when the component is mirrored or inverted
        let side = side as f64;
        let extra = match setting(cx, distance_key)? {
            d if d.is_empty() => 0.0,
            d => cx.length(Tokens::text(&d)).unwrap_or(0.0),
        };
        let text = units(&text.to_text());
        // the direction from the component to the label, on the canvas
        let outward = placed.angle + 90.0 * side;
        let rotate = match align.as_str() {
            "straight" => false,
            "rotate" => true,
            _ => {
                let m = placed.angle.rem_euclid(90.0);
                m > 20.0 && m < 70.0
            }
        };
        if rotate {
            // along the line, read left to right; `mid` sits on the line through the label's middle,
            // further out below the line than above it
            let mut turn = placed.angle;
            if turn > 90.5 || turn < -89.5 {
                turn += 180.0;
            }
            let below = (side < 0.0) != (turn != placed.angle);
            let space = if below { 2.25 } else { 1.5 } * LABEL_SPACE;
            let reach = placed.height / 2.0 * MARGIN + space + extra;
            let at = placed.mid.plus(placed.normal.scale(side * reach));
            s.push_str(&format!(" node[anchor=mid,inner sep=0pt,rotate={}] at {} {{{text}}}", round(turn), frame.coordinate(at)));
        } else {
            // the text keeps its baseline (above and below) or the middle of its x-height (at the
            // sides) at a fixed distance from the component's band, whatever its letters
            let border = placed.mid.plus(placed.normal.scale(side * (placed.height / 2.0 * MARGIN + extra)));
            let (anchor, shift) = match outward.rem_euclid(360.0) {
                a if (45.0..135.0).contains(&a) => ("base", Point::new(0.0, ABOVE)),
                a if (225.0..315.0).contains(&a) => ("base", Point::new(0.0, -BELOW)),
                a if (135.0..225.0).contains(&a) => ("mid east", Point::new(-SIDE, 0.0)),
                _ => ("mid west", Point::new(SIDE, 0.0)),
            };
            let at = border.plus(shift);
            s.push_str(&format!(" node[anchor={anchor},inner sep=0pt] at {} {{{text}}}", frame.coordinate(at)));
        }
    }
    Ok(s)
}

fn round(x: f64) -> String {
    format!("{}", x.round() as i64)
}

/// `value<unit>`, the siunitx shorthand of the manual (`R=20<\ohm>`, `C=1<\micro\farad>`): the value
/// and the unit's symbols as math, `$20\,\mathrm{\Omega}$`. Other text is left as it is.
pub fn units(text: &str) -> String {
    let t = text.trim();
    let Some(open) = t.rfind('<') else { return text.to_string() };
    if !t.ends_with('>') {
        return text.to_string();
    }
    let value = t[..open].trim();
    let unit = &t[open + 1..t.len() - 1];
    let mut symbols = String::new();
    let mut rest = unit.trim();
    while let Some(r) = rest.strip_prefix('\\') {
        let end = r.find(|c: char| !c.is_ascii_alphabetic()).unwrap_or(r.len());
        let name = &r[..end];
        symbols.push_str(match name {
            "ohm" => "\\Omega",
            "farad" => "F",
            "henry" => "H",
            "volt" => "V",
            "ampere" => "A",
            "siemens" => "S",
            "watt" => "W",
            "hertz" => "Hz",
            "second" => "s",
            "coulomb" => "C",
            "kilo" => "k",
            "mega" => "M",
            "giga" => "G",
            "milli" => "m",
            "micro" => "\\mu ",
            "nano" => "n",
            "pico" => "p",
            "degree" => "{}^\\circ",
            _ => name,
        });
        rest = r[end..].trim_start();
    }
    if !rest.is_empty() {
        symbols.push_str(rest);
    }
    // a value already in math keeps its own dollars
    let value = value.trim_matches('$');
    if value.is_empty() {
        format!("$\\mathrm{{{symbols}}}$")
    } else {
        format!("${value}\\,\\mathrm{{{symbols}}}$")
    }
}

#[cfg(test)]
mod tests {
    use super::units;

    #[test]
    fn siunitx_shorthand() {
        assert_eq!(units(r"20<\ohm>"), r"$20\,\mathrm{\Omega}$");
        assert_eq!(units(r"1<\micro\farad>"), r"$1\,\mathrm{\mu F}$");
        assert_eq!(units(r"$R_1$"), r"$R_1$");
        assert_eq!(units("a < b"), "a < b");
    }
}
