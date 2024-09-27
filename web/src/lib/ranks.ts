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

export interface LeaderboardInfo {
  ranks: RankingInfo[];
  generatedAt: Date;
}

const DEFAULT_LEADERBOARD_INFO: LeaderboardInfo = {
  ranks: [],
  generatedAt: new Date(0),
};

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

function mapLeaderboardToRanking(leaderboard: LeaderboardExport): LeaderboardInfo {
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
  return {
    ranks: rankingInfo,
    generatedAt: new Date(leaderboard.generatedAt * 1000)
  };
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


export const overallRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('overall.bin'));
export const chatOnlyRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('chat-only.bin'));
export const nonvipsRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('nonvips.bin'));
export const copypastaRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('copypasta.bin'));
export const bitsRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('bits-only.bin'));
export const subsRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('subs-only.bin'));
export const discordRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('discordlivestream.bin'))
export const partnersRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('partners-only.bin'))
export const bilibiliRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('bilibililivestreamchat.bin'))
export const adventureTheFarmRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('adventures_farm.bin'))
export const emoteRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('top-emote.bin'));
export const ironmousePixelRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('ironmouse_pxls.bin'));
export const pxlsRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('casual_pxls.bin'));
export const ironmouseChatRank = readable(DEFAULT_LEADERBOARD_INFO, makeRankingInfo('ironmousecanvaschat.bin'));
