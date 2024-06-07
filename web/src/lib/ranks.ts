import { readable } from 'svelte/store';
import axios from 'axios';

export interface RankingInfo {
  rank: number;
  elo: number;
  username: string;
  delta: number;
  avatar: string;
}

function makeRankingInfo(path: string) {
  return (set: (arg0: unknown) => void) => {
    axios.get(`./${path}`).then(result => {
      if (result.status !== 200) {
        console.error(`Cannot fetch leaderboard from ${path}`);
      }
      set(result.data as RankingInfo[]);
    });
    return () => {};
  };
}

export const overallRank = readable([], makeRankingInfo('overall.json'));
export const chatOnlyRank = readable([], makeRankingInfo('chat-only.json'));
export const nonvipsRank = readable([], makeRankingInfo('nonvips.json'));
export const copypastaRank = readable([], makeRankingInfo('copypasta.json'));
export const bitsRank = readable([], makeRankingInfo('bits-only.json'));
export const subsRank = readable([], makeRankingInfo('subs-only.json'));
