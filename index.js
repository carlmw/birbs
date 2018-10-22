const main = async () => {
  try {
    const { BoidManager } = await import('./pkg');
    const { memory } = await import('./pkg/birbs_bg');
    const canvas = document.querySelector('canvas');
    const ctx = canvas.getContext('2d');
    const boids = new BoidManager(5000);
    const length = boids.length();
    const loop = () => {
      boids.flock(ctx);
      const all = new Float32Array(memory.buffer, boids.items(), length);

      ctx.clearRect(0, 0, 1910, 1080);
      ctx.fillStyle = 'red';

      for (let i = 0; i < length; i += 4) {
        ctx.fillRect(all[i], all[i + 1], 2, 2);
      }

      window.requestAnimationFrame(loop);
    };

    loop();
  } catch (error) {
    console.error("Error importing `index.js`:", error);
  }
};

main();
