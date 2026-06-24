import shutil
from pathlib import Path
from invoke.context import Context
from invoke.tasks import task


@task
def build(ctx: Context):
    ctx.run("cargo build --release --color always")
    shutil.copy2(Path("target/x86_64-kernel/release/kernel"), Path("esp/kernel"))


@task
def run(ctx: Context):
    ctx.run("qemu-system-x86_64.exe -readconfig qemu.cfg")
