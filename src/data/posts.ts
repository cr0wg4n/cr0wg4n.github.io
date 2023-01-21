import postsData from './posts.json'
import extraPostData from './extra-posts.json'
import { orderByDatetime } from '../utils'

export interface PostCard {
  name: string,
  url: string,
  cover?: string,
  date?:string
};

const posts: PostCard[] = orderByDatetime([
  ...extraPostData,
  ...postsData,
], 'date');

export default function data(){
  return posts;
};
