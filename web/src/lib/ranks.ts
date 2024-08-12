import { readable } from 'svelte/store';
import axios from 'axios';
import { LeaderboardExport } from './leaderboardExportTypes';

export interface Badge {
  description: string;
  image_url: string;
};

export interface RankingInfo {
  id: string;
  rank: number;
  elo: number;
  username: string;
  delta: number;
  avatar: string;
  badges: Badge[];
}

function makeRankingInfo(path: string) {
  return (set: (arg0: any) => void) => {
    axios.get(`./${path}`, { responseType: 'arraybuffer' })
      .then(result => {
        if (result.status !== 200) {
          console.error(`Cannot fetch leaderboard from ${path}`);
          return;
        }
        try {
          const data = new Uint8Array(result.data);
          const leaderboard = LeaderboardExport.decode(data);
          const rankingInfo: RankingInfo[] = [];
          for (const item of leaderboard.items) {
            const badges = [];
            for (const badge of item.badges) {
              badges.push({
                description: badge.description,
                image_url: badge.imageUrl
              });
            }
            rankingInfo.push({
              id: item.id,
              rank: item.rank,
              elo: item.elo,
              username: item.username,
              delta: item.delta,
              avatar: item.avatar,
              badges: badges
            });
            set(rankingInfo);
          }
        } catch (error) {
          // TODO: Create an error handling page for this.
          alert(`Error parsing leaderboard from ${path}: ${error}`);
        }
      });
    return () => { };
  };
}

export const overallRank = readable([], makeRankingInfo('overall.bin'));
export const chatOnlyRank = readable([], makeRankingInfo('chat-only.bin'));
export const nonvipsRank = readable([], makeRankingInfo('nonvips.bin'));
export const copypastaRank = readable([], makeRankingInfo('copypasta.bin'));
export const bitsRank = readable([], makeRankingInfo('bits-only.bin'));
export const subsRank = readable([], makeRankingInfo('subs-only.bin'));
export const discordRank = readable([], makeRankingInfo('discordlivestream.bin'))
