import './App.css';
import ProfileCard from './components/Cards/ProfileCard';
import SimpleBox from './components/Boxes/SimpleBox';
import MainGame from './components/Game/Index';
import PostList from './components/Posts/PostsList';
import { useAppSelector } from './app/hooks';
import { selectProfile } from './store/profile';
import SlidesList from './components/Slides/SlidesList';
import ReactGa from 'react-ga4';
import { useEffect } from 'react';
import Clothes from './components/Clothes/Clothes';
import Control from './components/Boxes/Control';

function App() {
  const profile = useAppSelector(selectProfile);

  useEffect(()=>{
    ReactGa.send({ hitType: "pageview", page: "/" });
  },[])

  return (
    <div className='bg-blue-gray-900 p-4 md:p-10 font-ubuntu text-blue-gray-50 relative h-full'>

      <Control />

      <SimpleBox id={'profile'} firstColorClass='from-deep-orange-900' secondColorClass='to-deep-orange-500'>
        <ProfileCard 
            name={profile.name}
            role={profile.role}
            avatar={profile.avatar}
            avatarHover={profile.avatarHover}
            description={profile.description} 
        />
      </SimpleBox>

      <SimpleBox id='top-repos' firstColorClass="from-deep-purple-900" secondColorClass={'to-deep-purple-500'} title='Top Public Repos ⭐' orientation>
        <Clothes />
      </SimpleBox>

      <SimpleBox id='slides' firstColorClass="from-teal-900" secondColorClass="to-teal-500" title='Slides ▶️' orientation={false}>
        <SlidesList />
      </SimpleBox>

      <SimpleBox id='articles' firstColorClass="from-indigo-900" secondColorClass={'to-indigo-500'} title='Articles 📰' orientation>
        <div className='mt-5' />
        <PostList />
      </SimpleBox>

      <MainGame id='terminal' />
    </div>
  );
}

export default App;
