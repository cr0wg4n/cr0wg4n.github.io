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

function isInWhiteList(name: string): boolean {
  const whitelist: string[] = [
    'dev-station-extension',
    'rss-url-finder'
  ]
  return whitelist.includes(name)
}

const repositories: Repository[] = [
  ...repositoriesData.filter(({stargazerCount, name}) => stargazerCount >= 3 || 
  isInWhiteList(name))
    .sort((a,b)=>a.stargazerCount-b.stargazerCount)
    .reverse()
    .filter(({openGraphImageUrl}) => openGraphImageUrl.includes('opengraph')),
];

export default function data(){
  return repositories;
};
