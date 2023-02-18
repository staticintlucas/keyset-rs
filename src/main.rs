use std::fs::File;
use std::io::Write;
use std::path::Path;

use keyset_rs::*;

fn main() {
    let kle = r#"[
        ["¬\n`", "!\n1", "\"\n2", "£\n3", "$\n4", "%\n5", "^\n6", "&\n7", "*\n8", "(\n9", ")\n0", "_\n-", "+\n=", {"w": 2}, "Backspace"],
        [{"w": 1.5}, "Tab", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "{\n[", "}\n]", {"x": 0.25, "w": 1.25, "h": 2, "w2": 1.5, "h2": 1, "x2": -0.25}, "Enter"],
        [{"w": 1.25, "w2": 1.75, "l": true}, "Caps Lock", "A", "S", "D", "F", "G", "H", "J", "K", "L", ":\n;", "@\n'", "~\n#"],
        [{"w": 1.25}, "Shift", "|\n\\", "Z",  "X", "C", "V", "B", "N", "M", "<\n,", ">\n.", "?\n/", {"w": 2.75}, "Shift"],
        [{"w": 1.5}, "Ctrl", "Win", {"w": 1.5}, "Alt", {"a": 7, "w": 7}, "", {"a": 4, "w": 1.5}, "AltGr", "Win", {"w": 1.5}, "Ctrl"]
    ]"#;
    let profile = r#"
        type = 'cylindrical'
        depth = 0.5

        [bottom]
        width = 18.29
        height = 18.29
        radius = 0.38

        [top]
        width = 11.81
        height = 13.91
        radius = 1.52
        y-offset = -1.62

        [legend.5]
        size = 4.84
        width = 9.45
        height = 11.54
        y-offset = 0

        [legend.4]
        size = 3.18
        width = 9.53
        height = 9.56
        y-offset = 0.40

        [legend.3]
        size = 2.28
        width = 9.45
        height = 11.30
        y-offset = -0.12

        [homing]
        default = 'scoop'
        scoop = { depth = 1.5 }
        bar = { width = 3.85, height = 0.4, y-offset = 5.05 }
        bump = { diameter = 0.4, y-offset = -0.2 }
    "#;

    let layout = Layout::from_kle(kle).unwrap();
    let profile = Profile::from_toml(profile).unwrap();
    let drawing = Drawing::new(layout, profile);

    let mut file = File::create(Path::new("test.svg")).unwrap();

    file.write_all(drawing.to_svg().as_bytes()).unwrap();
}
