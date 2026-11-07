use serde::Serialize;
use std::{collections::{HashMap, HashSet}, fs, path::{Path, PathBuf}, process::Command, sync::Mutex, time::{Duration, SystemTime}};
use tauri::State;
use uuid::Uuid;

const MAX_FILES: usize = 100_000;
const MIN_AGE: Duration = Duration::from_secs(24 * 60 * 60);
const MAX_VM_DISKS: usize = 500;
const MAX_VM_DEPTH: usize = 8;

#[derive(Clone)]
struct Candidate { path: PathBuf, size: u64, modified: SystemTime }
#[derive(Default)]
struct ScanState(Mutex<HashMap<String, Candidate>>);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Item { id: String, name: String, size: u64, age_days: u64 }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ScanResult { folder: String, total_bytes: u64, items: Vec<Item>, truncated: bool, skipped: usize }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct VmDisk { path: String, size_bytes: u64, capacity_bytes: u64, compactable: bool, status: String }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct VmDiskScan { folder: String, disks: Vec<VmDisk>, skipped: usize }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct VmCompactResult { path: String, output: String }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CleanResult { deleted: usize, freed_bytes: u64, failed: usize }

fn virtualbox_manage() -> Result<PathBuf, String> {
    let program_files = std::env::var_os("ProgramFiles").ok_or("VirtualBox n?o encontrado")?;
    let path = PathBuf::from(program_files).join("Oracle").join("VirtualBox").join("VBoxManage.exe");
    if path.is_file() { Ok(path) } else { Err("VirtualBox n?o encontrado em Program Files\\Oracle\\VirtualBox".into()) }
}

fn medium_info(executable: &Path, disk: &Path) -> Result<String, String> {
    let output = Command::new(executable).args(["showmediuminfo", "disk"]).arg(disk).output().map_err(|e| format!("Falha ao consultar o disco: {e}"))?;
    if !output.status.success() { return Err(String::from_utf8_lossy(&output.stderr).trim().to_string()); }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn field<'a>(info: &'a str, name: &str) -> Option<&'a str> {
    info.lines().find_map(|line| line.split_once(':').filter(|(key, _)| key.trim().eq_ignore_ascii_case(name)).map(|(_, value)| value.trim()))
}

fn compactability(info: &str) -> Result<(u64, u64), String> {
    let state = field(info, "State").ok_or("Estado do disco indispon?vel")?;
    let kind = field(info, "Type").ok_or("Tipo do disco indispon?vel")?;
    let variant = field(info, "Format variant").ok_or("Formato do disco indispon?vel")?;
    if !state.eq_ignore_ascii_case("created") { return Err("Disco em uso ou bloqueado; desligue a m?quina virtual".into()); }
    if !kind.eq_ignore_ascii_case("normal (base)") { return Err("Discos com snapshots ou encadeados n?o s?o compactados por seguran?a".into()); }
    if !variant.to_ascii_lowercase().contains("dynamic") { return Err("Somente discos VDI din\u{e2}micos podem ser compactados".into()); }
    let capacity = field(info, "Capacity").and_then(|v| v.split_whitespace().next()).and_then(|v| v.parse::<u64>().ok()).ok_or("Capacidade do disco indispon?vel")?;
    let allocated = field(info, "Size on disk").and_then(|v| v.split_whitespace().next()).and_then(|v| v.parse::<u64>().ok()).ok_or("Tamanho ocupado indispon?vel")?;
    Ok((allocated.saturating_mul(1024 * 1024), capacity.saturating_mul(1024 * 1024)))
}

