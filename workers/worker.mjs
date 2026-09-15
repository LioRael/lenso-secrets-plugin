// Local qualification only: every value used here is synthetic fixture data.
import * as generated from './pkg/lenso_secrets_workers_smoke.js';
import wasmModule from './pkg/lenso_secrets_workers_smoke_bg.wasm';
import { createWorkersHttpHost, createEventScope } from '@lenso/workers-runtime';

export default createWorkersHttpHost({
  bindings: {
    ...generated,
    async handle_http(_input, scope) {
      const outcome = await generated.exercise(scope.lookup, scope.mutate, scope.mode, scope.expected);
      const count = scope.reads();
      // Unknown/invalid references must never call the binding lookup.
      const expectedReads = scope.mode === 'invalid-config' ? 0
        : ['missing', 'wrong-type', 'throws'].includes(scope.mode) ? 1
        : ['rotate', 'loss', 'closed'].includes(scope.mode) ? 3 : 2;
      if (count !== expectedReads) throw new Error('Unexpected binding access count');
      return JSON.stringify({ status: 200, headers: [], body: Array.from(new TextEncoder().encode(JSON.stringify({ outcome, reads: count }))), shutdown: 'clean' });
    },
  },
  wasmModule,
  createScope(request, env) {
    const url = new URL(request.url);
    const mode = url.searchParams.get('mode') || 'success';
    return createEventScope(resources => {
      let available = mode !== 'missing';
      let value = mode === 'wrong-type' ? {} : mode === 'second' ? env.SECOND_SECRET : env.FIRST_SECRET;
      let reads = 0;
      return {
        mode,
        expected: mode === 'second' ? env.SECOND_SECRET : env.FIRST_SECRET,
        reads: () => reads,
        lookup(name) {
          reads += 1;
          if (resources.closed) throw new Error('event_scope_closed');
          if (mode === 'throws') throw new Error(env.FIRST_SECRET);
          return available && name === 'APP_SECRET' ? value : undefined;
        },
        mutate() {
          if (mode === 'rotate') value = 'rotated-fixture';
          if (mode === 'loss') available = false;
          if (mode === 'closed') resources.abort();
        },
      };
    });
  },
});
