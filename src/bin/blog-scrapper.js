const Parser = require('rss-parser');
const fs = require('fs');
const { join } = require('path');
const fetchOpengraph = require('fetch-opengraph');

let parser = new Parser();

function parseDate (dateRaw) {
  return new Date(dateRaw).toDateString().slice(4)
}

async function getUrlOpenGraph(url) {
  return await fetchOpengraph.fetch(url);
}

async function getRssData(url, path) {
  let feed = await parser.parseURL(url);
  const database = []
  for (const item of feed.items) {
    const data = await getUrlOpenGraph(item.link);
    const image = data['og:image'].replace(/max\/\d+\//g, 'max/300/');
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
    console.log(data)
    database.push({
      name: data['og:title'],
      cover: data['og:image'],
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
  'https://slides.com/cr0wg4n/privacidad-y-datos-personales',
  'https://slides.com/cr0wg4n/empaqueta-tus-ideas-con-pypi',
  'https://slides.com/cr0wg4n/taller-de-latex',
  'https://slides.com/cr0wg4n/la-tercera-es-la-vencida-como-llevar-a-cabo-un-scraping-exitoso',
  'https://slides.com/cr0wg4n/la-navaja-suizapython'
],join(__dirname,'../data/slides.json'));

getUrlData([
  'https://dev-academy.com/csurf-vulnerability/',
  'https://dev-academy.com/react-localstorage/',
  'https://dev-academy.com/vue-router-best-practices/',
  'https://dev-academy.com/vue-security-best-practices/',
  'https://dev-academy.com/vue-xss/',
],join(__dirname,'../data/extra-posts.json'));

