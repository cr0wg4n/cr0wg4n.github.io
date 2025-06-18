import './App.css';

import Control from './components/Boxes/Control';
import PostList from './components/Posts/PostsList';
import ProfileCard from './components/Cards/ProfileCard';
import ProjectsList from './components/Projects/ProjectsList';
import ReactGa from 'react-ga4';
import Repositories from './components/Repositories/Index';
import SimpleBox from './components/Boxes/SimpleBox';
import SlidesList from './components/Slides/SlidesList';
import Terminal from './components/Terminal/Index';
import { selectProfile } from './store/profile';
import { useAppSelector } from './app/hooks';
import { useEffect } from 'react';

function App() {
  const profile = useAppSelector(selectProfile);

  useEffect(()=>{
    ReactGa.send({ hitType: "pageview", page: "/" });
  },[])

  return (
    <div className='bg-blue-gray-900 p-2 md:p-10 font-ubuntu text-blue-gray-50 relative h-full xl:px-[10%]'>
      <Control />

      <SimpleBox id={'profile'} firstColorClass='from-deep-orange-900' secondColorClass='to-deep-orange-500' orientation={false}>
        <ProfileCard 
            name={profile.name}
            role={profile.role}
            avatar={profile.avatar}
            avatarHover={profile.avatarHover}
            description={profile.description} 
        />
      </SimpleBox>

      <SimpleBox id='projects' firstColorClass="from-gray-900" secondColorClass={'to-gray-800'} title='Projects 🏗️'>
        <ProjectsList />
      </SimpleBox>

      <SimpleBox id='top-repos' firstColorClass="from-deep-purple-900" secondColorClass={'to-deep-purple-500'} title='Top Public Repos ⭐' orientation={false}>
        <Repositories />
      </SimpleBox>

      <SimpleBox id='slides' firstColorClass="from-teal-900" secondColorClass="to-teal-500" title='Slides / Talks & Workshops ▶️'>
        <SlidesList />
      </SimpleBox>

      <SimpleBox id='articles' firstColorClass="from-indigo-900" secondColorClass={'to-indigo-500'} title='Articles 📰' orientation={false}>
        <div className='mt-5' />
        <PostList />
      </SimpleBox>

      <Terminal id='terminal' />
    </div>
  );
}

export default App;
