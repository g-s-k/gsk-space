import loadWasm from './lib.rs';

loadWasm().then(result => {
  const addOne = result.instance.exports["add_one"];
  console.log(addOne(2));
});
