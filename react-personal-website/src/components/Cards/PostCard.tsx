import React from 'react';

export type PostCardProps = {
    children?: React.ReactNode,
    name: string,
    url: string,
    cover?: string,
    date?:string
}

const PostCard: React.FC<PostCardProps> = ({
    name='',
    url='',
    cover='',
    date='00/00/00'
}: PostCardProps)=>{
  
  const bgColors = [
    'bg-amber-500',
    'bg-lime-600',
    'bg-green-800',
    'bg-sky-500',
    'bg-indigo-700',
    'bg-fuchsia-500',
    'bg-pink-600',
    'bg-rose-700'
  ];
  const [bgColor] = React.useState(bgColors[Math.floor(Math.random() * (bgColors.length))]);

  return <>
      <div className='col-span-6 md:col-span-3 relative  hover:cursor-pointer'>
          <a href={url} target={'_blank'} rel={"noreferrer"}>
            <div className='w-full overflow-hidden h-32'>
              {cover?
                <img src={cover} alt={name} className='rounded-lg object-cover w-full h-full'/>:
                <img src={'/images/pattern.png'} alt={'pattern'} className={`rounded-lg object-cover w-full h-full bg-blue-gray-50 ${bgColor}`} />
              }
            </div>
            <div className='absolute before:content-[""] bottom-0 bg-black w-full opacity-80 px-2 py-1 font-quicksand md:text-sm text-xs'>
              {name}
            </div>
            
            <div className='absolute before:content-[""] top-0 bg-black w-18 opacity-80 px-2 py-1 font-quicksand text-right right-0 rounded-bl-xl'>
              <div className='text-center text-xs'>
                {date}
              </div>
            </div>
          </a>
      </div>
  </>;
};

export default PostCard;
