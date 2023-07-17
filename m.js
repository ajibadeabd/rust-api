function ArrayChallenge(strArr) {
  return strArr.reduce((store,eachString,i)=>{
    let database  = store[0]
    let hashMap = store[1];
  if(hashMap[eachString]){
    database.splice(database.indexOf(eachString))
  
  }else if(5==database.length){
    delete hashMap[database.pop()]
  }
  database.unshift(eachString)
  hashMap[eachString] = true
  return store
  },[[],{}])[0].join("-")
  
  }
  
  console.log(
  ArrayChallenge(["A","B","A","C","A","B"])
  ,
  ArrayChallenge( ["A", "B", "C", "D", "E", "D", "Q", "Z", "C"])
  )

