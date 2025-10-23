window.bittyFunctions = {
  randomFloat: function (min, max) {
    return (Math.random() * (max - min)) + min;
  },
  randomInt: function (min, max) {
    return Math.floor(Math.random() * (max - min + 1)) + min;
  },
  sleep: function (ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  },
  setProp: function (key, value) {
    document.documentElement.style.setProperty(key, value);
  },
  setPx: function (key, value) {
    document.documentElement.style.setProperty(key, `${value}px`);
  },
  shuffleArray: function (array) {
    let currentIndex = array.length;
    let randomIndex;
    while (currentIndex != 0) {
      randomIndex = Math.floor(Math.random() * currentIndex);
      currentIndex--;
      [array[currentIndex], array[randomIndex]] = [
        array[randomIndex],
        array[currentIndex],
      ];
    }
  },
};
