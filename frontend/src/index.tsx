/* @refresh reload */
import { render } from 'solid-js/web';

import initHttp from 'http-client';

import App from './App';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got mispelled?',
  );
}

initHttp('/node_modules/http-client/http_client_bg.wasm').then(() => render(() => <App />, root!));