#[tauri::command]
fn scan_vm_disks(folder: String) -> Result<VmDiskScan, String> {
    let requested = if folder.trim().is_empty() { PathBuf::from(std::env::var_os("USERPROFILE").ok_or("Perfil do usuario indisponivel")?).join("VirtualBox VMs") } else { PathBuf::from(folder) };
    let root = fs::canonicalize(requested).map_err(|e| format!("Pasta indispon?vel: {e}"))?;
    if !root.is_dir() { return Err("Selecione uma pasta v?lida".into()); }
    let executable = virtualbox_manage()?;
    let mut stack = vec![(root.clone(), 0usize)];
    let mut disks = Vec::new();
    let mut skipped = 0;
    while let Some((dir, depth)) = stack.pop() {
        let entries = match fs::read_dir(dir) { Ok(v) => v, Err(_) => { skipped += 1; continue; } };
        for entry in entries {
            if disks.len() >= MAX_VM_DISKS { break; }
            let entry = match entry { Ok(v) => v, Err(_) => { skipped += 1; continue; } };
            let path = entry.path();
            let meta = match fs::symlink_metadata(&path) { Ok(v) => v, Err(_) => { skipped += 1; continue; } };
            if meta.file_type().is_symlink() { skipped += 1; continue; }
            if meta.is_dir() && depth < MAX_VM_DEPTH { stack.push((path, depth + 1)); continue; }
            if !meta.is_file() || !path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("vdi")) { continue; }
            match medium_info(&executable, &path).and_then(|info| compactability(&info).map(|(size, capacity)| (info, size, capacity))) {
                Ok((_, size_bytes, capacity_bytes)) => disks.push(VmDisk { path: path.display().to_string(), size_bytes, capacity_bytes, compactable: true, status: "VDI din\u{e2}mico pronto para compacta\u{e7}\u{e3}o".into() }),
                Err(error) => disks.push(VmDisk { path: path.display().to_string(), size_bytes: meta.len(), capacity_bytes: 0, compactable: false, status: error }),
            }
        }
    }
    disks.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(VmDiskScan { folder: root.display().to_string(), disks, skipped })
}

#[tauri::command]
fn compact_vm_disk(path: String) -> Result<VmCompactResult, String> {
    let disk = fs::canonicalize(path).map_err(|e| format!("Disco indispon?vel: {e}"))?;
    if !disk.is_file() || !disk.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("vdi")) { return Err("Selecione um arquivo VDI".into()); }
    let executable = virtualbox_manage()?;
    let info = medium_info(&executable, &disk)?;
    compactability(&info)?;
    let output = Command::new(executable).args(["modifymedium", "disk"]).arg(&disk).arg("--compact").output().map_err(|e| format!("Falha ao compactar o disco: {e}"))?;
    if !output.status.success() { return Err(String::from_utf8_lossy(&output.stderr).trim().to_string()); }
    Ok(VmCompactResult { path: disk.display().to_string(), output: String::from_utf8_lossy(&output.stdout).trim().to_string() })
}

fn temp_root() -> Result<PathBuf, String> {
    let root = std::env::temp_dir();
    let canonical = fs::canonicalize(&root).map_err(|e| format!("Pasta TEMP inacessível: {e}"))?;
    if !canonical.is_dir() { return Err("TEMP não é uma pasta".into()); }
    Ok(canonical)
}

