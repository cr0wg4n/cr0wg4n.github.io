import React from 'react';
import data from '../../data/slides';
import SlideCard from '../Cards/SlideCard';

const SlidesList: React.FC = ()=>{
  return <>
    <div className='grid grid-cols-12 gap-3 md:gap-5'>
      {data().map((post,i)=>
        <SlideCard
          key={i}
          name={post.name}
          url={post.url}
          cover={post.cover}
        />)}
    </div>
  </>
}

export default SlidesList;