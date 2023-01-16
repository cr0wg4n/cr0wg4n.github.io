function classNames(...classes: (false | null | undefined | string)[]) {
  return classes.filter(Boolean).join(" ");
}

function sluglify(words: string){
  return words.toLowerCase().split(' ').join('-');
}

export {
  classNames,
  sluglify
}