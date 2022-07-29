import { createApp } from 'vue';
import App from './App.vue';
import store, { key } from './store';
import router from './router';
import disableDefaultEvents from '@/libs/disable-webview-default-events';

import './styles/global.scss';

disableDefaultEvents();
const app = createApp(App);
app.use(store, key);
app.use(router);
app.mount('#app');
