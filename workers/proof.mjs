import assert from 'node:assert/strict';
import { readFile, rm } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
const require = createRequire(import.meta.url);
const owner = createRequire(require.resolve('wrangler/package.json'));
const { build } = owner('esbuild');
const { Miniflare } = owner('miniflare');
const root = fileURLToPath(new URL('.', import.meta.url));
const output = `${root}proof.bundle.mjs`;
await build({ entryPoints: [`${root}worker.mjs`], outfile: output, bundle: true, format: 'esm', platform: 'browser', target: 'es2022', plugins: [{ name: 'wasm', setup(b) { b.onResolve({ filter: /\.wasm$/ }, args => ({ path: args.path, external: true })); } }] });
const mf = new Miniflare({ modules: true, scriptPath: output, modulesRoot: root, modulesRules: [{ type: 'CompiledWasm', include: ['**/*.wasm'] }], compatibilityDate: '2026-07-08', bindings: { FIRST_SECRET: 'synthetic-first', SECOND_SECRET: 'synthetic-second' } });
try {
  const modes = ['success', 'second', 'rotate', 'loss', 'closed', 'missing', 'wrong-type', 'throws', 'invalid-config', 'success'];
  for (const mode of modes) {
    const response = await mf.dispatchFetch(`http://local/?mode=${mode}`);
    const body = await response.text();
    assert.equal(response.status, 200, `mode=${mode}: ${body}`);
    assert(!body.includes('synthetic-'));
    assert.equal(JSON.parse(body).outcome, ['missing', 'wrong-type', 'throws', 'invalid-config'].includes(mode) ? 'startup-rejected' : 'passed');
  }
  // Independent event scopes under concurrent HTTP admission.
  const responses = await Promise.all(['success', 'second', 'success', 'second'].map(mode => mf.dispatchFetch(`http://local/?mode=${mode}`)));
  for (const response of responses) assert.equal(response.status, 200);
  const wasm = await readFile(`${root}pkg/lenso_secrets_workers_smoke_bg.wasm`);
  console.log(JSON.stringify({ passed: 14, runtime: 'workerd', workers_runtime: '0.1.2', wasm_sha256: createHash('sha256').update(wasm).digest('hex') }, null, 2));
} finally {
  await mf.dispose();
  await rm(output, { force: true });
}
