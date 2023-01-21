function classNames(...classes: (false | null | undefined | string)[]) {
  return classes.filter(Boolean).join(" ");
}

function sluglify(words: string){
  return words.toLowerCase().split(' ').join('-');
}

function orderByDatetime(array: any[], field: string){
  array.sort((a: any, b: any): any => {
    return (new Date(b[field]).getTime() - new Date(a[field]).getTime())
  })
  return array
}

export {
  classNames,
  sluglify,
  orderByDatetime
}