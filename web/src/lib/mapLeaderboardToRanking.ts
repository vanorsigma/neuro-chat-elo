import { LeaderboardExport } from '../home/codingindex/neuro-chat-elo/web/src/gen/leaderboardExportTypes';
import { LeaderboardInfo, RankingInfo, convertProtoBadges } from '../home/codingindex/neuro-chat-elo/web/src/lib/ranks';

export function mapLeaderboardToRanking(leaderboard: LeaderboardExport): LeaderboardInfo {
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
  return {};
}
