# Experimentos com discos de VM

Fixtures locais reproduzem a sa?da de `VBoxManage showmediuminfo` para verificar o fluxo de discos VDI din?micos, bloqueados, ligados a snapshots e de tamanho fixo. Eles n?o cont?m discos virtuais nem alteram uma VM.

Execute os cen?rios com `cargo test --manifest-path ..\src-tauri\Cargo.toml vm_disk_tests`.
