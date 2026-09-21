use super::*;
fn bounded(command: &mut Command, log_path: &Path, timeout: Duration) -> io::Result<ExitStatus> {
    let log = fs::File::create(log_path)?;
    // GNU timeout creates and signals a process group, including tool children.
    let mut runner = Command::new("timeout");
    runner
        .arg("--kill-after=5s")
        .arg(format!("{}s", timeout.as_secs_f64()))
        .arg(command.get_program())
        .args(command.get_args());
    if let Some(dir) = command.get_current_dir() {
        runner.current_dir(dir);
    }
    for (key, value) in command.get_envs() {
        if let Some(value) = value {
            runner.env(key, value);
        } else {
            runner.env_remove(key);
        }
    }
    let status = runner
        .stdout(Stdio::from(log.try_clone()?))
        .stderr(Stdio::from(log))
        .status()?;
    writeln!(
        fs::OpenOptions::new().append(true).open(log_path)?,
        "status={status}"
    )?;
    if matches!(status.code(), Some(124 | 137)) {
        return Err(io::Error::new(
            io::ErrorKind::TimedOut,
            format!("tool timed out; log {}", log_path.display()),
        ));
    }
    Ok(status)
}

pub(super) fn run_rtl(label: &str, hir: &FrozenHir, tb: &str) {
    let design = bitloom_vlog::emit(hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let (status, log) = run_rtl_source(label, &design, tb);
    assert!(status.success(), "FR196 {label}: vvp failed\n{log}");
    assert!(
        log.contains("FR196 PASS"),
        "simulation did not reach success marker"
    );
}

// Pure monitor sensitivity probes, not mutations of the product RTL.
pub(super) fn run_monitor_probe(label: &str, tb: &str, fatal: Option<&str>) {
    let (status, log) = run_rtl_source(label, "", tb);
    if let Some(diagnostic) = fatal {
        assert_eq!(
            status.code(),
            Some(1),
            "{label}: expected SV $fatal(1)\n{log}"
        );
        let fatals: Vec<_> = log
            .lines()
            .filter(|line| line.starts_with("FATAL: "))
            .collect();
        assert_eq!(
            fatals.len(),
            1,
            "{label}: expected exactly one fatal\n{log}"
        );
        assert!(
            fatals[0].ends_with(&format!(": {diagnostic}")),
            "{label}: wrong fatal diagnostic\n{log}"
        );
        assert!(
            !log.contains("FR196 PASS"),
            "{label}: negative probe reached success"
        );
    } else {
        assert!(status.success(), "{label}: positive probe failed\n{log}");
        assert!(
            log.contains("FR196 PASS"),
            "{label}: missing success marker"
        );
    }
}

// Compilation and instrument failures always fail the Rust test. Only a completed
// simulation is returned for callers to assert an intentional RTL fatal.
fn run_rtl_source(label: &str, design: &str, tb: &str) -> (ExitStatus, String) {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr196-axi-lite-csr")
        .join(format!("{label}-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("design.v"), design).unwrap();
    fs::write(dir.join("tb.sv"), tb).unwrap();
    let timeout = Duration::from_secs(60);
    let mut commands = String::new();
    for tool in ["iverilog", "vvp"] {
        let path = dir.join(format!("{tool}-version.log"));
        let status = bounded(Command::new(tool).arg("-V"), &path, timeout)
            .expect("FR196 RTL tool required and version probe must complete within 60s");
        assert!(
            status.success(),
            "{tool} version probe failed; {}",
            path.display()
        );
        writeln!(commands, "{tool} -V\n{}", fs::read_to_string(path).unwrap()).unwrap();
    }
    commands.push_str("iverilog -g2012 -s tb -o simulation design.v tb.sv\nvvp simulation\n");
    fs::write(dir.join("commands.log"), commands).unwrap();
    for (tool, args, filename) in [
        (
            "iverilog",
            vec![
                "-g2012",
                "-s",
                "tb",
                "-o",
                "simulation",
                "design.v",
                "tb.sv",
            ],
            "compile.log",
        ),
        ("vvp", vec!["simulation"], "run.log"),
    ] {
        let path = dir.join(filename);
        let status = bounded(
            Command::new(tool).args(args).current_dir(&dir),
            &path,
            timeout,
        )
        .expect("FR196 RTL tool must exist and finish within 60s; artifacts preserved");
        let log = fs::read_to_string(&path).unwrap();
        if tool == "vvp" {
            println!("{label}: {log}artifacts={}", dir.display());
            return (status, log);
        }
        assert!(
            status.success(),
            "FR196 {label}: compile failed; artifacts {}\n{log}",
            dir.display()
        );
    }
    unreachable!("simulation stage is mandatory")
}
