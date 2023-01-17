import React from 'react';
import IconList from '../Icons/IconList';

type ProfileProps = {
    children?: React.ReactNode,
    name: string,
    description: string,
    role: string,
    avatar: string,
    avatarHover?: string
}

const ProfileCard: React.FC<ProfileProps> = ({avatar,avatarHover,children, name, description, role}:ProfileProps)=>{
    const [finalAvatar, setFinalAvatar] = React.useState(avatar);
    const alternativeAvatar = ()=>{
        avatarHover&&setFinalAvatar(avatarHover);
    };
    const normalAvatar = ()=>{
        avatar&&setFinalAvatar(avatar);
    };
    return <>
        <div
            className="grid grid-cols-12 gap-1 md:p-10"
            onMouseEnter={alternativeAvatar}
            onMouseLeave={normalAvatar}
        >
            <div className='col-span-1 m-auto'>
                <IconList/>
            </div>
            <div className='col-span-4 m-auto'>
                <img 
                    src={finalAvatar}
                    alt='Mauricio Matias, cr0wg4n' 
                    className="rounded-full cursor-pointer w-32 md:w-64"
                />
            </div>
            <div className='col-span-7 my-auto'>
                <div className='font-normal text-sm md:text-base text-left font-quicksand text-white'>
                    Who am I?
                </div>
                <div className='font-medium text-xl md:text-2xl lg:text-4xl text-white text-center'>
                    {name}
                </div>
                <div className='font-normal text-xs md:text-lg text-center font-quicksand'>
                    {role}
                </div>
                <div 
                    className='font-quicksand text-xs lg:text-base sm:text-xs md:text-sm text-center text-white mt-2 md:mt-7'
                    dangerouslySetInnerHTML={{__html:description}}
                />
            </div>
        </div>
    </>
};

export default ProfileCard; 
