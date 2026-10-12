import { cp, mkdir, rm } from 'node:fs/promises';
const destination = new URL('../website/dist/', import.meta.url);
await rm(destination, { recursive: true, force: true });
await mkdir(destination, { recursive: true });
for (const file of ['index.html', 'style.css', 'main.js', 'releases.js', 'assets']) {
  await cp(new URL(`../website/${file}`, import.meta.url), new URL(file, destination), { recursive: true });
}
console.log('Static landing page ready in website/dist. No external runtime or analytics.');
