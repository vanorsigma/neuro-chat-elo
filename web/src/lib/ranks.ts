import { readable } from 'svelte/store';

// TODO: Currently dummy data
export const ranks = readable([{
  rank: 1,
  elo: 100.2,
  username: 'hjalnir',
  delta: 1.0
},
{
  rank: 2,
  elo: 200.2,
  username: 'cjmaxik',
  delta: 1.0
}, ...Array.from({ length: 100 }, (v, i) => ({
  rank: i + 2,
  elo: 100,
  username: `user ${i + 2}`,
  delta: 1.0
}))], () => {});
