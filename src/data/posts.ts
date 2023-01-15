import postsData from './posts.json'
import extraPostData from './extra-posts.json'

export interface PostCard {
  name: string,
  url: string,
  cover?: string,
  date?:string
};

const posts: PostCard[] = [
  ...extraPostData,
  ...postsData,
];

export default function data(){
  return posts;
};
