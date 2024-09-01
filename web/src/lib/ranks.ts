import { readable } from 'svelte/store';
import axios from 'axios';
import { BadgeInformation, LeaderboardExport } from '../gen/leaderboardExportTypes';

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
          const rankingInfo = mapLeaderboardToRanking(leaderboard);
          set(rankingInfo);
        } catch (error) {
          handleError(path, error);
        }
      });
    return () => {};
  };
}

function mapLeaderboardToRanking(leaderboard: LeaderboardExport): RankingInfo[] {
  const rankingInfo: RankingInfo[] = leaderboard.items.map(item => {
    const badges = convertProtoBadges(item.badges);
    return {
      id: item.id,
      rank: item.rank,
      elo: item.elo,
      username: item.username,
      delta: item.delta,
      avatar: item.avatar,
      badges: badges
    };
  });
  return rankingInfo;
}

function convertProtoBadges(badges: BadgeInformation[]): Badge[] {
  return badges.map(badge => ({
    description: badge.description,
    image_url: badge.imageUrl
  }));
}

function handleError(path: string, error: any) {
  // TODO: Create an error handling page for this.
  alert(`Error parsing leaderboard from ${path}: ${error}`);
}


export const overallRank = readable([], makeRankingInfo('overall.bin'));
export const chatOnlyRank = readable([], makeRankingInfo('chat-only.bin'));
export const nonvipsRank = readable([], makeRankingInfo('nonvips.bin'));
export const copypastaRank = readable([], makeRankingInfo('copypasta.bin'));
export const bitsRank = readable([], makeRankingInfo('bits-only.bin'));
export const subsRank = readable([], makeRankingInfo('subs-only.bin'));
export const discordRank = readable([], makeRankingInfo('discordlivestream.bin'))
export const partnersRank = readable([], makeRankingInfo('partners-only.bin'))
export const adventureTheFarmRank = readable([], makeRankingInfo('adventures_farm.bin'))
