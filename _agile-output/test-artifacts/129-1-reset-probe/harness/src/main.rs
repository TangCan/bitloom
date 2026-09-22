use std::{fs, path::PathBuf};
fn main() {
    let root = PathBuf::from(std::env::args().nth(1).expect("output directory"));
    for (name, typed) in [("typed", true), ("adapted", false)] {
        let hir = reset_probe_design::graph(typed);
        let dir = root.join(name);
        fs::create_dir_all(dir.join("src/main/scala")).unwrap();
        fs::write(
            dir.join("direct.v"),
            bitloom_vlog::emit(&hir)
                .files
                .iter()
                .map(|f| f.contents.as_str())
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap();
        fs::write(
            dir.join("design.fir"),
            &bitloom_firrtl::emit(&hir).files[0].contents,
        )
        .unwrap();
        fs::write(
            dir.join("src/main/scala/Design.scala"),
            &bitloom_firrtl::emit_chisel(&hir).unwrap().files[0].contents,
        )
        .unwrap();
    }
}
