import React from 'react';
import IconList from '../Icons/IconList';

type ProfileProps = {
    children?: React.ReactNode,
    name: string,
    description: React.ReactNode,
    role: string,
    avatar: string,
    avatarHover?: string
}

const ProfileCard: React.FC<ProfileProps>= ({avatar,avatarHover,children, name, description, role}:ProfileProps)=>{
    const [finalAvatar, setFinalAvatar] = React.useState(avatar);
    const alternativeAvatar = ()=>{
        avatarHover&&setFinalAvatar(avatarHover);
    };
    const normalAvatar = ()=>{
        avatar&&setFinalAvatar(avatar);
    };
    return <>
        <div className="grid grid-cols-12 gap-1 p-5 md:p-10">
            <div className='col-span-1 m-auto'>
                <IconList/>
            </div>
            <div className='m-auto col-span-5'>
                <img 
                    src={finalAvatar}
                    alt='Mauricio Matias, cr0wg4n' 
                    width={'240'} 
                    height={'240'}
                    className="rounded-full cursor-pointer"
                    onMouseEnter={alternativeAvatar}
                    onMouseLeave={normalAvatar}
                />
            </div>
            <div className='my-auto col-span-6'>
                <div className='font-normal text-normal text-left'>
                Who am I?
                </div>
                <div className='font-medium text-2xl md:text-2xl lg:text-4xl text-white'>
                {name}
                </div>
                <div className='font-normal text-xs md:text-lg text-left'>
                {role}
                </div>
                <br />
                <div className='font-normal text-xs md:text-sm text-center text-white font-sans'>
                    {description}
                </div>
            </div>
        </div>
    </>
};

export default ProfileCard; 
