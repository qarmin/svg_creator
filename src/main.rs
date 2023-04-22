use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::Range;
use std::path::Path;
use std::process::{Command, Stdio};

use config::Config;
use rand::prelude::*;
use rand::{self, Rng};

struct Childrens {
    childrens: Vec<usize>,
    name: String,
    arguments: String,
}

struct Settings {
    svg_tool_path: String,
    temp_files_save_location: String,
    found_broken_files_save_location: String,
    image_size: u32,
    number_generated_svg: u32,
    max_number_of_operators: u32,
    max_number_of_arguments: u32,
    generate_invalid_items: bool,
    timeout_seconds: String,
}

fn load_settings() -> Settings {
    let settings = Config::builder().add_source(config::File::with_name("settings")).build().unwrap();
    let config = settings.try_deserialize::<HashMap<String, HashMap<String, String>>>().unwrap();

    let general_settings = config["general"].clone();
    Settings {
        svg_tool_path: general_settings["svg_tool_path"].clone(),
        temp_files_save_location: general_settings["temp_files_save_location"].clone(),
        found_broken_files_save_location: general_settings["found_broken_files_save_location"].clone(),
        image_size: general_settings["image_size"].parse().unwrap(),
        number_generated_svg: general_settings["number_generated_svg"].parse().unwrap(),
        max_number_of_operators: general_settings["max_number_of_operators"].parse().unwrap(),
        max_number_of_arguments: general_settings["max_number_of_arguments"].parse().unwrap(),
        generate_invalid_items: general_settings["generate_invalid_items"].parse().unwrap(),
        timeout_seconds: general_settings["timeout_seconds"].clone(),
    }
}

fn main() {
    let settings = load_settings();
    let mut rng = thread_rng();

    for svg_index in 0..settings.number_generated_svg {
        if svg_index % 100 == 0 {
            println!("-- {}/{}", svg_index, settings.number_generated_svg);
        }
        let mut code = Vec::new();
        {
            {
                code.push(Childrens {
                    childrens: vec![],
                    name: "svg".to_string(),
                    arguments: "width=\"1\" height=\"1\"".to_string(),
                });
            }
            for _i in 0..rng.gen_range(Range {
                start: 0,
                end: settings.max_number_of_operators,
            }) {
                let parent_index = thread_rng().gen_range(0..code.len());

                let mut values = "".to_string();

                let mut args = Vec::new();
                for _j in 0..rng.gen_range(Range {
                    start: 0,
                    end: settings.max_number_of_arguments,
                }) {
                    let arg = ARGUMENTS.choose(&mut thread_rng()).unwrap();
                    // ThorVG leaks memory when two same arguments are used e.g. <image p=1 p=2>
                    if !args.contains(arg) {
                        args.push(arg);
                        values.push_str(format!("{}=\"{}\" ", arg, get_random_argument()).as_str());
                    }
                }
                let latest_index = code.len();
                code[parent_index].childrens.push(latest_index);
                code.push(Childrens {
                    childrens: vec![],
                    name: OPERATORS.choose(&mut thread_rng()).unwrap().to_string(),
                    arguments: values,
                });
            }
        }

        let file_name = format!("{}/{}.svg", settings.temp_files_save_location, svg_index);

        let mut file = File::create(&file_name).unwrap();
        return_children_text(&code, &code[0], &mut file, &settings);

        let out = Command::new("timeout")
            .arg("-v")
            .arg(&settings.timeout_seconds)
            .arg(&settings.svg_tool_path)
            .arg(&file_name)
            .arg("-r")
            .arg(&format!("{}x{}", settings.image_size, settings.image_size))
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();

        let err = String::from_utf8(out.stderr).unwrap();
        let out = String::from_utf8(out.stdout).unwrap();

        let mut is_broken_file = false;
        const TIMEOUT_MSG: &str = "timeout: sending signal";
        const RUNTIME_MSG: &str = "runtime error";
        const SANITIZER_MSG: &str = "Sanitizer";
        if err.contains(TIMEOUT_MSG) || out.contains(TIMEOUT_MSG) {
            println!("FOUND TIMEOUT");
            is_broken_file = true;
        } else if err.contains(RUNTIME_MSG) || out.contains(RUNTIME_MSG) {
            println!("FOUND RUNTIME ERROR");
            is_broken_file = true;
        } else if err.contains(SANITIZER_MSG) || out.contains(SANITIZER_MSG) {
            println!("FOUND SANITIZER");
            is_broken_file = true;
        } else {
            println!("NOT FOUND ANYTHING:\nOUT: {out}\nERR: {err}");
        }

        if is_broken_file {
            let old_fn = Path::new(&file_name).file_name().unwrap().to_str().unwrap().to_string();
            let full_name = format!("{}/{old_fn}", settings.found_broken_files_save_location);
            fs::copy(&file_name, &full_name).unwrap();
            println!("Found broken file {file_name}\nOUT: {out}\nERR: {err}");
        }

        // print!("echo \"{}\"; ", file_name);
        // print!("timeout 10 ");
        // println!("{} {} -r {}x{}", svg_tool, file_name, IMAGE_SIZE, IMAGE_SIZE);
    }
}

