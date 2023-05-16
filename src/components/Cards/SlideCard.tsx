import React from "react";

export interface SlideCardProps {
    children?: React.ReactNode,
    name: string,
    url: string,
    cover?: string,
}

const SlideCard: React.FC<SlideCardProps> = ({
    name,
    url,
    cover
}: SlideCardProps) => {
    return <div className='mt-4 relative hover:cursor-pointer hover:border-b-2'>
        <a href={url} target={'_blank'} rel={"noreferrer"}>
            <img src={cover} width={'100%'} alt='ehc group main page' className='h-60 rounded-t-lg object-cover' />
            <div className='absolute before:content-[""] bottom-0 bg-black w-full opacity-80 px-2 py-1 font-quicksand md:text-sm text-xs'>
                {name}
            </div>
        </a>
    </div>
}

export default SlideCard;