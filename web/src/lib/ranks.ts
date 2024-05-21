import { readable } from 'svelte/store';

export interface RankingInfo {
  rank: number;
  elo: number;
  username: string;
  delta: number;
  avatar: string;
}

// TODO: Currently dummy data
export const ranks: RankingInfo[] = readable([{
  rank: 1,
  elo: 200.2,
  username: 'hjalnir',
  delta: 1.0,
  avatar: 'https://static-cdn.jtvnw.net/jtv_user_pictures/90e5b4cf-72ce-42f4-9685-a5794bfec28d-profile_image-300x300.png'
},
{
  rank: 2,
  elo: 150,
  username: 'cjmaxik',
  delta: 1.0,
  avatar: 'https://static-cdn.jtvnw.net/jtv_user_pictures/d75d49ea-af2d-410b-9c63-2f8dcb341122-profile_image-300x300.png'
}, ...Array.from({ length: 100 }, (v, i) => ({
  rank: i + 3,
  elo: 100 - i,
  username: `user ${i + 3}`,
  delta: 1.0,
  avatar: 'https://i.pinimg.com/474x/b8/10/b7/b810b717e748149f5b8a39daabff88a4.jpg'
}))], () => {});

export const altRanks: RankingInfo[] = readable([
  ...Array.from({ length: 100 }, (v, i) => ({
    rank: i + 1,
    elo: 900 - i,
    username: `user ${i + 3}`,
    delta: 1.0,
    avatar: 'https://i.pinimg.com/474x/b8/10/b7/b810b717e748149f5b8a39daabff88a4.jpg'
  }))], () => {});
