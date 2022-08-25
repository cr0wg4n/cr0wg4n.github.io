import React from 'react';
import PostCard from '../Cards/PostCard';
import data from '../../data/posts';

const PostList: React.FC = ()=>{
  return <>
    <div className='grid grid-cols-12 gap-3 md:gap-5'>
      {data().map((post,i)=>
        <PostCard
          key={i}
          name={post.name}
          url={post.url}
          cover={post.cover}
          date={post.date}
        />)}
    </div>
  </>
}

export default PostList;