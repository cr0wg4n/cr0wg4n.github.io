import repositoriesData from './repositories.json'

export interface Repository {
  name: string,
  url: string,
  description?: string,
  updatedAt?: string,
  openGraphImageUrl?: string,
  stargazerCount: number,
  forkCount: number,
};

const repositories: Repository[] = [
  ...repositoriesData.filter(({stargazerCount})=>stargazerCount >= 3)
    .sort((a,b)=>a.stargazerCount-b.stargazerCount)
    .reverse()
    .filter(({openGraphImageUrl})=>openGraphImageUrl.includes('opengraph')),
];

export default function data(){
  return repositories;
};
