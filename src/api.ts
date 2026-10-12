import { invoke, isTauri } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { defaults, validateConfig, type AppStatus, type Config, type BeaconState } from './types';
export const native = isTauri();
const idle: BeaconState = {status:'inactive',application:null,model:null,effort:null,source:'none',message:'Open a Codex or Claude Code session to see its selection.',color:'#737983',bounds:null,preview:false};
let fallback = structuredClone(defaults);
try { const saved = localStorage.getItem('promptbeacon-preview'); if (saved) { const value: Config = JSON.parse(saved); if (!validateConfig(value)) fallback = value; } } catch { /* Preview falls back to defaults. */ }
export async function status(): Promise<AppStatus> {
  if (native) return invoke('get_status');
  return {config:fallback,beacon:idle,platform:'browser',permission:false,settingsWarning:null,integration:'Open the desktop application to generate the status-line configuration.',dataPath:'Browser preview only',version:'0.2.0'};
}
export async function save(config: Config): Promise<void> { if (native) return invoke('save_settings',{config}); fallback=structuredClone(config); localStorage.setItem('promptbeacon-preview',JSON.stringify(config)); }
export async function command(name: string, args?: Record<string,unknown>): Promise<void> { if (native) return invoke(name,args); }
const subscriptions = new Set<() => void>();
let disposed = false;
addEventListener('pagehide', () => { disposed = true; for (const cleanup of subscriptions) cleanup(); subscriptions.clear(); });
export async function on<T>(event: string, handler:(payload:T)=>void): Promise<()=>void> {
  if (!native) return () => {};
  const unlisten = await listen<T>(event,e=>handler(e.payload));
  if (disposed) { unlisten(); return () => {}; }
  const cleanup = () => { unlisten(); subscriptions.delete(cleanup); };
  subscriptions.add(cleanup);
  return cleanup;
}
