import React, { useState } from 'react';
import { createRoot } from 'react-dom/client';
import { invoke } from '@tauri-apps/api/core';
import { HardDrive, ScanSearch, Trash2, ShieldCheck, FolderOpen, Info, CheckCircle2, AlertTriangle, RefreshCcw, Github, Files, Settings2 } from 'lucide-react';
import './style.css';

type Item = { id: string; name: string; size: number; ageDays: number };
type ScanResult = { folder: string; totalBytes: number; items: Item[]; truncated: boolean; skipped: number };
type CleanResult = { deleted: number; freedBytes: number; failed: number };
type VmDisk = { path: string; sizeBytes: number; capacityBytes: number; compactable: boolean; status: string };
type VmDiskScan = { folder: string; disks: VmDisk[]; skipped: number };
const bytes = (value: number) => value >= 1024 ** 3 ? `${(value / 1024 ** 3).toFixed(2)} GB` : value >= 1024 ** 2 ? `${(value / 1024 ** 2).toFixed(1)} MB` : value >= 1024 ? `${(value / 1024).toFixed(1)} KB` : `${value} B`;
const fmt = (n: number) => new Intl.NumberFormat('pt-BR').format(n);
function App(){
 const [scan, setScan] = useState<ScanResult | null>(null);
 const [selected, setSelected] = useState<Set<string>>(new Set());
 const [busy, setBusy] = useState(false);
 const [message, setMessage] = useState('Pronto para analisar a pasta temporária do seu usuário.');
 const [result, setResult] = useState<CleanResult | null>(null);
 const [vmFolder, setVmFolder] = useState('');
 const [vmScan, setVmScan] = useState<VmDiskScan | null>(null);
 const chosen = scan?.items.filter(i => selected.has(i.id)) ?? [];
 const chosenBytes = chosen.reduce((s, i) => s + i.size, 0);
 const allChecked = !!scan?.items.length && selected.size === scan.items.length;
 const scanNow = async () => {
   setBusy(true); setResult(null); setSelected(new Set());
   try { const data = await invoke<ScanResult>('scan_temp'); setScan(data); setMessage(`Análise concluída: ${fmt(data.items.length)} arquivos elegíveis encontrados.`); }
   catch(e) { setMessage(`Erro na análise: ${String(e)}`); }
   finally { setBusy(false); }
 };
 const scanVms = async () => { if(busy) return; setBusy(true); setVmScan(null); try { const data = await invoke<VmDiskScan>('scan_vm_disks', {folder:vmFolder.trim()}); setVmScan(data); setMessage(`${fmt(data.disks.length)} discos VDI encontrados.`); } catch(e) { setMessage(`Erro ao analisar VMs: ${String(e)}`); } finally { setBusy(false); } };
 const compactVm = async (disk:VmDisk) => { if(busy || !disk.compactable) return; if(!window.confirm(`Compactar este disco virtual?\n\n${disk.path}\n\nA m\u00e1quina virtual precisa estar desligada. Para recuperar mais espa\u00e7o, execute sudo fstrim -av dentro do Linux, desligue a VM e analise novamente.`)) return; setBusy(true); try { await invoke('compact_vm_disk', {path:disk.path}); setMessage(`Compacta\u00e7\u00e3o conclu\u00edda: ${disk.path}`); setVmScan(null); } catch(e) { setMessage(`Erro ao compactar: ${String(e)}`); } finally { setBusy(false); } };
 const toggle = (id:string) => setSelected(previous => { const next=new Set(previous); next.has(id)?next.delete(id):next.add(id); return next; });
 const clean = async () => {
   if(!selected.size || busy) return;
   if(!window.confirm(`Excluir permanentemente ${fmt(selected.size)} arquivo(s) temporário(s) selecionado(s), totalizando aproximadamente ${bytes(chosenBytes)}?\n\nA exclusão NÃO poderá ser desfeita.`)) return;
   setBusy(true);
   try { const r = await invoke<CleanResult>('clean_selected', {ids:[...selected]}); setResult(r); setMessage(`${fmt(r.deleted)} arquivos excluídos. ${bytes(r.freedBytes)} recuperados. ${fmt(r.failed)} falha(s).`); setScan(null); setSelected(new Set()); }
   catch(e) { setMessage(`Erro ao limpar: ${String(e)}`); }
   finally { setBusy(false); }
 };
 return <div className="app"><aside className="sidebar">
   <div className="brand"><span className="brand-icon"><HardDrive size={23}/></span><span><b>WinClean</b><small>Open Source · v0.1</small></span></div>
   <div className="side-title">GERENCIAMENTO</div><div className="nav active"><ScanSearch size={19}/> Limpeza segura</div>
   <div className="nav active"><HardDrive size={19}/> Discos de VM</div><div className="nav muted"><Files size={19}/> Arquivos grandes <span>Em breve</span></div>
   <div className="nav muted"><Settings2 size={19}/> Sistema <span>Em breve</span></div>
   <div className="sidebar-bottom"><ShieldCheck size={20}/><div><b>Privacidade primeiro</b><p>Análise local. Nenhum arquivo enviado para servidores.</p></div></div>
 </aside><main><header><div><div className="eyebrow">DASHBOARD / LIMPEZA</div><h1>Limpeza inteligente</h1><p>Recupere espaço com controle total sobre o que será removido.</p></div><div className="version"> <span className="status-dot"/> Modo seguro</div></header>
 <section className="vm-panel"><div className="section-heading"><div><h2>Discos de m&aacute;quinas virtuais</h2><p>Encontre discos VDI do VirtualBox e compacte os discos din&acirc;micos eleg&iacute;veis.</p></div></div><div className="vm-controls"><input aria-label="Pasta de discos virtuais" value={vmFolder} onChange={e=>setVmFolder(e.target.value)} placeholder="Em branco: VirtualBox VMs do seu perfil"/><button className="btn primary-btn" onClick={scanVms} disabled={busy}><ScanSearch size={17}/>{busy?'Analisando...':'Analisar pasta'}</button></div><p className="vm-note">A an&aacute;lise n&atilde;o altera arquivos. S&oacute; discos base VDI din&acirc;micos desbloqueados podem ser compactados. Desligue a VM antes; snapshots e discos fixos s&atilde;o ignorados.</p>{vmScan&&<><div className="vm-path">Pasta: {vmScan.folder}{vmScan.skipped>0?` \u00b7 ${fmt(vmScan.skipped)} entradas ignoradas`:''}</div>{vmScan.disks.length?<div className="vm-list">{vmScan.disks.map(d=><div className="vm-row" key={d.path}><div className="vm-meta"><b title={d.path}>{d.path}</b><span>{d.status}{d.compactable?` \u00b7 ${bytes(d.sizeBytes)} ocupados de ${bytes(d.capacityBytes)} de capacidade`:''}</span></div><button className="btn compact-btn" disabled={busy||!d.compactable} onClick={()=>compactVm(d)}>Compactar</button></div>)}</div>:<div className="vm-empty">Nenhum disco VDI encontrado nesta pasta.</div>}</>}</section><div className="summary-grid"><section className="summary primary"><div className="metric-icon blue"><FolderOpen size={22}/></div><small>Espaço potencialmente recuperável</small><strong>{scan?bytes(scan.totalBytes):'—'}</strong><span>Arquivos temporários com mais de 24 horas</span></section><section className="summary"><div className="metric-icon purple"><Files size={22}/></div><small>Arquivos encontrados</small><strong>{scan?fmt(scan.items.length):'—'}</strong><span>Somente arquivos individuais</span></section><section className="summary"><div className="metric-icon green"><ShieldCheck size={22}/></div><small>Para exclusão</small><strong>{bytes(chosenBytes)}</strong><span>{fmt(selected.size)} arquivo(s) selecionado(s)</span></section></div>
 <div className="section-heading"><div><h2>Arquivos temporários</h2><p>Visualize o conteúdo elegível antes de excluir.</p></div><button className="btn primary-btn" onClick={scanNow} disabled={busy}><RefreshCcw size={17} className={busy?'spin':''}/>{busy?'Processando...':scan?'Analisar novamente':'Iniciar análise'}</button></div>
 <section className="data-panel"><div className="panel-top"><div className="path"><FolderOpen size={16}/>{scan?.folder ?? 'Pasta TEMP do usuário atual'}</div><div className="pill"><ShieldCheck size={14}/> Sem permissões de administrador</div></div>
 {scan?.items.length ? <><div className="table-wrap"><table><thead><tr><th><input type="checkbox" aria-label="Selecionar todos" checked={allChecked} onChange={e=>setSelected(e.target.checked?new Set(scan.items.map(i=>i.id)):new Set())}/></th><th>Arquivo</th><th>Idade</th><th className="right">Tamanho</th></tr></thead><tbody>{scan.items.slice(0,500).map(i=><tr key={i.id}><td><input type="checkbox" aria-label={`Selecionar ${i.name}`} checked={selected.has(i.id)} onChange={()=>toggle(i.id)}/></td><td className="filename" title={i.name}>{i.name}</td><td>{i.ageDays} dia(s)</td><td className="right size">{bytes(i.size)}</td></tr>)}</tbody></table></div>{scan.items.length>500&&<p className="notice">Exibidos os 500 maiores arquivos. A seleção geral abrange todos os {fmt(scan.items.length)} itens.</p>}</> : <div className="empty"><div className="empty-icon"><ScanSearch size={32}/></div><h3>{scan?'Nenhum arquivo elegível':'Seu armazenamento, sob controle'}</h3><p>{scan?'Nenhum arquivo com mais de 24 horas foi encontrado nesta pasta.':'Clique em “Iniciar análise” para identificar arquivos temporários antigos que podem ser removidos.'}</p></div>}
 <div className="panel-footer"><span><Info size={16}/> {scan?.truncated?'Limite de análise atingido (100 mil itens).':scan?`${fmt(scan.skipped)} entradas ignoradas ou inacessíveis.`:'Nenhum arquivo será removido durante a análise.'}</span><button className="btn danger-btn" disabled={busy||!selected.size} onClick={clean}><Trash2 size={17}/> Limpar selecionados</button></div></section>
 <div className="bottom-grid"><div className="info-card"><ShieldCheck size={23}/><div><b>Proteção por padrão</b><p>Sem limpeza de Registro, arquivos de sistema, diretórios inteiros ou arquivos modificados após a análise.</p></div></div><div className="info-card"><AlertTriangle size={23}/><div><b>Exclusão permanente</b><p>Itens removidos não passam pela Lixeira. Revise a seleção e mantenha backups importantes.</p></div></div></div>
 <div className="message" role="status">{result&&<CheckCircle2 size={17}/>} {message}</div><footer><span>WinClean · MIT License · Feito para Windows 11</span><a href="https://github.com" target="_blank" rel="noreferrer"><Github size={16}/> Projeto open source</a></footer>
 </main></div>;
}
createRoot(document.getElementById('root')!).render(<React.StrictMode><App/></React.StrictMode>);
