import slideData from './slides.json'

export interface SlideCard {
  name: string,
  url: string,
  cover?: string,
};

const slides: SlideCard[] = [
  ...slideData,
];

export default function data(){
  return slides;
};
