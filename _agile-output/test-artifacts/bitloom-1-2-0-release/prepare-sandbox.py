from pathlib import Path
import os,tarfile,json,hashlib,tempfile
root=Path.cwd(); sandbox=Path(tempfile.mkdtemp(prefix='bitloom-1.2.0-candidate-'))
names=['bitloom-macro','bitloom-hir','bitloom-builder','bitloom-vlog','bitloom-sim','bitloom-prelude','bitloom-firrtl','bitloom-viz','bitloom','bitloom-lsp']
home=sandbox/'cargo-home';home.mkdir(); sources=sandbox/'sources';sources.mkdir()
original=Path(os.environ.get('CARGO_HOME',Path.home()/'.cargo'))
(home/'registry').symlink_to(original/'registry',target_is_directory=True)
if (original/'git').exists():(home/'git').symlink_to(original/'git',target_is_directory=True)
lines=['[net]','offline = true','','[patch.crates-io]']; hashes={}
for name in names:
 archive=root/'target/package'/f'{name}-1.2.0.crate'
 with tarfile.open(archive) as t:t.extractall(sources,filter='data')
 lines.append(f'{name} = {{ path = "{sources}/{name}-1.2.0" }}')
 hashes[archive.name]=hashlib.sha256(archive.read_bytes()).hexdigest()
(home/'config.toml').write_text('\n'.join(lines)+'\n')
(root/'_agile-output/test-artifacts/bitloom-1-2-0-release/package-sha256.json').write_text(json.dumps(hashes,indent=2)+'\n')
Path('/tmp/bitloom-release-sandbox-path').write_text(str(sandbox))
print(sandbox)