fn return_children_text(code: &[Childrens], child: &Childrens, file: &mut File, settings: &Settings) {
    // println!("<{} {}>", child.name, child.ARGUMENTS);
    if settings.generate_invalid_items
        && (thread_rng().gen_range(0..(settings.max_number_of_operators * settings.max_number_of_arguments / 1000)) == 0)
    {
        let mut rar = format!("<{} {}>", child.name, child.arguments);
        if thread_rng().gen_bool(0.25) {
            rar = rar.replace('<', "");
        } else if thread_rng().gen_bool(0.3) {
            rar = rar.replace('>', "");
        } else if thread_rng().gen_bool(0.5) {
            rar = rar.replace('=', " ");
        } else {
            rar = rar.replace('>', "");
            rar = rar.replace('<', "");
        }
        writeln!(file, "{}", rar).unwrap();
    } else {
        writeln!(file, "<{} {}>", child.name, child.arguments).unwrap();
    }

    for grant_child in &child.childrens {
        return_children_text(code, &code[*grant_child], file, settings);
    }

    // println!("</{}>", child.name);
    writeln!(file, "</{}>", child.name).unwrap();
}

fn get_random_argument() -> String {
    let number = thread_rng().gen_range(0..9);

    if number == 0 {
        // Normal number
        let num = thread_rng().gen_range(-1000..1000);
        return num.to_string();
    } else if number == 1 {
        // Number with em or px
        let choosed_end = ["px", "em"].choose(&mut thread_rng()).unwrap();

        let num = thread_rng().gen_range(-1000..1000);
        return format!("{}{}", num, choosed_end);
    } else if number == 2 {
        // Color
        let mut color_string = "#".to_string();
        for _i in 0..6 {
            if thread_rng().gen_bool(0.5) {
                color_string.push_str(thread_rng().gen_range(0..10).to_string().as_str());
            } else if thread_rng().gen_bool(0.99) {
                color_string.push(*['a', 'b', 'c', 'd', 'e', 'f'].choose(&mut thread_rng()).unwrap());
            } else if thread_rng().gen_bool(0.99) {
                color_string.push(thread_rng().gen_range::<u8, Range<u8>>(0..255) as char);
            }
        }
        return "".to_string();
    } else if number == 3 {
        // Empty
        return "".to_string();
    } else if number == 4 {
        // Percent
        let num = thread_rng().gen_range(-300..300);
        return format!("{}%", num);
    } else if number == 5 {
        // 2/4 numbers
        let mut numbers: [i32; 4] = [0; 4];
        #[allow(clippy::needless_range_loop)]
        for i in 0..numbers.len() {
            numbers[i] = thread_rng().gen_range(-300..300);
        }
        return "".to_string();
    } else if number == 6 {
        // Real number
        let number: f32 = thread_rng().gen_range(-100f32..100f32);
        return number.to_string();
    } else if number == 7 {
        // Strange Values
        return STRANGE_VALUES.choose(&mut thread_rng()).unwrap().to_string();
    } else if number == 8 {
        // n points

        let mut text = "".to_string();
        for _i in 1..thread_rng().gen_range(1..20) {
            text.push_str(format!("{},{} ", thread_rng().gen_range(1..200), thread_rng().gen_range(1..200)).as_str());
        }

        return text;
    }
    unreachable!();
}

const STRANGE_VALUES: [&str; 16] = [
    "auto", "SourceGraphic", "SourceAlpha", "BackgroundImage", "BackgroundAlpha", "FillPaint", "StrokePaint", "pad", "reflect", "repeat",
    "strokeWidth", "userSpaceOnUse", "objectBoundingBox", "min", "mid", "max",
];

