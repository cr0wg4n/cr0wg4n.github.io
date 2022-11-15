import postData from './posts.json'

export interface PostCard {
  name: string,
  url: string,
  cover?: string,
  date?:string
};

const posts: PostCard[] = [
  ...postData,
];

export default function data(){
  return posts;
};
