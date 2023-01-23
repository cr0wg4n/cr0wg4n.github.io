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
import Helmet from 'react-helmet';

function App() {
  const profile = useAppSelector(selectProfile);

  useEffect(()=>{
    ReactGa.send({ hitType: "pageview", page: "/" });
  },[])

  const description = `Hi, I'm Mauricio. Technology Building and designing is my main passion. Currently, I'm learning about systems programming and Rust for fun. I love and follow the Open Source and Free Sofware initiatives. I occasionally instruct programming with Python and I share my day-to-day tech experience through articles and talks.`

  return (
    <div className='bg-blue-gray-900 p-4 md:p-10 font-ubuntu text-blue-gray-50 relative h-full'>
      <Helmet>
          <html lang="en" />
          <title>Mauricio M. - Website</title>
          <meta name="description" content={description} />
          <meta name="theme-color" content="#ffffff" />

          <meta property='og:title' content='Mauricio M. - Website' />
          <meta property='og:url' content='https://cr0wg4n.github.io/' />
          <meta property='og:description' content={description} />
          <meta property="twitter:site" content="@cr0wg4n" />
          <meta property="twitter:card" content="summary_large_image" />
          <meta property='twitter:description' content={description} />
      </Helmet>

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