const OPERATORS: [&str; 80] = [
    "a", "altGlyph", "altGlyphDef", "altGlyphItem", "animate", "animateColor", "animateMotion", "animateTransform", "circle", "clipPath",
    "color-profile", "cursor", "defs", "desc", "ellipse", "feBlend", "feColorMatrix", "feComponentTransfer", "feComposite", "feConvolveMatrix",
    "feDiffuseLighting", "feDisplacementMap", "feDistantLight", "feFlood", "feFuncA", "feFuncB", "feFuncG", "feFuncR", "feGaussianBlur", "feImage",
    "feMerge", "feMergeNode", "feMorphology", "feOffset", "fePointLight", "feSpecularLighting", "feSpotLight", "feTile", "feTurbulence", "filter",
    "font", "font-face", "font-face-format", "font-face-name", "font-face-src", "font-face-uri", "foreignObject", "g", "glyph", "glyphRef", "hkern",
    "image", "line", "linearGradient", "marker", "mask", "metadata", "missing-glyph", "mpath", "path", "pattern", "polygon", "polyline",
    "radialGradient", "rect", "script", "set", "stop", "style", "svg", "switch", "symbol", "text", "textPath", "title", "tref", "tspan", "use",
    "view", "vkern",
];
const ARGUMENTS: [&str; 257] = [
    "accent-height", "accumulate", "additive", "alignment-baseline", "alphabetic", "amplitude", "arabic-form", "ascent", "attributeName",
    "attributeType", "azimuth", "baseFrequency", "baseProfile", "baseline-shift", "bbox", "begin", "bias", "by", "calcMode", "cap-height", "class",
    "clip", "clip-path", "clip-rule", "clipPathUnits", "color", "color-interpolation", "color-interpolation-filters", "color-profile",
    "color-rendering", "contentScriptType", "contentStyleType", "crossorigin", "cursor", "cx", "cy", "d", "decelerate", "descent", "diffuseConstant",
    "direction", "display", "divisor", "dominant-baseline", "dur", "dx", "dy", "edgeMode", "elevation", "enable-background", "end", "exp", "fill",
    "fill-opacity", "fill-rule", "filter", "filterRes", "filterUnits", "flood-color", "flood-opacity", "font-family", "font-size",
    "font-size-adjust", "font-stretch", "font-style", "font-variant", "font-weight", "format", "fr", "from", "fx", "fy", "g1", "g2", "glyph-name",
    "glyph-orientation-horizontal", "glyph-orientation-vertical", "glyphRef", "gradientTransform", "gradientUnits", "hanging", "height",
    "horiz-adv-x", "horiz-origin-x", "href", "hreflang", "id", "ideographic", "image-rendering", "in", "in2", "intercept", "k", "k1", "k2", "k3",
    "k4", "kernelMatrix", "kernelUnitLength", "kerning", "keyPoints", "keySplines", "keyTimes", "lang", "lengthAdjust", "letter-spacing",
    "lighting-color", "limitingConeAngle", "local", "marker-end", "marker-mid", "marker-start", "markerHeight", "markerUnits", "markerWidth", "mask",
    "maskContentUnits", "maskUnits", "mathematical", "max", "media", "method", "min", "mode", "name", "numOctaves", "offset", "opacity", "operator",
    "order", "orient", "orientation", "origin", "overflow", "overline-position", "overline-thickness", "paint-order", "panose-1", "path",
    "pathLength", "patternContentUnits", "patternTransform", "patternUnits", "ping", "pointer-events", "points", "pointsAtX", "pointsAtY",
    "pointsAtZ", "preserveAlpha", "preserveAspectRatio", "primitiveUnits", "r", "radius", "refX", "refY", "referrerPolicy", "rel",
    "rendering-intent", "repeatCount", "repeatDur", "requiredExtensions", "requiredFeatures", "restart", "result", "rotate", "rx", "ry", "scale",
    "seed", "shape-rendering", "slope", "spacing", "specularConstant", "specularExponent", "speed", "spreadMethod", "startOffset", "stdDeviation",
    "stemh", "stemv", "stitchTiles", "stop-color", "stop-opacity", "strikethrough-position", "strikethrough-thickness", "string", "stroke",
    "stroke-dasharray", "stroke-dashoffset", "stroke-linecap", "stroke-linejoin", "stroke-miterlimit", "stroke-opacity", "stroke-width", "style",
    "surfaceScale", "systemLanguage", "tabindex", "tableValues", "target", "targetX", "targetY", "text-anchor", "text-decoration", "text-rendering",
    "textLength", "to", "transform", "transform-origin", "type", "u1", "u2", "underline-position", "underline-thickness", "unicode", "unicode-bidi",
    "unicode-range", "units-per-em", "v-alphabetic", "v-hanging", "v-ideographic", "v-mathematical", "values", "vector-effect", "version",
    "vert-adv-y", "vert-origin-x", "vert-origin-y", "viewBox", "viewTarget", "visibility", "width", "widths", "word-spacing", "writing-mode", "x",
    "x-height", "x1", "x2", "xChannelSelector", "xlink:actuate", "xlink:arcrole", "xlink:href", "xlink:role", "xlink:show", "xlink:title",
    "xlink:type", "xml:base", "xml:lang", "xml:space", "y", "y1", "y2", "yChannelSelector", "z", "zoomAndPan",
];
