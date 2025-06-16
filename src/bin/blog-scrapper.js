const Parser = require('rss-parser');
const fs = require('fs');
const { join } = require('path');
const fetchOpengraph = require('fetch-opengraph');

let parser = new Parser();

function parseDate (dateRaw) {
  return new Date(dateRaw).toDateString().slice(4)
}

function resizeIfMediumImage(image){
  return image.replace(/resize:fit:\d+/g, 'resize:fit:320').replace(/max\/\d+\//g, 'max/300/')
}

async function getUrlOpenGraph(url) {
  return await fetchOpengraph.fetch(url);
}

async function getRssData(url, path) {
  let feed = await parser.parseURL(url);
  const database = []
  for (const item of feed.items) {
    const data = await getUrlOpenGraph(item.link);
    const image = resizeIfMediumImage(data['og:image']);
    database.push({
      name: item.title,
      url: item.link,
      date: parseDate(item.isoDate),
      cover: image
    })
  }

  fs.writeFileSync(path, JSON.stringify(database,null,2))
}

async function getUrlData(urls, path) {
  const database = [];
  for (const url of urls) {
    const data = await getUrlOpenGraph(url);

    database.push({
      name: data['og:title'],
      cover: resizeIfMediumImage(data['og:image']),
      ...data['article:published_time'] && {
        date: parseDate(data['article:published_time']),
      },
      url,
    })
  }

  fs.writeFileSync(path,JSON.stringify(database,null,2))
}

getRssData('https://medium.com/feed/@cr0wg4n', join(__dirname,'../data/posts.json'));

getUrlData([
  'https://slides.com/cr0wg4n/los-enredos-de-la-web',
  'https://slides.com/cr0wg4n/la-educacion-cambio-mi-vida-dev-edition',
  'https://slides.com/cr0wg4n/escritura-tecnica-una-habilidad-invaluable',
  'https://slides.com/cr0wg4n/picosatelite-cansat-como-primer-acercamiento-a-tecnologias-espaciales-diseno-construccion-de-un-picosatelite-cansat-y-su-estacion-terrena',
  'https://slides.com/cr0wg4n/como-conseguir-tu-primera-dev-edition',  
  'https://slides.com/cr0wg4n/un-llamado-a-la-escritura-y-el-blogging',
  'https://slides.com/cr0wg4n/hardware-programming',
  'https://slides.com/cr0wg4n/privacidad-y-datos-personales',
  'https://slides.com/cr0wg4n/empaqueta-tus-ideas-con-pypi',
  'https://slides.com/cr0wg4n/taller-de-latex',
  'https://slides.com/cr0wg4n/la-tercera-es-la-vencida-como-llevar-a-cabo-un-scraping-exitoso',
  'https://slides.com/cr0wg4n/la-navaja-suizapython',
  'https://slides.com/cr0wg4n/pythoniza-tu-vida',
  'https://slides.com/cr0wg4n/web-scraping-scrapeo-hasta-el-suelo',
  'https://slides.com/cr0wg4n/herramientas-colaborativas-ia',
],join(__dirname,'../data/slides.json'));

getUrlData([
  'https://dev-academy.com/vue-design-patterns/',
  'https://dev-academy.com/csurf-vulnerability/',
  'https://dev-academy.com/react-localstorage/',
  'https://dev-academy.com/vue-router-best-practices/',
  'https://dev-academy.com/vue-security-best-practices/',
  'https://dev-academy.com/vue-xss/',

  'https://cr0wg4n.medium.com/latex-y-visual-studio-code-gu%C3%ADa-de-instalaci%C3%B3n-ca8bef3935e3',
  'https://cr0wg4n.medium.com/como-corregir-un-heap-out-of-memory-en-node-js-5e370fd1897',
  'https://cr0wg4n.medium.com/antena-yagi-de-433mhz-diy-bd568013c3fc',
  'https://cr0wg4n.medium.com/convirtiendo-un-teclado-musical-barato-en-uno-midi-d0c9893390b8',
  'https://cr0wg4n.medium.com/controlando-tiras-led-con-fastled-en-nodemcu-esp-8266-f20445735a22',
  'https://cr0wg4n.medium.com/gr%C3%A1fico-de-barras-con-unity-3d-b16b2e0a1625',

  'https://blog.ehcgroup.io/2022/03/28/15/54/05/12945/que-puede-decir-tu-navegador-sobre-ti/noticias-ehc/mmatias/',
  'https://blog.ehcgroup.io/2021/11/09/11/40/40/12083/los-5-arquetipos-del-desarrollador-de-software-ante-un-problema-con-cuales-te-identificas/noticias-ehc/mmatias/',
  'https://blog.ehcgroup.io/2021/06/23/09/22/11/11291/que-tan-seguro-es-pypi-python-package-index/hacking/mmatias/',
],join(__dirname,'../data/extra-posts.json'));

