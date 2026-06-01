import os
import shutil
from pathlib import Path

base_address = 0x80400000
step = 0x20000
linker = 'src/linker.ld'
target = 'riscv64gc-unknown-none-elf'
mode = 'release'
objcopy = 'rust-objcopy --binary-architecture=riscv64'

linker_path = Path(linker)
target_dir = Path('target') / target / mode
save_dir = Path('.build-app-cache')
if save_dir.exists():
    shutil.rmtree(save_dir)
save_dir.mkdir(parents=True)

apps = sorted(path.stem for path in Path('src/bin').glob('*.rs'))
original_linker = linker_path.read_text()

try:
    for app_id, app in enumerate(apps):
        app_base = base_address + step * app_id
        linker_path.write_text(original_linker.replace(hex(base_address), hex(app_base)))

        os.system('cargo clean')
        ret = os.system('cargo build --bin %s --release' % app)
        if ret != 0:
            raise SystemExit(ret)

        elf = target_dir / app
        bin_file = target_dir / (app + '.bin')
        ret = os.system('%s %s --strip-all -O binary %s' % (objcopy, elf, bin_file))
        if ret != 0:
            raise SystemExit(ret)

        shutil.copy2(elf, save_dir / app)
        shutil.copy2(bin_file, save_dir / (app + '.bin'))
        print('[build.py] application %s start with address %s' % (app, hex(app_base)))
finally:
    linker_path.write_text(original_linker)

target_dir.mkdir(parents=True, exist_ok=True)
for saved in save_dir.iterdir():
    shutil.copy2(saved, target_dir / saved.name)
shutil.rmtree(save_dir)
