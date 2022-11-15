const Parser = require('rss-parser');
const fs = require('fs');
const { join } = require('path');
const fetchOpengraph = require('fetch-opengraph');


let parser = new Parser();

(async () => {
    const URL = 'https://medium.com/feed/@cr0wg4n' 
    let feed = await parser.parseURL(URL);
    const database = []
    for(const item of feed.items){
      const data = await fetchOpengraph.fetch(item.link);
      const image = data['og:image'].replace('max/1200', 'max/400');

      database.push({
        name: item.title,
        url: item.link,
        date: new Date(item.isoDate).toDateString().slice(4),
        cover: image
      })
    }
    fs.writeFileSync(join(__dirname,'../data/posts.json'),JSON.stringify(database,null,2))
})();