import { test } from 'node:test';
import assert from 'node:assert/strict';
import { detectOS, downloadURL } from '../website/main.js';
test('desktop download buttons detect each OS, preferring client hints', () => {
  assert.equal(detectOS('macOS', ''), 'macos');
  assert.equal(detectOS('Windows', ''), 'windows');
  assert.equal(detectOS('Linux', ''), 'linux');
  assert.equal(detectOS('', 'Mozilla/5.0 (X11; Linux x86_64)'), 'linux');
  assert.equal(detectOS('MacIntel', 'Mozilla/5.0 (Macintosh; Intel Mac OS X)'), 'macos');
});
test('phones do not receive a desktop installer', () => {
  assert.equal(detectOS('Linux arm', 'Android'), 'mobile');
  assert.equal(detectOS('iPhone', 'like Mac OS X'), 'mobile');
  assert.equal(downloadURL('mobile'), '#downloads');
  assert.equal(downloadURL('unknown'), '#downloads');
});
test('each desktop target points to a versioned artifact in the intended public repo', () => {
  for (const os of ['macos', 'windows', 'linux']) {
    const url = new URL(downloadURL(os));
    assert.equal(url.hostname, 'github.com');
    assert.match(url.pathname, /^\/Tengify\/promptbeacon\/releases\/download\/v0\.2\.0\//);
  }
});
