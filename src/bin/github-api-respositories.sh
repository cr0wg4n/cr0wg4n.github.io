gh repo list --limit 250 --json url,name,description,updatedAt,stargazerCount,openGraphImageUrl,forkCount > repositories.json
mv repositories.json src/data/
