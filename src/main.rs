use rand::prelude::*;
use rand::{self, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

struct Childrens {
    child_of: Option<u32>,
    name: String,
    arguments: String,
}

fn main() {
    let mut rng = rand::thread_rng();

    for svg_index in 0..1 {
        let mut code = Vec::new();

        code.push(Childrens {
            child_of: None,
            name: "svg".to_string(),
            arguments: "width=\"100\" height=\"100\"".to_string(),
        });

        all_code.push("<svg >".to_string());

        for _i in 0..rng.gen_range(Range { start: 1, end: 5 }) {
            let mut start_string = format!("<{} ", types[types_index]);

            for _j in 1..rng.gen_range(Range { start: 1, end: 20 }) {
                let argument_string = format!(
                    "{}=\"{}\" ",
                    arguments.choose(&mut rand::thread_rng()).unwrap(),
                    get_random_argument()
                );
                start_string.push_str(argument_string.as_str());
            }
            start_string.push('>');
            all_code.push(start_string);
            all_code.push(format!("</{}>", types[types_index]));
        }

        all_code.push("</svg>".to_string());

        for i in &all_code {
            println!("{}", i);
        }
        let mut file =
            File::create(format!("/home/rafal/Desktop/SV/file{}.svg", svg_index)).unwrap();
        for line in all_code {
            writeln!(file, "{}", line);
        }
    }
}

fn get_random_argument() -> String {
    let number = rand::thread_rng().gen_range(0..7);

    if number == 0 {
        // Normal number
        let num = rand::thread_rng().gen_range(-1000..1000);
        return num.to_string();
    } else if number == 1 {
        // Number with em or px
        let choosed_end = ["px", "em"].choose(&mut rand::thread_rng()).unwrap();

        let num = rand::thread_rng().gen_range(-1000..1000);
        return format!("{}{}", num.to_string(), choosed_end);
    } else if number == 2 {
        // Color
        let mut color_string = "#".to_string();
        for _i in 0..6 {
            if rand::thread_rng().gen_bool(0.5) {
                color_string.push_str(rand::thread_rng().gen_range(0..10).to_string().as_str());
            } else {
                color_string.push(
                    *['a', 'b', 'c', 'd', 'e', 'f']
                        .choose(&mut rand::thread_rng())
                        .unwrap(),
                );
            }
        }
        return "".to_string();
    } else if number == 3 {
        // Empty
        return "".to_string();
    } else if number == 4 {
        // Percent
        let num = rand::thread_rng().gen_range(-300..300);
        return format!("{}%", num);
    } else if number == 5 {
        // 2/4 numbers
        let mut numbers: [i32; 4] = [0; 4];
        for i in 0..numbers.len() {
            numbers[i] = rand::thread_rng().gen_range(-300..300);
        }
        return "".to_string();
    } else if number == 6 {
        // Real number
        let number: f32 = rand::thread_rng().gen_range(-100f32..100f32);
        return number.to_string();
    }
    unreachable!();
}

const strange_values: [&str; 16] = [
    "auto",
    "SourceGraphic",
    "SourceAlpha",
    "BackgroundImage",
    "BackgroundAlpha",
    "FillPaint",
    "StrokePaint",
    "pad",
    "reflect",
    "repeat",
    "strokeWidth",
    "userSpaceOnUse",
    "objectBoundingBox",
    "min",
    "mid",
    "max",
];

const operators: [&str; 77] = [
    "Element",
    "a",
    "altGlyph",
    "altGlyphDef",
    "altGlyphItem",
    "animate",
    "animateMotion",
    "animateTransform",
    "circle",
    "clipPath",
    "color-profile",
    "cursor",
    "defs",
    "desc",
    "ellipse",
    "feBlend",
    "feColorMatrix",
    "feComponentTransfer",
    "feComposite",
    "feConvolveMatrix",
    "feDiffuseLighting",
    "feDisplacementMap",
    "feDistantLight",
    "feFlood",
    "feFuncA",
    "feFuncB",
    "feFuncG",
    "feFuncR",
    "feGaussianBlur",
    "feImage",
    "feMerge",
    "feMergeNode",
    "feMorphology",
    "feOffset",
    "fePointLight",
    "feSpecularLighting",
    "feSpotLight",
    "feTile",
    "feTurbulence",
    "filter",
    "font",
    "font-face",
    "font-face-format",
    "font-face-name",
    "font-face-src",
    "font-face-uri",
    "foreignObject",
    "g",
    "glyph",
    "glyphRef",
    "hkern",
    "image",
    "line",
    "linearGradient",
    "marker",
    "mask",
    "metadata",
    "missing-glyph",
    "mpath",
    "path",
    "pattern",
    "polygon",
    "polyline",
    "radialGradient",
    "rect",
    "script",
    "set",
    "stop",
    "style",
    "switch",
    "symbol",
    "text",
    "textPath",
    "title",
    "tref",
    "tspan",
    "use",
];
const arguments: [&str; 84] = [
    "Attributes",
    "URI",
    "a",
    "as",
    "attributeName",
    "by",
    "calcMode",
    "clip-path",
    "clipPathUnits",
    "cursor",
    "cx",
    "cy",
    "d",
    "dur",
    "dx",
    "dy",
    "fill",
    "fill-rule",
    "format",
    "from",
    "fx",
    "fy",
    "glyphRef",
    "gradientTransform",
    "gradientUnits",
    "height",
    "id",
    "image",
    "in",
    "in2",
    "keyPoints",
    "lengthAdjust",
    "local",
    "markerHeight",
    "markerUnits",
    "markerWidth",
    "maskContentUnits",
    "maskUnits",
    "mode",
    "name",
    "of",
    "offset",
    "opacity",
    "orient",
    "path",
    "pathLength",
    "patternContentUnits",
    "patternTransform",
    "patternUnits",
    "points",
    "preserveAspectRatio",
    "r",
    "refx",
    "refy",
    "rendering-intent",
    "repeatCount",
    "rotate",
    "rx",
    "ry",
    "spreadMethod",
    "stop-color",
    "stop-opacity",
    "target",
    "textLength",
    "the",
    "to",
    "transform",
    "type",
    "use",
    "viewBox",
    "width",
    "x",
    "x1",
    "x2",
    "xlink:actuate",
    "xlink:href",
    "xlink:show",
    "xml",
    "xml:space",
    "xmlns:xlink",
    "y",
    "y1",
    "y2",
    "zoomAndPan",
];
