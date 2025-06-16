import React from 'react';
import SlideCard from '../Cards/SlideCard';
import data from '../../data/slides';

const SlidesList: React.FC = ()=>{
  return <>
    <div className='grid xl:grid-cols-5 lg:grid-cols-4 md:grid-cols-3 grid-cols-2 gap-3 md:gap-5'>
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