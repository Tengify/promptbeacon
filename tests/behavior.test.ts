import { describe, expect, it } from 'vitest';
import { defaults, effortBars, escapeHtml, modelColor, validateConfig } from '../src/types';
describe('model color rules',()=>{
 it('normalizes the model names shown by different apps',()=>expect(modelColor('GPT 6.1 Sol',defaults)).toBe('#2870D5'));
 it('uses neutral for unknown models without guessing',()=>expect(modelColor('unrecognized model',defaults)).toBe('#737983'));
 it('uses the most specific match',()=>{const config=structuredClone(defaults);config.rules.unshift({name:'Any GPT',pattern:'gpt',color:'#ffffff'});expect(modelColor('gpt-6.1-sol',config)).toBe('#2870D5');});
 it('rejects duplicate and empty match phrases',()=>{const config=structuredClone(defaults);config.rules.push({...config.rules[0]});expect(validateConfig(config)).toMatch(/unique/);config.rules[0].pattern='';expect(validateConfig(config)).toMatch(/match phrase/);});
 it('rejects CSS injection and invalid appearance',()=>{const config=structuredClone(defaults);config.rules[0].color='red;display:none';expect(validateConfig(config)).not.toBeNull();config.rules[0].color='#2870D5';config.tintOpacity=Infinity;expect(validateConfig(config)).not.toBeNull();});
});
describe('honest effort display',()=>{it('has no filled bars when effort is unknown',()=>expect(effortBars(null)).toBe(0));it('distinguishes Extra High from High',()=>expect(effortBars('Extra High')).toBeGreaterThan(effortBars('High')));});
describe('safe labels',()=>{it('escapes user-defined names in markup',()=>expect(escapeHtml('<img src=x onerror="alert(1)">')).toBe('&lt;img src=x onerror=&quot;alert(1)&quot;&gt;'));});
