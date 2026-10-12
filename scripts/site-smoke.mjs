import { chromium } from '@playwright/test';
import assert from 'node:assert/strict';
import { mkdir, writeFile } from 'node:fs/promises';
await mkdir('work/site-qa', { recursive: true });
const siteURL = process.env.PROMPTBEACON_SITE_URL || 'http://127.0.0.1:4317';
const browser = await chromium.launch({ headless: true });
const errors = [];
const results = [];
try {
  for (const [name, width, height] of [['desktop', 1440, 1100], ['mobile', 390, 844], ['compact', 320, 740]]) {
    const context = await browser.newContext({ viewport: { width, height }, reducedMotion: 'reduce' });
    const page = await context.newPage();
    page.on('pageerror', e => errors.push(e.message));
    await page.goto(siteURL);
    await page.evaluate(() => document.fonts.ready);
    assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), true, `${name} overflow`);
    await page.getByRole('button', { name: 'Harbor API Model B' }).click();
    assert.equal(await page.locator('#beacon-model').textContent(), 'Model B');
    assert.equal(await page.locator('#beacon-effort').textContent(), 'High');
    assert.equal(await page.locator('#beacon-bars .filled').count(), 3);
    await page.getByLabel('Effort', { exact: true }).selectOption('Max');
    assert.equal(await page.locator('#beacon-bars .filled').count(), 5);
    await page.getByRole('button', { name: 'Tint', exact: true }).click();
    assert.equal(await page.locator('#demo-window').getAttribute('data-style'), 'tint');
    await page.getByRole('button', { name: 'Orbit CLI Model C' }).click();
    assert.equal(await page.locator('#beacon-effort').textContent(), 'Low');
    await page.getByRole('button', { name: 'Border', exact: true }).click();
    await page.getByRole('button', { name: 'Atlas UI Model A' }).click();
    await page.evaluate(() => scrollTo(0, 0));
    await page.screenshot({ path: `work/site-qa/${name}.png`, fullPage: true });
    await page.screenshot({ path: `work/site-qa/${name}-hero.png` });
    results.push(`${name}: no overflow; session, effort, and appearance controls work`);
    await context.close();
  }
  for (const [platform, agent, expected] of [
    ['macOS', 'Macintosh', 'Download for macOS'],
    ['Windows', 'Windows NT 10.0', 'Download for Windows'],
    ['Linux', 'X11; Linux x86_64', 'Download for Linux'],
    ['iPhone', 'iPhone like Mac OS X', 'View desktop downloads'],
    ['MacIntel', 'Macintosh', 'View desktop downloads'],
    ['Unknown', 'Unknown', 'Choose your download'],
  ]) {
    const context = await browser.newContext();
    await context.addInitScript(({platform, agent}) => {
      Object.defineProperty(navigator, 'platform', { get: () => platform });
      Object.defineProperty(navigator, 'userAgent', { get: () => agent });
      Object.defineProperty(navigator, 'userAgentData', { get: () => ({platform}) });
      Object.defineProperty(navigator, 'maxTouchPoints', { get: () => platform === 'MacIntel' ? 5 : 0 });
    }, {platform, agent});
    const page = await context.newPage();
    await page.goto(siteURL);
    await page.waitForFunction(() => document.querySelector('#beacon-bars .filled'));
    assert.equal(await page.locator('[data-download-label]').first().textContent(), expected);
    if (expected.startsWith('Download')) assert.match(await page.locator('[data-download]').first().getAttribute('href'), /github.com\/Tengify\/promptbeacon\/releases\/download/);
    results.push(`${platform}: ${expected}`);
    await context.close();
  }
  assert.deepEqual(errors, []);
  await writeFile('work/site-qa/results.json', JSON.stringify({results, errors}, null, 2));
  console.log(JSON.stringify({results, errors}, null, 2));
} finally { await browser.close(); }
