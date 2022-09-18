export interface PostCard {
  name: string,
  url: string,
  cover?: string,
  date?:string
};

const posts: PostCard[] = [
  {
    name: 'Palabras Vacías en Español (stop words) ft Python',
    url: 'https://cr0wg4n.medium.com/palabras-vac%C3%ADas-en-espa%C3%B1ol-stop-words-ft-python-3117e52d2bff',
    cover: '/portfolio/post_1.png',
    date: '07/03/22'
  },
  {
    name: 'Palabras Vacías en Español (stop words) ft Python',
    url: 'https://cr0wg4n.medium.com/palabras-vac%C3%ADas-en-espa%C3%B1ol-stop-words-ft-python-3117e52d2bff',
    cover: '/portfolio/post_1.png',
    date: '07/03/22'
  },
  {
    name: 'Palabras Vacías en Español (stop words) ft Python',
    url: 'https://cr0wg4n.medium.com/palabras-vac%C3%ADas-en-espa%C3%B1ol-stop-words-ft-python-3117e52d2bff',
    // date: '07/03/22'
  },
  {
    name: 'Palabras Vacías en Español (stop words) ft Python',
    url: 'https://cr0wg4n.medium.com/palabras-vac%C3%ADas-en-espa%C3%B1ol-stop-words-ft-python-3117e52d2bff',
    cover: '/portfolio/post_1.png',
    date: '07/03/22'
  },
];

export default function data(){
  return posts;
};
