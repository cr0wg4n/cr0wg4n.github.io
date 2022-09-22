import './App.css';
import ProfileCard from './components/Cards/ProfileCard';
import SimpleBox from './components/Boxes/SimpleBox';
import PostCard from './components/Cards/PostCard';
import MainGame from './components/Game/Index';
import PostList from './components/Posts/PostsList';
import { useAppSelector } from './app/hooks';
import { selectProfile } from './store/profile';

function App() {
  const profile = useAppSelector(selectProfile);

  return (
    <div className='bg-blue-gray-900 p-4 md:p-10 font-ubuntu text-blue-gray-50'>

      <SimpleBox firstColorClass='from-deep-orange-900' secondColorClass='to-deep-orange-500'>
        <ProfileCard 
            name={profile.name}
            role={profile.role}
            avatar={profile.avatar}
            avatarHover={profile.avatarHover}
            description={profile.description} 
        />
      </SimpleBox>

      <SimpleBox firstColorClass="from-teal-900" secondColorClass="to-teal-500" title='Side Projects' orientation>
        <div className='grid grid-cols-12 gap-5'>
          <div className=' col-span-4 mt-4'>
            <img src='/portfolio/ehc-mainpage.png' width={'100%'} alt='ehc group main page' className='rounded-lg'/>
          </div>
          
          <div className=' col-span-4 mt-4'>
            <img src='/portfolio/ehc-mainpage.png' width={'100%'} alt='ehc group main page' className='rounded-lg'/>
          </div>
          
          <div className=' col-span-4 mt-4'>
            <img src='/portfolio/ehc-mainpage.png' width={'100%'} alt='ehc group main page' className='rounded-lg'/>
          </div>
        </div>
      </SimpleBox>

      <SimpleBox firstColorClass="from-deep-purple-900" secondColorClass={'to-deep-purple-500'} title='My Blog Posts'>
        <div className='mt-5' />
        <PostList/>
        {/* <div className='grid grid-cols-12 gap-3 md:gap-5'>
          <PostCard 
            name='Palabras Vacías en Español (stop words) ft Python'
            url='https://cr0wg4n.medium.com/palabras-vac%C3%ADas-en-espa%C3%B1ol-stop-words-ft-python-3117e52d2bff'
            cover='/portfolio/post_1.png'
            date='07/03/22'
          />
          <PostCard 
            name='Palabras Vacías en Español (stop words) ft Python'
            url='https://cr0wg4n.medium.com/palabras-vac%C3%ADas-en-espa%C3%B1ol-stop-words-ft-python-3117e52d2bff'
            cover='/portfolio/post_1.png'
            date='07/03/22'
          />
          <PostCard 
            name='Palabras Vacías en Español (stop words) ft Python' 
            url='https://cr0wg4n.medium.com/palabras-vac%C3%ADas-en-espa%C3%B1ol-stop-words-ft-python-3117e52d2bff'
            date='10/02/21'
          />
          <PostCard 
            name='Palabras Vacías en Español (stop words) ft Python'
            url='https://cr0wg4n.medium.com/palabras-vac%C3%ADas-en-espa%C3%B1ol-stop-words-ft-python-3117e52d2bff'
            cover='/portfolio/post_1.png'
            date='07/03/22'
          />
          <PostCard 
            name='Palabras Vacías en Español (stop words) ft Python'
            url='https://cr0wg4n.medium.com/palabras-vac%C3%ADas-en-espa%C3%B1ol-stop-words-ft-python-3117e52d2bff'
            cover='/portfolio/post_1.png'
            date='07/03/22'
          />
        </div> */}
      </SimpleBox>

      <MainGame />
    </div>
  );
}

export default App;