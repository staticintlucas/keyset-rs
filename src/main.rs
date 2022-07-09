use std::fs::File;
use std::io::Write;
use std::path::Path;

use keyset_rs::*;

fn main() {
    let json = r#"[
        ["¬\n`", "!\n1", "\"\n2", "£\n3", "$\n4", "%\n5", "^\n6", "&\n7", "*\n8", "(\n9", ")\n0", "_\n-", "+\n=", {"w": 2}, "Backspace"],
        [{"w": 1.5}, "Tab", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "{\n[", "}\n]", {"x": 0.25, "w": 1.25, "h": 2, "w2": 1.5, "h2": 1, "x2": -0.25}, "Enter"],
        [{"w": 1.25, "w2": 1.75, "l": true}, "Caps Lock", "A", "S", "D", "F", "G", "H", "J", "K", "L", ":\n;", "@\n'", "~\n#"],
        [{"w": 1.25}, "Shift", "|\n\\", "Z",  "X", "C", "V", "B", "N", "M", "<\n,", ">\n.", "?\n/", {"w": 2.75}, "Shift"],
        [{"w": 1.5}, "Ctrl", "Win", {"w": 1.5}, "Alt", {"a": 7, "w": 7}, "", {"a": 4, "w": 1.5}, "AltGr", "Win", {"w": 1.5}, "Ctrl"]
    ]"#;

    let layout = Layout::from_kle(json).unwrap();

    let mut file = File::create(Path::new("test.svg")).unwrap();

    file.write_all(layout.to_svg().as_bytes()).unwrap();
}
