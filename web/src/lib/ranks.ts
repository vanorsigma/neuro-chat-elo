import { readable } from 'svelte/store';
import axios from 'axios';
import protobuf from 'protobufjs';

const protoPath = './src/lib/leaderboard.proto';  // Adjust the path to where your proto file is located
let root: protobuf.Root;

protobuf.load(protoPath)
  .then((loadedRoot) => {
    root = loadedRoot;
  })
  .catch((error) => {
    console.error('Error loading the .proto file:', error);
  });


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
          const LeaderboardExport = root.lookupType("leaderboard.LeaderboardExport");
          const message = LeaderboardExport.decode(new Uint8Array(result.data));
          const object = LeaderboardExport.toObject(message, {
            longs: String,
            enums: String,
            bytes: String,
          });
          set(object.items as any);
        } catch (err) {
          console.error('Protobuf decoding error:', err);
        }
      });
    return () => {};
  };
}

export const overallRank = readable([], makeRankingInfo('overall.bin'));
export const chatOnlyRank = readable([], makeRankingInfo('chat-only.bin'));
export const nonvipsRank = readable([], makeRankingInfo('nonvips.bin'));
export const copypastaRank = readable([], makeRankingInfo('copypasta.bin'));
export const bitsRank = readable([], makeRankingInfo('bits-only.bin'));
export const subsRank = readable([], makeRankingInfo('subs-only.bin'));
export const discordRank = readable([], makeRankingInfo('discordlivestream.bin'))
