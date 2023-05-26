import type { Component } from 'solid-js';

import init, {greet} from 'http-client';

init('/node_modules/http-client/http_client_bg.wasm');

const App: Component = () => {
  return (
    <div>
      <button onClick={() => greet()}>GREET</button>
    </div>
  );
};

export default App;
