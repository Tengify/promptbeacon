import { release } from './releases.js';
export function detectOS(platform = '', agent = '') {
  const text = `${platform} ${agent}`;
  if (/Android|iPhone|iPad|iPod/i.test(text)) return 'mobile';
  if (/Win/i.test(platform) || /Windows/i.test(agent)) return 'windows';
  if (/Mac/i.test(platform) || /Macintosh|Mac OS X/i.test(agent)) return 'macos';
  if (/Linux|X11/i.test(text)) return 'linux';
  return 'unknown';
}
export function downloadURL(os) {
  return release.assets[os] ? release.base + release.assets[os] : '#downloads';
}
if (typeof document !== 'undefined') {
  const platform = navigator.userAgentData?.platform || navigator.platform || '';
  const os = navigator.maxTouchPoints > 1 && /Mac/i.test(platform) ? 'mobile' : detectOS(platform, navigator.userAgent);
  const names = { macos: 'macOS', windows: 'Windows', linux: 'Linux' };
  const notes = { macos: 'Apple Silicon + Intel · macOS 11+ · Free & open source', windows: 'Windows 10 / 11 · x64 · Free & open source', linux: 'Debian / Ubuntu · x64 · Free & open source', mobile: 'A desktop app for macOS, Windows, and Linux' };
  document.querySelectorAll('[data-download]').forEach(link => { link.href = downloadURL(os); });
  document.querySelectorAll('[data-download-label]').forEach(label => { label.textContent = names[os] ? `Download for ${names[os]}` : os === 'mobile' ? 'View desktop downloads' : 'Choose your download'; });
  document.querySelectorAll('[data-download-note]').forEach(note => { note.textContent = notes[os] || 'macOS · Windows · Linux / Free & open source'; });
  document.querySelectorAll('[data-platform]').forEach(link => { link.href = downloadURL(link.dataset.platform); });
  const sessions = [
    { title: 'Atlas UI', model: 'Model A', effort: 'Extra High', color: '#2870D5' },
    { title: 'Harbor API', model: 'Model B', effort: 'High', color: '#168577' },
    { title: 'Orbit CLI', model: 'Model C', effort: 'Low', color: '#C45843' },
  ];
  const levels = { Low: 1, Medium: 2, High: 3, 'Extra High': 4, Max: 5 };
  let selected = 0;
  const effort = document.querySelector('#demo-effort');
  const render = () => {
    const session = sessions[selected];
    document.querySelector('.instrument').style.setProperty('--signal', session.color);
    document.querySelector('#session-title').textContent = session.title;
    for (const id of ['composer-model', 'beacon-model']) document.getElementById(id).textContent = session.model;
    for (const id of ['composer-effort', 'beacon-effort']) document.getElementById(id).textContent = session.effort;
    document.querySelectorAll('#beacon-bars i').forEach((bar, i) => bar.classList.toggle('filled', i < levels[session.effort]));
    document.querySelectorAll('[data-session]').forEach(button => button.setAttribute('aria-pressed', String(Number(button.dataset.session) === selected)));
    effort.value = session.effort;
  };
  document.querySelectorAll('[data-session]').forEach(button => button.addEventListener('click', () => { selected = Number(button.dataset.session); render(); }));
  document.querySelectorAll('button[data-style]').forEach(button => button.addEventListener('click', () => {
    document.querySelector('#demo-window').dataset.style = button.dataset.style;
    document.querySelectorAll('button[data-style]').forEach(item => item.setAttribute('aria-pressed', String(item === button)));
  }));
  effort.addEventListener('change', () => { sessions[selected].effort = effort.value; render(); });
  render();
}