#[tauri::command]
fn scan_temp(state: State<'_, ScanState>) -> Result<ScanResult, String> {
    let root = temp_root()?;
    let now = SystemTime::now();
    let mut stack = vec![root.clone()];
    let mut visited = HashSet::new();
    let mut items = Vec::new();
    let mut candidates = HashMap::new();
    let mut total_bytes: u64 = 0;
    let mut skipped = 0;
    let mut truncated = false;
    while let Some(dir) = stack.pop() {
        if !visited.insert(dir.clone()) { continue; }
        let entries = match fs::read_dir(&dir) { Ok(v) => v, Err(_) => { skipped += 1; continue; } };
        for entry in entries {
            if items.len() >= MAX_FILES { truncated = true; break; }
            let entry = match entry { Ok(v) => v, Err(_) => { skipped += 1; continue; } };
            let path = entry.path();
            let meta = match fs::symlink_metadata(&path) { Ok(v) => v, Err(_) => { skipped += 1; continue; } };
            if meta.file_type().is_symlink() { skipped += 1; continue; }
            if meta.is_dir() {
                if let Ok(p) = fs::canonicalize(&path) { if p.starts_with(&root) { stack.push(p); } }
                continue;
            }
            if !meta.is_file() { skipped += 1; continue; }
            let modified = match meta.modified() { Ok(v) => v, Err(_) => { skipped += 1; continue; } };
            let age = match now.duration_since(modified) { Ok(v) if v >= MIN_AGE => v, _ => continue };
            let canonical = match fs::canonicalize(&path) { Ok(v) if v.starts_with(&root) => v, _ => { skipped += 1; continue; } };
            let id = Uuid::new_v4().to_string();
            total_bytes = total_bytes.saturating_add(meta.len());
            items.push(Item { id: id.clone(), name: canonical.strip_prefix(&root).unwrap_or(Path::new("")).display().to_string(), size: meta.len(), age_days: age.as_secs() / 86400 });
            candidates.insert(id, Candidate { path: canonical, size: meta.len(), modified });
        }
        if truncated { break; }
    }
    items.sort_by(|a, b| b.size.cmp(&a.size));
    *state.0.lock().map_err(|_| "Estado indisponível")? = candidates;
    Ok(ScanResult { folder: root.display().to_string(), total_bytes, items, truncated, skipped })
}

#[tauri::command]
fn clean_selected(ids: Vec<String>, state: State<'_, ScanState>) -> Result<CleanResult, String> {
    let root = temp_root()?;
    let mut cache = state.0.lock().map_err(|_| "Estado indisponível")?;
    let mut deleted = 0;
    let mut failed = 0;
    let mut freed_bytes: u64 = 0;
    let unique: HashSet<_> = ids.into_iter().collect();
    for id in unique {
        // Ids only come from the most recent backend scan, never from arbitrary frontend paths.
        let Some(c) = cache.remove(&id) else { failed += 1; continue; };
        if !c.path.starts_with(&root) { failed += 1; continue; }
        let meta = match fs::symlink_metadata(&c.path) { Ok(v) if v.is_file() && !v.file_type().is_symlink() => v, _ => { failed += 1; continue; } };
        if meta.len() != c.size || meta.modified().ok() != Some(c.modified) { failed += 1; continue; }
        match fs::canonicalize(&c.path) { Ok(p) if p == c.path && p.starts_with(&root) => (), _ => { failed += 1; continue; } }
        match fs::remove_file(&c.path) { Ok(()) => { deleted += 1; freed_bytes = freed_bytes.saturating_add(c.size); }, Err(_) => failed += 1 }
    }
    Ok(CleanResult { deleted, freed_bytes, failed })
}

pub fn run() {
    tauri::Builder::default()
        .manage(ScanState::default())
        .invoke_handler(tauri::generate_handler![scan_temp, clean_selected, scan_vm_disks, compact_vm_disk])
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar WinClean");
}


#[cfg(test)]
mod vm_disk_tests {
    use super::compactability;

    #[test]
    fn accepts_unlocked_dynamic_base_disk() {
        let info = include_str!("../../experiments/vdi-dynamic-base.txt");
        assert_eq!(compactability(info), Ok((61_257 * 1_048_576, 82_015 * 1_048_576)));
    }

    #[test]
    fn rejects_locked_disk() {
        let info = include_str!("../../experiments/vdi-locked.txt");
        assert!(compactability(info).unwrap_err().contains("bloqueado"));
    }

    #[test]
    fn rejects_snapshot_disk() {
        let info = include_str!("../../experiments/vdi-snapshot.txt");
        assert!(compactability(info).unwrap_err().contains("snapshots"));
    }

    #[test]
    fn rejects_fixed_disk() {
        let info = include_str!("../../experiments/vdi-fixed.txt");
        assert!(compactability(info).unwrap_err().contains("VDI din"));
    }
}
