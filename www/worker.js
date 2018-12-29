import('./edge.js').then(handler => {
  console.log('handler', handler);
  self.addEventListener('message', handler);
});
