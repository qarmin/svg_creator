use rand::prelude::*;
use rand::{self, Rng};
use std::fs::File;
use std::io::Write;
use std::ops::Range;

struct Childrens {
    childrens: Vec<usize>,
    name: String,
    arguments: String,
}

fn main() {
    let mut rng = rand::thread_rng();

    for svg_index in 0..2000 {
        let mut code = Vec::new();
        {
            {
                code.push(Childrens {
                    childrens: vec![],
                    name: "svg".to_string(),
                    arguments: "width=\"100\" height=\"100\"".to_string(),
                });
            }
            for _i in 0..rng.gen_range(Range { start: 0, end: 50 }) {
                let parent_index = rand::thread_rng().gen_range(0..code.len());

                let mut values = "".to_string();

                for _j in 0..rng.gen_range(Range { start: 0, end: 20 }) {
                    values.push_str(format!("{}=\"{}\" ", ARGUMENTS.choose(&mut rand::thread_rng()).unwrap(), get_random_argument()).as_str());
                }
                let latest_index = code.len();
                code[parent_index].childrens.push(latest_index);
                code.push(Childrens {
                    childrens: vec![],
                    name: OPERATORS.choose(&mut rand::thread_rng()).unwrap().to_string(),
                    arguments: values,
                });
            }
        }

        let file_name = format!("/home/rafal/Desktop/SV/file{}.svg", svg_index);

        let mut file = File::create(&file_name).unwrap();
        return_children_text(&code, &code[0], &mut file);

        print!("echo \"{}\"; ", file_name);
        print!("timeout 2 ");
        println!("/home/rafal/thorvg/build/src/bin/svg2png/svg2png {}", file_name);
    }
}
fn return_children_text(code: &[Childrens], child: &Childrens, file: &mut File) {
    // println!("<{} {}>", child.name, child.ARGUMENTS);
    writeln!(file, "<{} {}>", child.name, child.arguments).unwrap();

    for grant_child in &child.childrens {
        return_children_text(code, &code[*grant_child], file);
    }

    // println!("</{}>", child.name);
    writeln!(file, "</{}>", child.name).unwrap();
}

fn get_random_argument() -> String {
    let number = rand::thread_rng().gen_range(0..9);

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
                color_string.push(*['a', 'b', 'c', 'd', 'e', 'f'].choose(&mut rand::thread_rng()).unwrap());
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
        #[allow(clippy::needless_range_loop)]
        for i in 0..numbers.len() {
            numbers[i] = rand::thread_rng().gen_range(-300..300);
        }
        return "".to_string();
    } else if number == 6 {
        // Real number
        let number: f32 = rand::thread_rng().gen_range(-100f32..100f32);
        return number.to_string();
    } else if number == 7 {
        // Strange Values
        return STRANGE_VALUES.choose(&mut rand::thread_rng()).unwrap().to_string();
    } else if number == 8 {
        // n points

        let mut text = "".to_string();
        for _i in 1..rand::thread_rng().gen_range(1..20) {
            text.push_str(format!("{},{} ", rand::thread_rng().gen_range(1..200), rand::thread_rng().gen_range(1..200)).as_str());
        }

        return text;
    }
    unreachable!();
}

const STRANGE_VALUES: [&str; 16] = [
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

const OPERATORS: [&str; 77] = [
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
const ARGUMENTS: [&str; 84] = [
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
