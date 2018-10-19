import("./pkg")
  .then(module => {
    module.greet();
  })
  .catch(e => console.error("Error importing `index.js`:", e));
