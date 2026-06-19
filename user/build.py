import os
from pathlib import Path

apps = sorted(path.stem for path in Path('src/bin').glob('*.rs'))
ret = os.system('cargo build --release')
if ret != 0:
    raise SystemExit(ret)

for app in apps:
    print('[build.py] application %s linked at virtual address 0x0' % app)
