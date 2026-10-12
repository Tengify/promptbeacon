export interface ColorRule { name: string; pattern: string; color: string }
export interface Config { enabled: boolean; style: string; borderWidth: number; tintOpacity: number; badgePosition: string; pollMs: number; rules: ColorRule[] }
export interface Bounds { x: number; y: number; width: number; height: number; logical: boolean }
export interface BeaconState { status: string; application: string | null; model: string | null; effort: string | null; source: string; message: string; color: string; bounds: Bounds | null; preview: boolean }
export interface AppStatus { config: Config; beacon: BeaconState; platform: string; permission: boolean; settingsWarning: string | null; integration: string; dataPath: string; version: string }
export const defaults: Config = { enabled: true, style: 'border', borderWidth: 3, tintOpacity: 0.045, badgePosition: 'bottom-right', pollMs: 650, rules: [
  { name: 'GPT-6.1 Sol', pattern: 'gpt-6.1-sol', color: '#2870D5' },
  { name: 'GPT-6 Astra', pattern: 'gpt-6-astra', color: '#8956C8' },
  { name: 'GPT-6 Sol', pattern: 'gpt-6-sol', color: '#168577' },
  { name: 'GPT-6 Luna', pattern: 'gpt-6-luna', color: '#AA691D' },
  { name: 'Claude Opus', pattern: 'opus', color: '#C45843' },
  { name: 'Claude Sonnet', pattern: 'sonnet', color: '#AF4589' },
  { name: 'Claude Haiku', pattern: 'haiku', color: '#537D24' },
] };
export function normalize(text: string): string { return text.toLowerCase().replace(/[^\p{L}\p{N}]/gu, ''); }
export function modelColor(model: string | null, config: Config): string {
  if (!model) return '#737983';
  return config.rules.filter(r => normalize(model).includes(normalize(r.pattern))).sort((a,b) => normalize(b.pattern).length - normalize(a.pattern).length)[0]?.color ?? '#737983';
}
export function effortBars(effort: string | null): number { return ({ None:0, Minimal:1, Low:1, Medium:2, High:3, 'Extra High':4, Max:5, Ultra:5 } as Record<string,number>)[effort ?? ''] ?? 0; }
export function escapeHtml(value: string): string { return value.replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]!)); }
export function validateConfig(config: Config): string | null {
  if (!config || typeof config !== 'object' || typeof config.enabled !== 'boolean' || !Array.isArray(config.rules)) return 'Saved settings have an invalid format.';
  if (!Number.isInteger(config.borderWidth) || !Number.isInteger(config.pollMs)) return 'Appearance values must be whole numbers.';
  if (!['border','tint','badge'].includes(config.style)) return 'Choose a valid style.';
  if (!['bottom-right','bottom-left','top-right'].includes(config.badgePosition)) return 'Choose a valid position.';
  if (config.borderWidth < 1 || config.borderWidth > 8 || config.pollMs < 350 || config.pollMs > 3000 || !Number.isFinite(config.tintOpacity) || config.tintOpacity < 0 || config.tintOpacity > .15) return 'Appearance values are out of range.';
  if (config.rules.length > 64) return 'Use at most 64 model colors.';
  const seen = new Set<string>();
  for (const r of config.rules) {
    if (!r || typeof r.name !== 'string' || typeof r.pattern !== 'string' || typeof r.color !== 'string') return 'Each model needs a valid name, match phrase, and color.';
    if (!r.name.trim() || !r.pattern.trim() || !normalize(r.pattern) || r.name.length > 80 || r.pattern.length > 80) return 'Each model needs a name and match phrase (up to 80 characters).';
    if (!/^#[0-9a-f]{6}$/i.test(r.color)) return 'Use a six-digit hex color.';
    if (seen.has(normalize(r.pattern))) return 'Each model match phrase must be unique.';
    seen.add(normalize(r.pattern));
  }
  return null;
}
