import './App.css';
import ProfileCard from './components/Cards/ProfileCard';
import SimpleBox from './components/Boxes/SimpleBox';
import PostCard from './components/Cards/PostCard';
import MainGame from './components/Game/Index';

function App() {
  return (
    <div className='bg-blue-gray-900 p-4 md:p-10 font-ubuntu text-blue-gray-50'>

      <SimpleBox firstColorClass='from-deep-orange-900' secondColorClass='to-deep-orange-500'>
        <ProfileCard 
            name={'Mauricio Matias C.'}
            avatar={'/images/mauri.jpeg'}
            avatarHover={'/images/cr0wg4n.png'}
            description={<div className='font-quicksand'>
                Hi, I'm Mauricio. Technology Building and designing is my main passion. 
                Currently, I'm learning about systems programming and
                <span className='text-light-blue-200'>
                  &nbsp;
                  <a href="https://www.rust-lang.org/" rel="nofollow noopener noreferrer" target={'_blank'}>@rust</a> 
                  &nbsp;
                </span>
                for fun. I love and follow 
                the Open Source and Free Sofware initiatives. I occasionally instruct 
                programming and share my day-to-day tech experience as Software Developer through talks.
            </div>} 
            role={'Software Developer'}
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
        <div className='grid grid-cols-12 gap-3 md:gap-5'>
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
        </div>
      </SimpleBox>

      <MainGame />
    </div>
  );
}

export default App;
